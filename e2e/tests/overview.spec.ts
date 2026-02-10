import { test, expect } from './fixtures';

test.describe('Overview Feature', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('#tasks-list', { state: 'visible', timeout: 5000 });
  });

  test('default tab is "Heute" on page load', async ({ page }) => {
    const tabToday = page.locator('[data-testid="tab-today"]');
    const tabAll = page.locator('[data-testid="tab-all"]');
    
    await expect(tabToday).toHaveClass(/active/);
    await expect(tabAll).not.toHaveClass(/active/);
    
    const todaySection = page.locator('#today-section');
    const allTasksSection = page.locator('#all-tasks-section');
    
    await expect(todaySection).toBeVisible();
    await expect(allTasksSection).not.toBeVisible();
  });

  test('switches tabs between Heute and Alle Aufgaben', async ({ page }) => {
    const tabToday = page.locator('[data-testid="tab-today"]');
    const tabAll = page.locator('[data-testid="tab-all"]');
    const todaySection = page.locator('#today-section');
    const allTasksSection = page.locator('#all-tasks-section');
    
    await tabAll.click();
    
    await expect(tabAll).toHaveClass(/active/);
    await expect(tabToday).not.toHaveClass(/active/);
    await expect(allTasksSection).toBeVisible();
    await expect(todaySection).not.toBeVisible();
    
    await tabToday.click();
    
    await expect(tabToday).toHaveClass(/active/);
    await expect(tabAll).not.toHaveClass(/active/);
    await expect(todaySection).toBeVisible();
    await expect(allTasksSection).not.toBeVisible();
  });

  test('overview displays all daily tasks', async ({ page }) => {
    const tabAll = page.locator('[data-testid="tab-all"]');
    await tabAll.click();
    
    const allDailyTasksList = page.locator('[data-testid="all-daily-tasks-list"]');
    await expect(allDailyTasksList).toBeVisible();
    
    const taskCount = await allDailyTasksList.locator('.task-item').count();
    expect(taskCount).toBeGreaterThan(0);
  });

  test('overview displays all deep cleaning tasks', async ({ page }) => {
    const tabAll = page.locator('[data-testid="tab-all"]');
    await tabAll.click();
    
    const allDeepTasksList = page.locator('[data-testid="all-deep-tasks-list"]');
    await expect(allDeepTasksList).toBeVisible();
    
    const taskCount = await allDeepTasksList.locator('.task-item').count();
    expect(taskCount).toBeGreaterThan(0);
  });

  test('allows editing a task from overview', async ({ page }) => {
    const tabAll = page.locator('[data-testid="tab-all"]');
    await tabAll.click();
    
    const allDailyTasksList = page.locator('[data-testid="all-daily-tasks-list"]');
    await expect(allDailyTasksList).toBeVisible();
    
    const firstTask = allDailyTasksList.locator('.task-item').first();
    const originalName = await firstTask.locator('.task-name').textContent();
    
    const editButton = firstTask.locator('[data-testid="edit-btn"]');
    await editButton.click();
    
    const modal = page.locator('#task-modal');
    await expect(modal).toBeVisible();
    
    const nameInput = page.locator('[data-testid="task-name-input"]');
    const currentValue = await nameInput.inputValue();
    expect(currentValue).toBe(originalName);
    
    const updatedName = `Edited from Overview ${Date.now()}`;
    await nameInput.fill(updatedName);
    await page.locator('[data-testid="modal-save-btn"]').click();
    
    await expect(modal).not.toBeVisible({ timeout: 10000 });
    
    const tabToday = page.locator('[data-testid="tab-today"]');
    await tabToday.click();
    await tabAll.click();
    
    await expect(allDailyTasksList.locator('.task-name', { hasText: updatedName })).toBeVisible();
  });

  test('allows deleting a task from overview', async ({ page }) => {
    const addButton = page.locator('[data-testid="add-daily-task-btn"]');
    await addButton.click();
    
    const modal = page.locator('#task-modal');
    await expect(modal).toBeVisible();
    
    const testTaskName = `Delete Test ${Date.now()}`;
    await page.locator('[data-testid="task-name-input"]').fill(testTaskName);
    await page.locator('[data-testid="task-day-select"]').selectOption('-1');
    await page.locator('[data-testid="modal-save-btn"]').click();
    
    await expect(modal).not.toBeVisible({ timeout: 10000 });
    
    const tabAll = page.locator('[data-testid="tab-all"]');
    await tabAll.click();
    
    const allDailyTasksList = page.locator('[data-testid="all-daily-tasks-list"]');
    await expect(allDailyTasksList).toBeVisible();
    
    const initialCount = await allDailyTasksList.locator('.task-item').count();
    
    const testTask = allDailyTasksList.locator('.task-item', { 
      has: page.locator('.task-name', { hasText: testTaskName }) 
    });
    
    page.on('dialog', dialog => {
      expect(dialog.message()).toContain('Möchtest du diese Aufgabe wirklich löschen?');
      dialog.accept();
    });
    
    const deleteButton = testTask.locator('[data-testid="delete-btn"]');
    await deleteButton.click();
    
    await page.waitForLoadState('networkidle');
    
    const tabToday = page.locator('[data-testid="tab-today"]');
    await tabToday.click();
    await tabAll.click();
    
    const updatedCount = await allDailyTasksList.locator('.task-item').count();
    expect(updatedCount).toBe(initialCount - 1);
    
    const deletedTask = allDailyTasksList.locator('.task-name', { hasText: testTaskName });
    await expect(deletedTask).not.toBeVisible();
  });

  test('changes in overview reflect in Heute view', async ({ page }) => {
    const currentDay = await page.evaluate(() => {
      const jsDay = new Date().getDay();
      return jsDay === 0 ? '7' : String(jsDay);
    });
    
    const addButton = page.locator('[data-testid="add-daily-task-btn"]');
    await addButton.click();
    
    const modal = page.locator('#task-modal');
    await expect(modal).toBeVisible();
    
    const testTaskName = `Sync Test ${Date.now()}`;
    await page.locator('[data-testid="task-name-input"]').fill(testTaskName);
    await page.locator('[data-testid="task-desc-input"]').fill('Original Description');
    await page.locator('[data-testid="task-day-select"]').selectOption(currentDay);
    await page.locator('[data-testid="modal-save-btn"]').click();
    
    await expect(modal).not.toBeVisible({ timeout: 10000 });
    
    const tabAll = page.locator('[data-testid="tab-all"]');
    await tabAll.click();
    
    const allDailyTasksList = page.locator('[data-testid="all-daily-tasks-list"]');
    await expect(allDailyTasksList).toBeVisible();
    
    const testTask = allDailyTasksList.locator('.task-item', { 
      has: page.locator('.task-name', { hasText: testTaskName }) 
    });
    const editButton = testTask.locator('[data-testid="edit-btn"]');
    await editButton.click();
    
    await expect(modal).toBeVisible();
    
    const updatedDescription = 'Updated from Overview';
    await page.locator('[data-testid="task-desc-input"]').fill(updatedDescription);
    await page.locator('[data-testid="modal-save-btn"]').click();
    
    await expect(modal).not.toBeVisible({ timeout: 10000 });
    
    const tabToday = page.locator('[data-testid="tab-today"]');
    await tabToday.click();
    
    const todayTasksList = page.locator('#tasks-list');
    await expect(todayTasksList).toBeVisible();
    
    const taskInTodayView = todayTasksList.locator('.task-item', { 
      has: page.locator('.task-name', { hasText: testTaskName }) 
    });
    await expect(taskInTodayView.locator('.task-description', { hasText: updatedDescription })).toBeVisible();
  });
});
