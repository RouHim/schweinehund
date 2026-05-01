import { test, expect } from './fixtures';

test.describe('Start Date Feature', () => {
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

  test('creates a task with custom start date via UI', async ({ page }) => {
    const addButton = page.locator('[data-testid="add-daily-task-btn"]');
    await addButton.click();
    
    const modal = page.locator('#task-modal');
    await expect(modal).toBeVisible();
    
    await page.locator('[data-testid="task-name-input"]').fill('Future Task');
    await page.locator('[data-testid="task-day-select"]').selectOption('1');
    await page.locator('[data-testid="start-date-input"]').fill('2099-01-01');
    
    await page.locator('[data-testid="modal-save-btn"]').click();
    
    await expect(modal).not.toBeVisible({ timeout: 10000 });
    
    const tabAll = page.locator('[data-testid="tab-all"]');
    await tabAll.click();
    
    const allTasksList = page.locator('[data-testid="all-daily-tasks-list"]');
    await expect(allTasksList).toBeVisible();
    
    const taskItem = allTasksList.locator('.task-item', {
      has: page.locator('.task-name', { hasText: 'Future Task' })
    });
    await expect(taskItem).toBeVisible();
    await expect(taskItem.locator('.start-date-badge', { hasText: 'ab 2099-01-01' })).toBeVisible();
  });

  test('hides future-dated task from today view', async ({ page }) => {
    await page.request.post('/api/tasks', {
      data: {
        name: 'Future Weekly Task',
        day_of_week: 1,
        start_date: '2099-01-01',
      },
    });

    await page.reload();
    await page.waitForSelector('#tasks-list', { state: 'visible', timeout: 5000 });

    const todayTasksList = page.locator('#tasks-list');
    await expect(todayTasksList).not.toContainText('Future Weekly Task');

    const tabAll = page.locator('[data-testid="tab-all"]');
    await tabAll.click();
    
    const allTasksList = page.locator('[data-testid="all-daily-tasks-list"]');
    await expect(allTasksList).toBeVisible();
    await expect(allTasksList).toContainText('Future Weekly Task');
  });

  test('shows task with today start date in today view', async ({ page }) => {
    const today = new Date().toISOString().split('T')[0];
    const todayDayOfWeek = new Date().getDay() === 0 ? 7 : new Date().getDay();

    await page.request.post('/api/tasks', {
      data: {
        name: 'Today Task',
        day_of_week: todayDayOfWeek,
        start_date: today,
      },
    });

    await page.reload();
    await page.waitForSelector('#tasks-list', { state: 'visible', timeout: 5000 });

    const todayTasksList = page.locator('#tasks-list');
    await expect(todayTasksList).toContainText('Today Task');
  });

  test('start date field pre-populates when editing', async ({ page }) => {
    await page.request.post('/api/tasks', {
      data: {
        name: 'Task with Date',
        day_of_week: 1,
        start_date: '2026-03-15',
      },
    });

    const tabAll = page.locator('[data-testid="tab-all"]');
    await tabAll.click();
    
    const allTasksList = page.locator('[data-testid="all-daily-tasks-list"]');
    await expect(allTasksList).toBeVisible();

    const taskItem = allTasksList.locator('.task-item', {
      has: page.locator('.task-name', { hasText: 'Task with Date' })
    });
    const editButton = taskItem.locator('[data-testid="edit-btn"]');
    await editButton.click();
    
    const modal = page.locator('#task-modal');
    await expect(modal).toBeVisible();

    const startDateInput = page.locator('[data-testid="start-date-input"]');
    await expect(startDateInput).toHaveValue('2026-03-15');
  });

  test('hides start date field for mini-routines', async ({ page }) => {
    const addButton = page.locator('[data-testid="add-daily-task-btn"]');
    await addButton.click();
    
    const modal = page.locator('#task-modal');
    await expect(modal).toBeVisible();

    await page.locator('[data-testid="task-day-select"]').selectOption('1');
    const startDateGroup = page.locator('#start-date-group');
    await expect(startDateGroup).toBeVisible();

    await page.locator('[data-testid="task-day-select"]').selectOption('-1');
    await expect(startDateGroup).not.toBeVisible();
  });

  test('allows editing start date on existing task', async ({ page }) => {
    await page.request.post('/api/tasks', {
      data: {
        name: 'Original Date Task',
        day_of_week: 1,
        start_date: '2099-02-15',
      },
    });

    const tabAll = page.locator('[data-testid="tab-all"]');
    await tabAll.click();
    
    const allTasksList = page.locator('[data-testid="all-daily-tasks-list"]');
    await expect(allTasksList).toBeVisible();

    const taskItem = allTasksList.locator('.task-item', {
      has: page.locator('.task-name', { hasText: 'Original Date Task' })
    });
    const editButton = taskItem.locator('[data-testid="edit-btn"]');
    await editButton.click();
    
    const modal = page.locator('#task-modal');
    await expect(modal).toBeVisible();

    await page.locator('[data-testid="start-date-input"]').fill('2099-03-20');
    await page.locator('[data-testid="modal-save-btn"]').click();
    
    await expect(modal).not.toBeVisible({ timeout: 10000 });

    await tabAll.click();

    const updatedTaskItem = allTasksList.locator('.task-item', {
      has: page.locator('.task-name', { hasText: 'Original Date Task' })
    });
    await expect(updatedTaskItem.locator('.start-date-badge', { hasText: 'ab 2099-03-20' })).toBeVisible();
  });
});
