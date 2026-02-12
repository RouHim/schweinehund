import { test, expect } from './fixtures';

test.describe('Start Date Hint Feature', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('#tasks-list', { state: 'visible', timeout: 5000 });
    await page.request.post('/api/debug/reset-all');
    await page.reload();
    await page.waitForSelector('#tasks-list', { state: 'visible', timeout: 5000 });
  });

  test.afterEach(async ({ page }) => {
    const modal = page.locator('#task-modal');
    const isVisible = await modal.isVisible().catch(() => false);
    if (isVisible) {
      await page.keyboard.press('Escape');
      await modal.waitFor({ state: 'hidden', timeout: 5000 }).catch(() => {});
    }
  });

  test('1. Hint appears when day_of_week mismatches start_date weekday', async ({ page }) => {
    const addButton = page.locator('[data-testid="add-daily-task-btn"]');
    await addButton.click();
    
    const modal = page.locator('#task-modal');
    await expect(modal).toBeVisible();
    
    await page.locator('[data-testid="task-name-input"]').fill('Test Task');
    await page.locator('[data-testid="task-day-select"]').selectOption('1');
    await page.locator('[data-testid="start-date-input"]').fill('2026-02-18');
    
    const hint = page.locator('[data-testid="start-date-hint"]');
    await expect(hint).toContainText('Erscheint erstmals am Mo, 23.02.2026');
  });

  test('2. Hint disappears when weekdays match', async ({ page }) => {
    const addButton = page.locator('[data-testid="add-daily-task-btn"]');
    await addButton.click();
    
    const modal = page.locator('#task-modal');
    await expect(modal).toBeVisible();
    
    await page.locator('[data-testid="task-name-input"]').fill('Test Task');
    await page.locator('[data-testid="task-day-select"]').selectOption('1');
    await page.locator('[data-testid="start-date-input"]').fill('2026-02-23');
    
    const hint = page.locator('[data-testid="start-date-hint"]');
    await expect(hint).toHaveText('');
  });

  test('3. No hint for mini-routine (day_of_week = -1)', async ({ page }) => {
    const addButton = page.locator('[data-testid="add-daily-task-btn"]');
    await addButton.click();
    
    const modal = page.locator('#task-modal');
    await expect(modal).toBeVisible();
    
    await page.locator('[data-testid="task-name-input"]').fill('Test Task');
    await page.locator('[data-testid="task-day-select"]').selectOption('-1');
    
    const hint = page.locator('[data-testid="start-date-hint"]');
    await expect(hint).toHaveText('');
  });

  test('4. Hint with biweekly interval', async ({ page }) => {
    const addButton = page.locator('[data-testid="add-daily-task-btn"]');
    await addButton.click();
    
    const modal = page.locator('#task-modal');
    await expect(modal).toBeVisible();
    
    await page.locator('[data-testid="task-name-input"]').fill('Biweekly Task');
    await page.locator('[data-testid="task-day-select"]').selectOption('1');
    await page.locator('[data-testid="start-date-input"]').fill('2026-02-18');
    await page.locator('[data-testid="interval-weeks-input"]').fill('2');
    
    const hint = page.locator('[data-testid="start-date-hint"]');
    await expect(hint).toContainText('Erscheint erstmals am Mo, 02.03.2026');
  });

  test('5. Dynamic update when day_of_week changes', async ({ page }) => {
    const addButton = page.locator('[data-testid="add-daily-task-btn"]');
    await addButton.click();
    
    const modal = page.locator('#task-modal');
    await expect(modal).toBeVisible();
    
    await page.locator('[data-testid="task-name-input"]').fill('Dynamic Task');
    await page.locator('[data-testid="start-date-input"]').fill('2026-02-18');
    
    await page.locator('[data-testid="task-day-select"]').selectOption('4');
    const hint = page.locator('[data-testid="start-date-hint"]');
    await expect(hint).toContainText('19.02.2026');
    
    await page.locator('[data-testid="task-day-select"]').selectOption('1');
    await expect(hint).toContainText('23.02.2026');
  });

  test('6. Dynamic update when start_date changes', async ({ page }) => {
    const addButton = page.locator('[data-testid="add-daily-task-btn"]');
    await addButton.click();
    
    const modal = page.locator('#task-modal');
    await expect(modal).toBeVisible();
    
    await page.locator('[data-testid="task-name-input"]').fill('Date Change Task');
    await page.locator('[data-testid="task-day-select"]').selectOption('4');
    
    await page.locator('[data-testid="start-date-input"]').fill('2026-02-18');
    const hint = page.locator('[data-testid="start-date-hint"]');
    await expect(hint).toContainText('19.02.2026');
    
    await page.locator('[data-testid="start-date-input"]').fill('2026-02-19');
    await expect(hint).toHaveText('');
  });

  test('7. Hint when editing existing task', async ({ page }) => {
    await page.request.post('/api/tasks', {
      data: {
        name: 'Existing Task',
        day_of_week: 1,
        start_date: '2026-02-18',
      },
    });

    const tabAll = page.locator('[data-testid="tab-all"]');
    await tabAll.click();
    
    const allTasksList = page.locator('[data-testid="all-daily-tasks-list"]');
    await expect(allTasksList).toBeVisible();

    const taskItem = allTasksList.locator('.task-item', {
      has: page.locator('.task-name', { hasText: 'Existing Task' })
    });
    const editButton = taskItem.locator('[data-testid="edit-btn"]');
    await editButton.click();
    
    const modal = page.locator('#task-modal');
    await expect(modal).toBeVisible();

    const hint = page.locator('[data-testid="start-date-hint"]');
    await expect(hint).toContainText('Erscheint erstmals am Mo, 23.02.2026');
  });

  test('8. No hint when start_date is empty', async ({ page }) => {
    const addButton = page.locator('[data-testid="add-daily-task-btn"]');
    await addButton.click();
    
    const modal = page.locator('#task-modal');
    await expect(modal).toBeVisible();
    
    await page.locator('[data-testid="task-name-input"]').fill('Empty Date Task');
    await page.locator('[data-testid="task-day-select"]').selectOption('4');
    
    const hint = page.locator('[data-testid="start-date-hint"]');
    await expect(hint).toHaveText('');
  });
});
