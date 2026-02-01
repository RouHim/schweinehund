import { test, expect } from '@playwright/test';

test.describe('Mobile Viewport Interactions', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('#tasks-list', { state: 'visible', timeout: 5000 });
  });

  test('page renders correctly on mobile viewport', async ({ page }) => {
    const viewport = page.viewportSize();
    expect(viewport).not.toBeNull();
    
    const heading = page.locator('h1');
    await expect(heading).toBeVisible();
    
    const tasksList = page.locator('#tasks-list');
    await expect(tasksList).toBeVisible();
  });

  test('checkboxes are touch-friendly', async ({ page }) => {
    const checkboxes = page.locator('.task-checkbox');
    const count = await checkboxes.count();
    
    if (count > 0) {
      const firstCheckbox = checkboxes.first();
      const boundingBox = await firstCheckbox.boundingBox();
      
      expect(boundingBox).not.toBeNull();
      
      if (boundingBox) {
        expect(boundingBox.width).toBeGreaterThanOrEqual(24);
        expect(boundingBox.height).toBeGreaterThanOrEqual(24);
      }
    }
  });

  test('deep cleaning checkboxes are touch-friendly', async ({ page }) => {
    await page.waitForSelector('#deep-cleaning-list', { state: 'visible', timeout: 5000 });
    
    const deepCleaningList = page.locator('#deep-cleaning-list');
    const deepCheckboxes = deepCleaningList.locator('.task-checkbox');
    const count = await deepCheckboxes.count();
    
    if (count > 0) {
      const firstCheckbox = deepCheckboxes.first();
      const boundingBox = await firstCheckbox.boundingBox();
      
      expect(boundingBox).not.toBeNull();
      
      if (boundingBox) {
        expect(boundingBox.width).toBeGreaterThanOrEqual(24);
        expect(boundingBox.height).toBeGreaterThanOrEqual(24);
      }
    }
  });

  test('can scroll through all sections', async ({ page }) => {
    const todaySection = page.locator('#today-section');
    await expect(todaySection).toBeVisible();
    
    const deepCleaningSection = page.locator('#deep-cleaning-section');
    await deepCleaningSection.scrollIntoViewIfNeeded();
    await expect(deepCleaningSection).toBeVisible();
    
    const settingsSection = page.locator('#settings-section');
    await settingsSection.scrollIntoViewIfNeeded();
    await expect(settingsSection).toBeVisible();
    
    await todaySection.scrollIntoViewIfNeeded();
    await expect(todaySection).toBeVisible();
  });

  test('task interaction works on mobile', async ({ page }) => {
    const firstCheckbox = page.locator('.task-checkbox').first();
    const wasChecked = await firstCheckbox.isChecked();
    
    await firstCheckbox.click();
    await page.waitForTimeout(500);
    
    const isNowChecked = await firstCheckbox.isChecked();
    expect(isNowChecked).toBe(!wasChecked);
  });

  test('deep cleaning checkbox works on mobile', async ({ page }) => {
    await page.waitForSelector('#deep-cleaning-list', { state: 'visible', timeout: 5000 });
    
    const deepCleaningList = page.locator('#deep-cleaning-list');
    const deepCleaningItems = deepCleaningList.locator('.task-item');
    const count = await deepCleaningItems.count();
    
    if (count > 1) {
      const firstTaskName = await deepCleaningItems.first().locator('.task-name').textContent();
      
      const firstCheckbox = deepCleaningItems.first().locator('.task-checkbox');
      await firstCheckbox.click();
      await page.waitForTimeout(500);
      
      const newFirstTaskName = await deepCleaningItems.first().locator('.task-name').textContent();
      expect(newFirstTaskName).not.toBe(firstTaskName);
    }
  });

  test('theme toggle works on mobile', async ({ page }) => {
    const themeToggle = page.locator('#theme-toggle');
    await expect(themeToggle).toBeVisible();
    
    const htmlElement = page.locator('html');
    const initialTheme = await htmlElement.getAttribute('data-theme');
    
    await themeToggle.click();
    await page.waitForTimeout(300);
    
    const newTheme = await htmlElement.getAttribute('data-theme');
    expect(newTheme).not.toBe(initialTheme);
  });
});
