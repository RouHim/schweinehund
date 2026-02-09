import { test, expect, type Page } from '@playwright/test';

async function createDeepCleaningTask(page: Page, name: string) {
  await page.locator('[data-testid="add-deep-cleaning-btn"]').click();
  await expect(page.locator('#task-modal')).toBeVisible();
  await page.locator('[data-testid="task-name-input"]').fill(name);
  await page.locator('[data-testid="modal-save-btn"]').click();
  await expect(page.locator('#task-modal')).not.toBeVisible({ timeout: 10000 });
  await expect(page.locator('#deep-cleaning-list .task-name', { hasText: name })).toBeVisible();
}

async function deepTaskNames(page: Page): Promise<string[]> {
  const names = page.locator('#deep-cleaning-list .task-item .task-name');
  const count = await names.count();
  const values: string[] = [];

  for (let i = 0; i < count; i++) {
    values.push((await names.nth(i).textContent()) ?? '');
  }

  return values;
}

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

  test('deep cleaning complete buttons are touch-friendly', async ({ page }) => {
    await page.waitForSelector('#deep-cleaning-list', { state: 'visible', timeout: 5000 });
    
    const deepCleaningList = page.locator('#deep-cleaning-list');
    const completeBtns = deepCleaningList.locator('[data-testid="complete-btn"]');
    const count = await completeBtns.count();
    
    if (count > 0) {
      const firstBtn = completeBtns.first();
      const boundingBox = await firstBtn.boundingBox();
      
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

  test('deep cleaning complete button works on mobile', async ({ page }) => {
    await page.waitForSelector('#deep-cleaning-list', { state: 'visible', timeout: 5000 });

    const taskA = `Mobile Deep A ${Date.now()}`;
    const taskB = `Mobile Deep B ${Date.now()}`;

    await createDeepCleaningTask(page, taskA);
    await createDeepCleaningTask(page, taskB);

    const before = await deepTaskNames(page);
    expect(before.indexOf(taskA)).toBeLessThan(before.indexOf(taskB));

    const responsePromise = page.waitForResponse(
      response =>
        response.url().includes('/api/deep-cleaning/') &&
        response.url().includes('/complete') &&
        response.request().method() === 'POST',
    );
    await page
      .locator('#deep-cleaning-list .task-item', { has: page.locator('.task-name', { hasText: taskA }) })
      .locator('[data-testid="complete-btn"]')
      .click();
    await responsePromise;

    const after = await deepTaskNames(page);
    expect(after.indexOf(taskB)).toBeLessThan(after.indexOf(taskA));
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
