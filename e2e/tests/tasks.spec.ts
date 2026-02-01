import { test, expect } from '@playwright/test';

test.describe('Task Management', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('#tasks-list', { state: 'visible', timeout: 5000 });
  });

  test('displays daily tasks on page load', async ({ page }) => {
    const tasksList = page.locator('#tasks-list');
    await expect(tasksList).toBeVisible();
    
    const taskItems = tasksList.locator('.task-item');
    const count = await taskItems.count();
    expect(count).toBeGreaterThan(0);
    
    const firstTask = taskItems.first();
    await expect(firstTask.locator('.task-checkbox')).toBeVisible();
    await expect(firstTask.locator('.task-name')).toBeVisible();
  });

  test('toggles task completion state', async ({ page }) => {
    const tasksList = page.locator('#tasks-list');
    const firstCheckbox = tasksList.locator('.task-checkbox').first();
    const firstTaskItem = tasksList.locator('.task-item').first();
    
    const wasChecked = await firstCheckbox.isChecked();
    
    await firstCheckbox.click();
    await page.waitForTimeout(500);
    
    const isNowChecked = await firstCheckbox.isChecked();
    expect(isNowChecked).toBe(!wasChecked);
    
    if (isNowChecked) {
      await expect(firstTaskItem).toHaveClass(/completed/);
    } else {
      const classes = await firstTaskItem.getAttribute('class');
      expect(classes).not.toContain('completed');
    }
  });

  test('persists task completion across page reload', async ({ page }) => {
    const tasksList = page.locator('#tasks-list');
    const firstCheckbox = tasksList.locator('.task-checkbox').first();
    
    const initialState = await firstCheckbox.isChecked();
    
    await firstCheckbox.click();
    await page.waitForTimeout(1000);
    
    const newState = await firstCheckbox.isChecked();
    expect(newState).toBe(!initialState);
    
    await page.reload();
    await page.waitForSelector('#tasks-list', { state: 'visible', timeout: 5000 });
    
    const reloadedTasksList = page.locator('#tasks-list');
    const reloadedCheckbox = reloadedTasksList.locator('.task-checkbox').first();
    const reloadedState = await reloadedCheckbox.isChecked();
    
    expect(reloadedState).toBe(newState);
  });

  test('unchecks task and persists', async ({ page }) => {
    const tasksList = page.locator('#tasks-list');
    const secondCheckbox = tasksList.locator('.task-checkbox').nth(1);
    
    const initialState = await secondCheckbox.isChecked();
    
    await secondCheckbox.click();
    await page.waitForTimeout(1000);
    
    const newState = await secondCheckbox.isChecked();
    expect(newState).toBe(!initialState);
    
    await page.reload();
    await page.waitForSelector('#tasks-list', { state: 'visible', timeout: 5000 });
    
    const reloadedTasksList = page.locator('#tasks-list');
    const reloadedCheckbox = reloadedTasksList.locator('.task-checkbox').nth(1);
    const reloadedState = await reloadedCheckbox.isChecked();
    
    expect(reloadedState).toBe(newState);
  });
});
