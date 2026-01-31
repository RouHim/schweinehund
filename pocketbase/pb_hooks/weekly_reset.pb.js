/// <reference path="../pb_data/types.d.ts" />

/**
 * Weekly Reset Scheduler
 * Automatically resets daily tasks every Monday at 00:00
 * - Sets completed = false
 * - Clears completed_at timestamp
 * - Sends notification
 */

const NTFY_URL = 'http://ntfy:8091/schweinehund';

/**
 * Send notification to ntfy.sh
 * @param {string} message - Notification body
 * @param {string} title - Notification title (default: "Schweinehund")
 * @param {number} priority - Priority 1-5 (default: 3)
 */
function sendNotification(message, title = 'Schweinehund', priority = 3) {
  try {
    const response = $http.send({
      url: NTFY_URL,
      method: 'POST',
      body: message,
      headers: {
        'Title': title,
        'Priority': String(priority),
        'Tags': 'recycle'
      },
      timeout: 5 // 5 seconds timeout
    });
    
    if (response.statusCode >= 200 && response.statusCode < 300) {
      console.log(`Weekly reset notification sent: ${message}`);
    } else {
      console.error(`Failed to send reset notification. Status: ${response.statusCode}`);
    }
  } catch (error) {
    console.error(`Error sending reset notification: ${error}`);
    // Don't crash - just log the error
  }
}

/**
 * Reset all daily tasks
 * Sets completed=false and completed_at="" for is_daily=true tasks
 * @returns {number} Number of tasks reset
 */
function resetDailyTasks() {
  try {
    console.log('Starting weekly reset of daily tasks...');
    
    const tasks = $app.findRecordsByFilter('tasks', 'is_daily=true');
    
    if (!tasks || tasks.length === 0) {
      console.log('No daily tasks found to reset');
      return 0;
    }
    
    let resetCount = 0;
    
    tasks.forEach(task => {
      try {
        task.set('completed', false);
        task.set('completed_at', '');
        $app.save(task);
        resetCount++;
      } catch (error) {
        console.error(`Failed to reset task ${task.id}: ${error}`);
        // Continue processing other tasks
      }
    });
    
    console.log(`Weekly reset completed: ${resetCount} daily task(s) reset`);
    return resetCount;
    
  } catch (error) {
    console.error(`Error during weekly reset: ${error}`);
    return 0;
  }
}

// Bootstrap hook - runs once when PocketBase starts
onAfterBootstrap((e) => {
  console.log('Weekly reset scheduler initializing...');
  
  // Schedule weekly reset for Monday at 00:00
  const weeklyCron = new Cron();
  weeklyCron.mustAdd('weekly-reset', '0 0 * * 1', () => {
    const resetCount = resetDailyTasks();
    
    // Send notification after successful reset
    if (resetCount > 0) {
      sendNotification(
        'Neue Woche! Aufgaben zurückgesetzt 🔄',
        'Schweinehund',
        4 // Higher priority for weekly reset
      );
    }
  });
  
  console.log('Weekly reset scheduled for Monday at 00:00 (cron: 0 0 * * 1)');
});

console.log('Weekly reset hook loaded');
