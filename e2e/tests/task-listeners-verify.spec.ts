import { test, expect } from '@playwright/test';

test.describe('attachTaskListeners verification', () => {
  test('function exists in global scope', async ({ page }) => {
    await page.goto('http://localhost:9666/');
    
    await page.waitForSelector('#tasks-list', { state: 'visible', timeout: 5000 });
    
    const hasFunction = await page.evaluate(() => {
      return typeof (window as any).attachTaskListeners === 'function';
    });
    expect(hasFunction).toBe(true);
  });

  test('no JavaScript errors on page load', async ({ page }) => {
    const errors: string[] = [];
    page.on('pageerror', (error) => {
      errors.push(error.message);
    });
    
    await page.goto('http://localhost:9666/');
    await page.waitForSelector('#tasks-list', { state: 'visible', timeout: 5000 });
    await page.waitForLoadState('networkidle');
    
    expect(errors).toHaveLength(0);
  });

  test('checkbox toggle updates state', async ({ page }) => {
    await page.goto('http://localhost:9666/');
    await page.waitForSelector('#tasks-list .task-checkbox', { state: 'visible', timeout: 5000 });
    
    const checkbox = await page.locator('#tasks-list .task-checkbox').first();
    const initialState = await checkbox.isChecked();
    
    await checkbox.click();
    await page.waitForLoadState('networkidle');
    
    const newState = await checkbox.isChecked();
    expect(newState).not.toBe(initialState);
  });

  test('edit button opens modal', async ({ page }) => {
    await page.goto('http://localhost:9666/');
    await page.waitForSelector('#tasks-list', { state: 'visible', timeout: 5000 });
    
    const editBtn = await page.locator('#tasks-list [data-testid="edit-btn"]').first();
    await editBtn.click();
    
    const modal = page.locator('#task-modal');
    await expect(modal).toBeVisible();
  });
});
