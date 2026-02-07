import { test, expect } from '@playwright/test';

test.describe('Schweinehund UI Basic Tests', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
  });

  test('page loads with Schweinehund title', async ({ page }) => {
    await expect(page).toHaveTitle(/Schweinehund/);
    
    const heading = page.locator('h1');
    await expect(heading).toContainText('Schweinehund');
    
    const mascot = page.locator('.mascot');
    await expect(mascot).toBeVisible();
  });

  test('tasks visible on iPhone 13 viewport', async ({ page }) => {
    await page.setViewportSize({ width: 390, height: 844 });
    
    const todaySection = page.locator('#today-section');
    await expect(todaySection).toBeVisible();
    
    const todayHeading = todaySection.locator('h2');
    await expect(todayHeading).toContainText("Today's Tasks");
    
    await page.waitForSelector('#tasks-list', { state: 'visible', timeout: 5000 });
    
    const tasksList = page.locator('#tasks-list');
    await expect(tasksList).toBeVisible();
  });

  test('checkbox interaction works', async ({ page }) => {
    await page.waitForSelector('.task-checkbox', { state: 'visible', timeout: 5000 });
    
    const firstCheckbox = page.locator('.task-checkbox').first();
    await expect(firstCheckbox).toBeVisible();
    
    const isInitiallyChecked = await firstCheckbox.isChecked();
    
    await firstCheckbox.click();
    await page.waitForTimeout(500);
    
    const isNowChecked = await firstCheckbox.isChecked();
    expect(isNowChecked).toBe(!isInitiallyChecked);
    
    const taskItem = page.locator('.task-item').first();
    if (isNowChecked) {
      await expect(taskItem).toHaveClass(/completed/);
    }
  });

  test('deep cleaning section visible', async ({ page }) => {
    const deepCleaningSection = page.locator('#deep-cleaning-section');
    await expect(deepCleaningSection).toBeVisible();
    
    const deepCleaningHeading = deepCleaningSection.locator('h2');
    await expect(deepCleaningHeading).toContainText('Deep Cleaning Queue');
  });

  test('settings section visible', async ({ page }) => {
    const settingsSection = page.locator('#settings-section');
    await expect(settingsSection).toBeVisible();
    
    const settingsHeading = settingsSection.locator('h2');
    await expect(settingsHeading).toContainText('Settings');
    
    await page.waitForSelector('#settings-form', { state: 'visible', timeout: 5000 });
    
    const settingsForm = page.locator('#settings-form');
    await expect(settingsForm).toBeVisible();
  });

  test('theme toggle button works', async ({ page }) => {
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
