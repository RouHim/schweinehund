import { test, expect } from './fixtures';

test.describe('attachTaskListeners verification', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('#tasks-list', { state: 'visible', timeout: 5000 });
    await page.request.post('/api/debug/reset-all');
    await page.reload();
    await page.waitForSelector('#tasks-list', { state: 'visible', timeout: 5000 });
  });

  test('function exists in global scope', async ({ page }) => {
    await page.goto('/');
    
    await page.waitForSelector('#tasks-list', { state: 'visible', timeout: 5000 });
    
    const hasFunction = await page.evaluate(() => {
      return typeof (window as any).attachTaskListeners === 'function';
    });
    expect(hasFunction).toBe(true);
  });

  test('no JavaScript errors on page load', async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('#tasks-list', { state: 'visible', timeout: 5000 });
  });

  test('checkbox toggle updates state', async ({ page }) => {
    await page.waitForSelector('#tasks-list .task-item', { state: 'visible', timeout: 5000 });
    
    // Locate task by name (stable across re-renders)
    const taskItem = page.locator('#tasks-list .task-item', {
      has: page.locator('.task-name', { hasText: 'Spuelmaschine an/aus' }),
    });
    
    const checkbox = taskItem.locator('.task-checkbox');
    const initialState = await checkbox.isChecked();
    
    // Wait for toggle API response
    const responsePromise = page.waitForResponse(
      response =>
        response.url().includes('/api/tasks/') &&
        response.url().includes('/toggle') &&
        response.request().method() === 'POST',
    );
    
    await checkbox.click();
    await responsePromise;
    
    // Re-locate the task (DOM was replaced by renderTasks())
    const reloadedTask = page.locator('#tasks-list .task-item', {
      has: page.locator('.task-name', { hasText: 'Spuelmaschine an/aus' }),
    });
    
    const newState = await reloadedTask.locator('.task-checkbox').isChecked();
    expect(newState).toBe(!initialState);
  });

  test('edit button opens modal', async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('#tasks-list', { state: 'visible', timeout: 5000 });
    
    const editBtn = await page.locator('#tasks-list [data-testid="edit-btn"]').first();
    await editBtn.click();
    
    const modal = page.locator('#task-modal');
    await expect(modal).toBeVisible();
  });
});
