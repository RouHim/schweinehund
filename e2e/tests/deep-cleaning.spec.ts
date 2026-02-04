import { test, expect } from '@playwright/test';

test.describe('Deep Cleaning Queue', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('#deep-cleaning-list', { state: 'visible', timeout: 5000 });
  });

  test('displays deep cleaning tasks', async ({ page }) => {
    const deepCleaningList = page.locator('#deep-cleaning-list');
    await expect(deepCleaningList).toBeVisible();
    
    const deepCleaningItems = deepCleaningList.locator('.task-item');
    const count = await deepCleaningItems.count();
    expect(count).toBeGreaterThan(0);
    
    const firstItem = deepCleaningItems.first();
    await expect(firstItem.locator('.task-name')).toBeVisible();
    await expect(firstItem.locator('.task-checkbox')).toBeVisible();
  });

  test('completes deep cleaning task and rotates queue', async ({ page }) => {
    const deepCleaningList = page.locator('#deep-cleaning-list');
    const deepCleaningItems = deepCleaningList.locator('.task-item');
    
    const firstTaskName = await deepCleaningItems.first().locator('.task-name').textContent();
    const initialSecondTaskName = await deepCleaningItems.nth(1).locator('.task-name').textContent();
    
    expect(firstTaskName).toBeTruthy();
    expect(initialSecondTaskName).toBeTruthy();
    expect(firstTaskName).not.toBe(initialSecondTaskName);
    
    const firstCheckbox = deepCleaningItems.first().locator('.task-checkbox');
    await firstCheckbox.click();
    
    // Wait for the list to update after completion
    await page.waitForSelector('#deep-cleaning-list .task-item', { state: 'attached' });
    await page.waitForTimeout(300);
    
    const updatedItems = page.locator('#deep-cleaning-list .task-item');
    const newFirstTaskName = await updatedItems.first().locator('.task-name').textContent();
    expect(newFirstTaskName).toBe(initialSecondTaskName);
  });

  test('completed task moves to end of queue', async ({ page }) => {
    const deepCleaningList = page.locator('#deep-cleaning-list');
    const deepCleaningItems = deepCleaningList.locator('.task-item');
    const initialCount = await deepCleaningItems.count();
    
    const firstTaskName = await deepCleaningItems.first().locator('.task-name').textContent();
    
    const firstCheckbox = deepCleaningItems.first().locator('.task-checkbox');
    await firstCheckbox.click();
    
    // Wait for the list to update after completion
    await page.waitForSelector('#deep-cleaning-list .task-item', { state: 'attached' });
    await page.waitForTimeout(300);
    
    const updatedItems = page.locator('#deep-cleaning-list .task-item');
    const finalCount = await updatedItems.count();
    expect(finalCount).toBe(initialCount);
    
    const tasks = await updatedItems.all();
    const lastTaskName = await tasks[tasks.length - 1].locator('.task-name').textContent();
    expect(lastTaskName).toBe(firstTaskName);
  });

  test('queue state persists after page reload', async ({ page }) => {
    const deepCleaningList = page.locator('#deep-cleaning-list');
    const deepCleaningItems = deepCleaningList.locator('.task-item');
    
    const beforeReloadState = await deepCleaningItems.all().then(async items => {
      return Promise.all(items.map(item => item.locator('.task-name').textContent()));
    });
    
    await page.reload();
    await page.waitForSelector('#deep-cleaning-list', { state: 'visible', timeout: 5000 });
    
    const reloadedItems = page.locator('#deep-cleaning-list .task-item');
    const afterReloadState = await reloadedItems.all().then(async items => {
      return Promise.all(items.map(item => item.locator('.task-name').textContent()));
    });
    
    expect(afterReloadState).toEqual(beforeReloadState);
  });
});
