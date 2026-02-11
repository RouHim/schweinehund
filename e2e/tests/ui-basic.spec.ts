import { test, expect } from './fixtures';

test.describe('Schweinehund UI Basic Tests', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('#tasks-list', { state: 'visible', timeout: 5000 });
    const response = await page.request.post('/api/debug/reset-all');
    expect(response.ok()).toBeTruthy();
    await page.reload();
    await page.waitForSelector('#tasks-list', { state: 'visible', timeout: 5000 });
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
    await expect(todayHeading).toContainText("Heutige Aufgaben");
    
    await page.waitForSelector('#tasks-list', { state: 'visible', timeout: 5000 });
    
    const tasksList = page.locator('#tasks-list');
    await expect(tasksList).toBeVisible();
  });

  test('checkbox interaction works', async ({ page }) => {
    // Wait for tasks to load
    await page.waitForSelector('#tasks-list .task-checkbox', { state: 'visible', timeout: 5000 });
    
    // Use a known seed task name to locate the checkbox
    const taskItem = page.locator('.task-item', { has: page.locator('text=Spuelmaschine an/aus') }).first();
    await expect(taskItem).toBeVisible();
    
    const checkbox = taskItem.locator('.task-checkbox');
    const isInitiallyChecked = await checkbox.isChecked();
    
    // Set up response listener before clicking
    const responsePromise = page.waitForResponse(
      response =>
        response.url().includes('/api/tasks/') &&
        response.url().includes('/toggle') &&
        response.request().method() === 'POST',
    );
    
    await checkbox.click();
    await responsePromise;
    
    // Re-locate the element after toggle (DOM was replaced by renderTasks)
    const updatedTaskItem = page.locator('.task-item', { has: page.locator('text=Spuelmaschine an/aus') }).first();
    const updatedCheckbox = updatedTaskItem.locator('.task-checkbox');
    const isNowChecked = await updatedCheckbox.isChecked();
    
    expect(isNowChecked).toBe(!isInitiallyChecked);
    
    if (isNowChecked) {
      await expect(updatedTaskItem).toHaveClass(/completed/);
    }
  });

  test('deep cleaning section visible', async ({ page }) => {
    const deepCleaningSection = page.locator('#deep-cleaning-section');
    await expect(deepCleaningSection).toBeVisible();
    
    const deepCleaningHeading = deepCleaningSection.locator('h2');
    await expect(deepCleaningHeading).toContainText('Grundreinigung');
  });

  test('settings section visible', async ({ page }) => {
    const settingsSection = page.locator('#settings-section');
    await expect(settingsSection).toBeVisible();
    
    const settingsHeading = settingsSection.locator('h2');
    await expect(settingsHeading).toContainText('Einstellungen');
    
    await page.waitForSelector('#settings-form', { state: 'visible', timeout: 5000 });
    
    const settingsForm = page.locator('#settings-form');
    await expect(settingsForm).toBeVisible();
  });

  test('theme toggle button works', async ({ page }) => {
    const themeToggle = page.locator('#theme-toggle');
    await expect(themeToggle).toBeVisible();
    
    const htmlElement = page.locator('html');
    const initialTheme = await htmlElement.getAttribute('data-theme');
    
    const expectedTheme = initialTheme === 'dark' ? 'light' : 'dark';
    
    await themeToggle.click();
    await expect(htmlElement).toHaveAttribute('data-theme', expectedTheme);
  });
});
