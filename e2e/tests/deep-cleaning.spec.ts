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
    const secondTaskName = await deepCleaningItems.nth(1).locator('.task-name').textContent();
    
    expect(firstTaskName).toBeTruthy();
    expect(secondTaskName).toBeTruthy();
    expect(firstTaskName).not.toBe(secondTaskName);
    
    const firstCheckbox = deepCleaningItems.first().locator('.task-checkbox');
    await firstCheckbox.click();
    await page.waitForTimeout(500);
    
    const newFirstTaskName = await deepCleaningItems.first().locator('.task-name').textContent();
    expect(newFirstTaskName).toBe(secondTaskName);
  });

  test('completed task moves to end of queue', async ({ page }) => {
    const deepCleaningList = page.locator('#deep-cleaning-list');
    const deepCleaningItems = deepCleaningList.locator('.task-item');
    const initialCount = await deepCleaningItems.count();
    
    const firstTaskName = await deepCleaningItems.first().locator('.task-name').textContent();
    
    const firstCheckbox = deepCleaningItems.first().locator('.task-checkbox');
    await firstCheckbox.click();
    await page.waitForTimeout(500);
    
    const newCount = await deepCleaningItems.count();
    expect(newCount).toBe(initialCount);
    
    const lastTaskName = await deepCleaningItems.nth(initialCount - 1).locator('.task-name').textContent();
    expect(lastTaskName).toBe(firstTaskName);
  });

  test('queue persists after page reload', async ({ page }) => {
    const deepCleaningList = page.locator('#deep-cleaning-list');
    const deepCleaningItems = deepCleaningList.locator('.task-item');
    
    const firstTaskName = await deepCleaningItems.first().locator('.task-name').textContent();
    
    const firstCheckbox = deepCleaningItems.first().locator('.task-checkbox');
    await firstCheckbox.click();
    await page.waitForTimeout(500);
    
    await page.reload();
    await page.waitForSelector('#deep-cleaning-list', { state: 'visible', timeout: 5000 });
    
    const reloadedList = page.locator('#deep-cleaning-list');
    const reloadedItems = reloadedList.locator('.task-item');
    const reloadedFirstTaskName = await reloadedItems.first().locator('.task-name').textContent();
    
    expect(reloadedFirstTaskName).not.toBe(firstTaskName);
  });
});
