import { test, expect } from './fixtures';

test.describe('Intervals Feature', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
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

  test('creates a task with custom interval via UI', async ({ page }) => {
    const addButton = page.locator('[data-testid="add-daily-task-btn"]');
    await addButton.click();
    
    const modal = page.locator('#task-modal');
    await expect(modal).toBeVisible();
    
    const taskName = `Interval Task ${Date.now()}`;
    await page.locator('[data-testid="task-name-input"]').fill(taskName);
    await page.locator('[data-testid="task-day-select"]').selectOption('1');
    await page.locator('[data-testid="interval-weeks-input"]').fill('2');
    
    await page.locator('[data-testid="modal-save-btn"]').click();
    
    await expect(modal).not.toBeVisible({ timeout: 10000 });
    
    const tabAll = page.locator('[data-testid="tab-all"]');
    await tabAll.click();
    
    const allTasksList = page.locator('[data-testid="all-daily-tasks-list"]');
    await expect(allTasksList).toBeVisible();
    await expect(allTasksList.locator('.task-name', { hasText: taskName })).toBeVisible();
    
    const newTaskItem = allTasksList.locator('.task-item', { 
      has: page.locator('.task-name', { hasText: taskName }) 
    });
    await expect(newTaskItem.locator('.interval-badge', { hasText: 'alle 2 Wo.' })).toBeVisible();
  });

  test('verifies interval badge is displayed for bi-weekly tasks', async ({ page }) => {
    const addButton = page.locator('[data-testid="add-daily-task-btn"]');
    await addButton.click();
    
    const modal = page.locator('#task-modal');
    await expect(modal).toBeVisible();
    
    const taskName = `Triweekly Task ${Date.now()}`;
    await page.locator('[data-testid="task-name-input"]').fill(taskName);
    await page.locator('[data-testid="task-day-select"]').selectOption('2');
    await page.locator('[data-testid="interval-weeks-input"]').fill('3');
    await page.locator('[data-testid="modal-save-btn"]').click();
    
    await expect(modal).not.toBeVisible({ timeout: 10000 });
    
    const tabAll = page.locator('[data-testid="tab-all"]');
    await tabAll.click();
    
    const allTasksList = page.locator('[data-testid="all-daily-tasks-list"]');
    await expect(allTasksList).toBeVisible();
    
    const newTaskItem = allTasksList.locator('.task-item', { 
      has: page.locator('.task-name', { hasText: taskName }) 
    });
    await expect(newTaskItem.locator('.interval-badge', { hasText: 'alle 3 Wo.' })).toBeVisible();
  });

  test('allows editing a task interval', async ({ page }) => {
    const addButton = page.locator('[data-testid="add-daily-task-btn"]');
    await addButton.click();
    
    const modal = page.locator('#task-modal');
    await expect(modal).toBeVisible();
    
    const taskName = `Editable Interval Task ${Date.now()}`;
    await page.locator('[data-testid="task-name-input"]').fill(taskName);
    await page.locator('[data-testid="task-day-select"]').selectOption('3');
    await page.locator('[data-testid="interval-weeks-input"]').fill('1');
    await page.locator('[data-testid="modal-save-btn"]').click();
    
    await expect(modal).not.toBeVisible({ timeout: 10000 });
    
    const tabAll = page.locator('[data-testid="tab-all"]');
    await tabAll.click();
    
    const allTasksList = page.locator('[data-testid="all-daily-tasks-list"]');
    await expect(allTasksList).toBeVisible();
    
    const taskItem = allTasksList.locator('.task-item', { 
      has: page.locator('.task-name', { hasText: taskName }) 
    });
    const editButton = taskItem.locator('[data-testid="edit-btn"]');
    await editButton.click();
    
    await expect(modal).toBeVisible();
    
    await page.locator('[data-testid="interval-weeks-input"]').fill('4');
    await page.locator('[data-testid="modal-save-btn"]').click();
    
    await expect(modal).not.toBeVisible({ timeout: 10000 });
    
    await tabAll.click();
    
    const updatedTaskItem = allTasksList.locator('.task-item', { 
      has: page.locator('.task-name', { hasText: taskName }) 
    });
    await expect(updatedTaskItem.locator('.interval-badge', { hasText: 'alle 4 Wo.' })).toBeVisible();
  });

  test('hides interval field for mini-routines', async ({ page }) => {
    const addButton = page.locator('[data-testid="add-daily-task-btn"]');
    await addButton.click();
    
    const modal = page.locator('#task-modal');
    await expect(modal).toBeVisible();
    
    await page.locator('[data-testid="task-day-select"]').selectOption('-1');
    
    const intervalField = page.locator('#interval-group');
    await expect(intervalField).not.toBeVisible();
    
    await page.locator('[data-testid="task-day-select"]').selectOption('1');
    
    await expect(intervalField).toBeVisible();
  });
});
