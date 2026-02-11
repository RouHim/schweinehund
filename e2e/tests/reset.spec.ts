import { test, expect } from './fixtures';

test.describe('Weekly Reset Functionality', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('#tasks-list', { state: 'visible', timeout: 5000 });
  });

  test('resets all daily tasks to unchecked', async ({ page, request }) => {
    const tasksList = page.locator('#tasks-list');
    const dailyCheckboxes = tasksList.locator('.task-checkbox');
    const initialCount = await dailyCheckboxes.count();
    expect(initialCount).toBeGreaterThan(0);
    
    for (let i = 0; i < Math.min(5, initialCount); i++) {
      const checkbox = tasksList.locator('.task-checkbox:not(:checked)').first();
      if (await checkbox.count() === 0) break;

      const responsePromise = page.waitForResponse(
        response =>
          response.url().includes('/api/tasks/') &&
          response.url().includes('/toggle') &&
          response.request().method() === 'POST',
      );
      await checkbox.click();
      await responsePromise;
    }
    
    const checkedCountBefore = await tasksList.locator('.task-checkbox:checked').count();
    expect(checkedCountBefore).toBeGreaterThan(0);
    
    const resetResponse = await request.post('/api/debug/reset');
    expect(resetResponse.status()).toBe(200);
    
    await page.reload();
    await page.waitForSelector('#tasks-list', { state: 'visible', timeout: 5000 });
    
    const reloadedTasksList = page.locator('#tasks-list');
    const checkedCountAfter = await reloadedTasksList.locator('.task-checkbox:checked').count();
    expect(checkedCountAfter).toBe(0);
  });

  test('reset endpoint returns success', async ({ request }) => {
    const response = await request.post('/api/debug/reset');
    expect(response.status()).toBe(200);
    
    const json = await response.json();
    expect(json).toHaveProperty('message');
    expect(json.message).toContain('reset');
  });

  test('tasks remain after reset', async ({ page, request }) => {
    const tasksList = page.locator('#tasks-list');
    const checkboxesBefore = tasksList.locator('.task-checkbox');
    const countBefore = await checkboxesBefore.count();
    
    await request.post('/api/debug/reset');
    
    await page.reload();
    await page.waitForSelector('#tasks-list', { state: 'visible', timeout: 5000 });
    
    const reloadedTasksList = page.locator('#tasks-list');
    const checkboxesAfter = reloadedTasksList.locator('.task-checkbox');
    const countAfter = await checkboxesAfter.count();
    
    expect(countAfter).toBe(countBefore);
    expect(countAfter).toBeGreaterThan(0);
  });
});
