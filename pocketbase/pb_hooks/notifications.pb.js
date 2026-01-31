/// <reference path="../pb_data/types.d.ts" />

/**
 * ntfy.sh push notification integration
 * Sends notifications for:
 * - Daily reminder at 09:00
 * - Zone reminder based on current weekday
 * - Reset completion notifications
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
        'Tags': 'house_with_garden'
      },
      timeout: 5 // 5 seconds timeout
    });
    
    if (response.statusCode >= 200 && response.statusCode < 300) {
      console.log(`Notification sent: ${message}`);
    } else {
      console.error(`Failed to send notification. Status: ${response.statusCode}`);
    }
  } catch (error) {
    console.error(`Error sending notification: ${error}`);
    // Don't crash - just log the error
  }
}

/**
 * Get zone name for current weekday
 * @returns {string|null} Zone name or null if not found
 */
function getCurrentZoneName() {
  try {
    const now = new Date();
    const weekday = now.getDay(); // 0 = Sunday, 1 = Monday, ..., 6 = Saturday
    
    const zones = $app.findRecordsByFilter(
      'zones',
      `weekday = ${weekday}`,
      '-created',
      1
    );
    
    if (zones && zones.length > 0) {
      const zone = zones[0];
      return zone.getString('emoji') ? 
        `${zone.getString('emoji')} ${zone.getString('name')}` :
        zone.getString('name');
    }
    
    return null;
  } catch (error) {
    console.error(`Error getting zone name: ${error}`);
    return null;
  }
}

/**
 * Send daily reminder notification
 */
function sendDailyReminder() {
  const zoneName = getCurrentZoneName();
  
  if (zoneName) {
    sendNotification(
      `Zeit für deine Aufgaben!\n\nHeute ist ${zoneName} dran 📋`,
      'Schweinehund',
      4 // Higher priority for daily reminder
    );
  } else {
    sendNotification(
      'Zeit für deine Aufgaben! 📋',
      'Schweinehund',
      4
    );
  }
}

/**
 * Send reset completion notification
 */
function sendResetNotification() {
  sendNotification(
    'Neue Woche! Aufgaben zurückgesetzt 🔄',
    'Schweinehund',
    3
  );
}

// Bootstrap hook - runs once when PocketBase starts
onAfterBootstrap((e) => {
  console.log('Notification system initialized');
  
  // Send test notification on startup
  sendNotification('Schweinehund ist bereit! 🐕', 'System', 2);
  
  // Schedule daily reminder at 09:00
  const dailyCron = new Cron();
  dailyCron.mustAdd('daily-reminder', '0 9 * * *', () => {
    sendDailyReminder();
  });
  
  console.log('Daily reminder scheduled for 09:00');
});

// Hook for settings updates (to detect weekly reset)
onRecordAfterUpdateSuccess((e) => {
  if (e.record.tableName() === 'settings') {
    const key = e.record.getString('key');
    
    // Detect weekly reset completion
    if (key === 'last_reset') {
      sendResetNotification();
    }
  }
}, 'settings');

console.log('Notification hooks loaded');
