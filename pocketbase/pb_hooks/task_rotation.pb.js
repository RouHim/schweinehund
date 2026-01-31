/// <reference path="../pb_data/types.d.ts" />

/**
 * Task Rotation Logic
 * Automatically rotates large/weekly tasks when completed.
 * - Triggers on task completion (is_daily=false)
 * - Moves completed task to end of rotation queue
 * - Updates sort_order to max + 1
 */

/**
 * Rotate task to end of queue
 * @param {models.Record} task - The task record to rotate
 */
function rotateTaskToEnd(task) {
  try {
    const taskId = task.getString('id');
    console.log(`Rotating task ${taskId} to end of queue...`);
    
    // Find all rotating (non-daily) tasks
    const allRotating = $app.findRecordsByFilter('tasks', 'is_daily=false');
    
    if (!allRotating || allRotating.length === 0) {
      console.warn('No rotating tasks found');
      return;
    }
    
    // Calculate max sort_order
    const maxOrder = Math.max(...allRotating.map(t => t.getInt('sort_order')));
    const currentOrder = task.getInt('sort_order');
    
    // Prevent unnecessary updates if already at end
    if (currentOrder >= maxOrder) {
      console.log(`Task ${taskId} already at end (order: ${currentOrder})`);
      return;
    }
    
    // Move to end of queue
    const newOrder = maxOrder + 1;
    task.set('sort_order', newOrder);
    $app.save(task);
    
    console.log(`Task ${taskId} rotated: ${currentOrder} -> ${newOrder}`);
    
  } catch (error) {
    console.error(`Error rotating task: ${error}`);
    // Don't throw - allow update to complete
  }
}

/**
 * Hook: After task update
 * Triggers rotation when weekly task is marked completed
 */
onRecordAfterUpdateSuccess((e) => {
  const task = e.record;
  
  // Only process non-daily (rotating) tasks
  const isDaily = task.getBool('is_daily');
  if (isDaily) {
    return;
  }
  
  // Only rotate when task becomes completed
  const completed = task.getBool('completed');
  if (!completed) {
    return;
  }
  
  // Check if this is a state change (was uncompleted, now completed)
  // Prevent rotation on every update to an already-completed task
  const oldRecord = e.record.originalCopy();
  const wasCompleted = oldRecord.getBool('completed');
  
  if (wasCompleted) {
    // Already was completed, don't rotate again
    return;
  }
  
  // Rotate to end
  rotateTaskToEnd(task);
  
}, 'tasks');

console.log('Task rotation hook loaded');
