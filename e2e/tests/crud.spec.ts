import { test, expect } from './fixtures';

test.describe('CRUD Operations - Daily Tasks', () => {
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

  test('creates a new daily task via modal', async ({ page }) => {
    const addButton = page.locator('[data-testid="add-daily-task-btn"]');
    await addButton.click();
    
    // Wait for modal to appear
    const modal = page.locator('#task-modal');
    await expect(modal).toBeVisible();
    
    // Fill form fields
    const taskName = `Test Daily Task ${Date.now()}`;
    await page.locator('[data-testid="task-name-input"]').fill(taskName);
    await page.locator('[data-testid="task-desc-input"]').fill('Test description');
    await page.locator('[data-testid="task-zone-input"]').fill('Test Zone');
    const currentDay = await page.evaluate(() => {
      const jsDay = new Date().getDay();
      return jsDay === 0 ? '7' : String(jsDay);
    });
    await page.locator('[data-testid="task-day-select"]').selectOption(currentDay);
    
    // Submit form
    await page.locator('[data-testid="modal-save-btn"]').click();
    
    await expect(modal).not.toBeVisible({ timeout: 10000 });
    
    // Verify task appears in list
    const tasksList = page.locator('#tasks-list');
    await expect(tasksList.locator('.task-name', { hasText: taskName })).toBeVisible();
    
    // Verify metadata
    const newTaskItem = tasksList.locator('.task-item', { has: page.locator('.task-name', { hasText: taskName }) });
    await expect(newTaskItem.locator('.task-badge', { hasText: 'Test Zone' })).toBeVisible();
    const dayNames: Record<string, string> = {
      '1': 'Montag',
      '2': 'Dienstag',
      '3': 'Mittwoch',
      '4': 'Donnerstag',
      '5': 'Freitag',
      '6': 'Samstag',
      '7': 'Sonntag',
    };
    await expect(newTaskItem.locator('.task-badge', { hasText: dayNames[currentDay] })).toBeVisible();
  });

  test('edits a daily task and modal pre-populates', async ({ page }) => {
    const tasksList = page.locator('#tasks-list');
    
    // Get first task's name
    const firstTask = tasksList.locator('.task-item').first();
    const originalName = await firstTask.locator('.task-name').textContent();
    
    // Click edit button
    const editButton = firstTask.locator('[data-testid="edit-btn"]');
    await editButton.click();
    
    // Verify modal opens and pre-populates
    const modal = page.locator('#task-modal');
    await expect(modal).toBeVisible();
    
    const nameInput = page.locator('[data-testid="task-name-input"]');
    const currentValue = await nameInput.inputValue();
    expect(currentValue).toBe(originalName);
    
    // Edit the task
    const updatedName = `${originalName} EDITED`;
    await nameInput.fill(updatedName);
    await page.locator('[data-testid="task-desc-input"]').fill('Updated description');
    
    const responsePromise = page.waitForResponse(response => 
      response.url().includes('/api/tasks/') && response.request().method() === 'PUT'
    );
    
    await page.locator('[data-testid="modal-save-btn"]').click();
    
    const response = await responsePromise;
    expect(response.ok()).toBeTruthy();
    
    await page.waitForFunction(() => {
      const modal = document.getElementById('task-modal') as HTMLDialogElement;
      return !modal || !modal.open;
    }, { timeout: 10000 });
    
    await expect(tasksList.locator('.task-name', { hasText: updatedName })).toBeVisible({ timeout: 10000 });
  });

  test('deletes a daily task with confirmation', async ({ page }) => {
    const tasksList = page.locator('#tasks-list');
    
    // Get initial count
    const initialCount = await tasksList.locator('.task-item').count();
    expect(initialCount).toBeGreaterThan(0);
    
    // Get task name we're about to delete
    const firstTask = tasksList.locator('.task-item').first();
    const taskName = await firstTask.locator('.task-name').textContent();
    
    // Setup dialog handler before clicking delete
    page.on('dialog', dialog => {
      expect(dialog.message()).toContain('Möchtest du diese Aufgabe wirklich löschen?');
      dialog.accept();
    });
    
    const responsePromise = page.waitForResponse(response => 
      response.url().includes('/api/tasks/') && response.request().method() === 'DELETE'
    );
    
    const deleteButton = firstTask.locator('[data-testid="delete-btn"]');
    await deleteButton.click();
    
    await responsePromise;
    
    await page.waitForLoadState('networkidle');
    
    const updatedCount = await tasksList.locator('.task-item').count();
    expect(updatedCount).toBe(initialCount - 1);
    
    const deletedTask = tasksList.locator('.task-name', { hasText: taskName || '' });
    await expect(deletedTask).not.toBeVisible();
  });
});

test.describe('CRUD Operations - Deep Cleaning Tasks', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('#deep-cleaning-list', { state: 'visible', timeout: 5000 });
  });

  test.afterEach(async ({ page }) => {
    const modal = page.locator('#task-modal');
    const isVisible = await modal.isVisible().catch(() => false);
    if (isVisible) {
      await page.keyboard.press('Escape');
      await modal.waitFor({ state: 'hidden', timeout: 5000 }).catch(() => {});
    }
  });

  test('creates a new deep cleaning task', async ({ page }) => {
    const addButton = page.locator('[data-testid="add-deep-cleaning-btn"]');
    await addButton.click();
    
    // Wait for modal to appear
    const modal = page.locator('#task-modal');
    await expect(modal).toBeVisible();
    
    // Verify day_of_week field is hidden for deep cleaning
    const dayField = page.locator('#day-of-week-field');
    await expect(dayField).not.toBeVisible();
    
    // Fill form fields
    const taskName = `Test Deep Cleaning ${Date.now()}`;
    await page.locator('[data-testid="task-name-input"]').fill(taskName);
    await page.locator('[data-testid="task-desc-input"]').fill('Deep cleaning test');
    await page.locator('[data-testid="task-zone-input"]').fill('Basement');
    
    // Submit form
    await page.locator('[data-testid="modal-save-btn"]').click();
    
    await expect(modal).not.toBeVisible({ timeout: 10000 });
    
    // Verify task appears at end of queue
    const deepCleaningList = page.locator('#deep-cleaning-list');
    await expect(deepCleaningList.locator('.task-name', { hasText: taskName })).toBeVisible();
    
    // Verify it's in the queue (has queue badge)
    const newTaskItem = deepCleaningList.locator('.task-item', { has: page.locator('.task-name', { hasText: taskName }) });
    await expect(newTaskItem.locator('.deep-cleaning-position')).toBeVisible();
  });

  test('edits a deep cleaning task and modal pre-populates', async ({ page }) => {
    const deepCleaningList = page.locator('#deep-cleaning-list');
    
    // Get first task's name
    const firstTask = deepCleaningList.locator('.task-item').first();
    const originalName = await firstTask.locator('.task-name').textContent();
    
    // Click edit button
    const editButton = firstTask.locator('[data-testid="edit-btn"]');
    await editButton.click();
    
    // Verify modal opens and pre-populates
    const modal = page.locator('#task-modal');
    await expect(modal).toBeVisible();
    
    const nameInput = page.locator('[data-testid="task-name-input"]');
    const currentValue = await nameInput.inputValue();
    expect(currentValue).toBe(originalName);
    
    // Verify day field is hidden
    const dayField = page.locator('#day-of-week-field');
    await expect(dayField).not.toBeVisible();
    
    // Edit the task
    const updatedName = `${originalName} UPDATED`;
    await nameInput.fill(updatedName);
    
    const responsePromise = page.waitForResponse(response => 
      response.url().includes('/api/deep-cleaning/') && response.request().method() === 'PUT'
    );
    
    await page.locator('[data-testid="modal-save-btn"]').click();
    
    await responsePromise;
    await page.waitForLoadState('networkidle');
    
    await expect(modal).not.toBeVisible({ timeout: 10000 });
    
    await expect(deepCleaningList.locator('.task-name', { hasText: updatedName })).toBeVisible();
  });

  test('deletes a deep cleaning task', async ({ page }) => {
    const deepCleaningList = page.locator('#deep-cleaning-list');
    
    const initialCount = await deepCleaningList.locator('.task-item').count();
    expect(initialCount).toBeGreaterThan(0);
    
    const secondTask = deepCleaningList.locator('.task-item').nth(1);
    const taskName = await secondTask.locator('.task-name').textContent();
    
    page.on('dialog', dialog => {
      expect(dialog.message()).toContain('Möchtest du diese Aufgabe wirklich löschen?');
      dialog.accept();
    });
    
    const responsePromise = page.waitForResponse(response => 
      response.url().includes('/api/deep-cleaning/') && response.request().method() === 'DELETE'
    );
    
    const deleteButton = secondTask.locator('[data-testid="delete-btn"]');
    await deleteButton.click();
    
    await responsePromise;
    
    await page.waitForLoadState('networkidle');
    
    const updatedCount = await deepCleaningList.locator('.task-item').count();
    expect(updatedCount).toBe(initialCount - 1);
    
    const deletedTask = deepCleaningList.locator('.task-name', { hasText: taskName || '' });
    await expect(deletedTask).not.toBeVisible();
  });
});

test.describe('Persistence Tests', () => {
  test('daily task persists after page reload', async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('#tasks-list', { state: 'visible', timeout: 5000 });
    
    // Create a unique task
    const taskName = `Persistence Test ${Date.now()}`;
    const addButton = page.locator('[data-testid="add-daily-task-btn"]');
    await addButton.click();
    
    const modal = page.locator('#task-modal');
    await expect(modal).toBeVisible();
    
    await page.locator('[data-testid="task-name-input"]').fill(taskName);
    await page.locator('[data-testid="task-day-select"]').selectOption('-1');
    await page.locator('[data-testid="modal-save-btn"]').click();
    
    // Verify task exists
    const tasksList = page.locator('#tasks-list');
    await expect(tasksList.locator('.task-name', { hasText: taskName })).toBeVisible();
    
    // Reload page
    await page.reload();
    await page.waitForSelector('#tasks-list', { state: 'visible', timeout: 5000 });
    
    // Verify task still exists
    const reloadedTasksList = page.locator('#tasks-list');
    await expect(reloadedTasksList.locator('.task-name', { hasText: taskName })).toBeVisible();
  });

  test('deep cleaning task persists after page reload', async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('#deep-cleaning-list', { state: 'visible', timeout: 5000 });
    
    // Create a unique task
    const taskName = `Deep Persistence ${Date.now()}`;
    const addButton = page.locator('[data-testid="add-deep-cleaning-btn"]');
    await addButton.click();
    
    const modal = page.locator('#task-modal');
    await expect(modal).toBeVisible();
    
    await page.locator('[data-testid="task-name-input"]').fill(taskName);
    await page.locator('[data-testid="modal-save-btn"]').click();
    
    // Verify task exists
    const deepCleaningList = page.locator('#deep-cleaning-list');
    await expect(deepCleaningList.locator('.task-name', { hasText: taskName })).toBeVisible();
    
    // Reload page
    await page.reload();
    await page.waitForSelector('#deep-cleaning-list', { state: 'visible', timeout: 5000 });
    
    // Verify task still exists
    const reloadedList = page.locator('#deep-cleaning-list');
    await expect(reloadedList.locator('.task-name', { hasText: taskName })).toBeVisible();
  });
});
