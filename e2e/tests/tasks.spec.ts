import { test, expect } from './fixtures';
import { type Locator, type Page } from '@playwright/test';

function currentApiDay(): string {
  const jsDay = new Date().getDay();
  return jsDay === 0 ? '7' : String(jsDay);
}

async function createTodayTask(page: Page, name: string): Promise<Locator> {
  await page.locator('[data-testid="add-daily-task-btn"]').click();
  await expect(page.locator('#task-modal')).toBeVisible();

  await page.locator('[data-testid="task-name-input"]').fill(name);
  await page.locator('[data-testid="task-day-select"]').selectOption(currentApiDay());
  await page.locator('[data-testid="modal-save-btn"]').click();

  await expect(page.locator('#task-modal')).not.toBeVisible({ timeout: 10000 });

  const taskItem = page.locator('#tasks-list .task-item', {
    has: page.locator('.task-name', { hasText: name }),
  });
  await expect(taskItem).toBeVisible();

  return taskItem;
}

async function toggleTask(taskItem: Locator, page: Page): Promise<void> {
  const responsePromise = page.waitForResponse(
    response =>
      response.url().includes('/api/tasks/') &&
      response.url().includes('/toggle') &&
      response.request().method() === 'POST',
  );

  await taskItem.locator('.task-checkbox').click();
  await responsePromise;
}

test.describe('Task Management', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('#tasks-list', { state: 'visible', timeout: 5000 });
    const response = await page.request.post('/api/debug/reset-all');
    expect(response.ok()).toBeTruthy();
    await page.reload();
    await page.waitForSelector('#tasks-list', { state: 'visible', timeout: 5000 });
  });

  test('displays daily tasks on page load', async ({ page }) => {
    const tasksList = page.locator('#tasks-list');
    await expect(tasksList).toBeVisible();

    const taskItems = tasksList.locator('.task-item');
    const count = await taskItems.count();
    expect(count).toBeGreaterThan(0);

    const firstTask = taskItems.first();
    await expect(firstTask.locator('.task-checkbox')).toBeVisible();
    await expect(firstTask.locator('.task-name')).toBeVisible();
  });

   test('toggles task completion state', async ({ page }) => {
     const taskName = 'Kueche grob aufraeumen';
     const taskItem = page.locator('#tasks-list .task-item', {
       has: page.locator('.task-name', { hasText: taskName }),
     });

     const wasChecked = await taskItem.locator('.task-checkbox').isChecked();
     await toggleTask(taskItem, page);

     const reLocatedTaskItem = page.locator('#tasks-list .task-item', {
       has: page.locator('.task-name', { hasText: taskName }),
     });

     const isNowChecked = await reLocatedTaskItem.locator('.task-checkbox').isChecked();
     expect(isNowChecked).toBe(!wasChecked);

     if (isNowChecked) {
       await expect(reLocatedTaskItem).toHaveClass(/completed/);
     } else {
       await expect(reLocatedTaskItem).not.toHaveClass(/completed/);
     }
   });

  test('persists task completion across page reload', async ({ page }) => {
    const taskName = `Persist Toggle ${Date.now()}`;
    const taskItem = await createTodayTask(page, taskName);

    await toggleTask(taskItem, page);
    await expect(taskItem.locator('.task-checkbox')).toBeChecked();

    await page.reload();
    await page.waitForSelector('#tasks-list', { state: 'visible', timeout: 5000 });

    const reloadedTask = page.locator('#tasks-list .task-item', {
      has: page.locator('.task-name', { hasText: taskName }),
    });
    await expect(reloadedTask.locator('.task-checkbox')).toBeChecked();
  });

  test('unchecks task and persists', async ({ page }) => {
    const taskName = `Persist Uncheck ${Date.now()}`;
    const taskItem = await createTodayTask(page, taskName);

    await toggleTask(taskItem, page);
    await expect(taskItem.locator('.task-checkbox')).toBeChecked();

    // Re-query task after first toggle (task moved in DOM)
    const reQueriedTask = page.locator('#tasks-list .task-item', {
      has: page.locator('.task-name', { hasText: taskName }),
    });
    await toggleTask(reQueriedTask, page);
    await expect(reQueriedTask.locator('.task-checkbox')).not.toBeChecked();

    await page.reload();
    await page.waitForSelector('#tasks-list', { state: 'visible', timeout: 5000 });

    const reloadedTask = page.locator('#tasks-list .task-item', {
      has: page.locator('.task-name', { hasText: taskName }),
    });
    await expect(reloadedTask.locator('.task-checkbox')).not.toBeChecked();
  });

  test('moves completed task to bottom of list after reload', async ({ page }) => {
    const taskName = `Bottom Move ${Date.now()}`;
    const taskItem = await createTodayTask(page, taskName);

    await toggleTask(taskItem, page);

    await page.reload();
    await page.waitForSelector('#tasks-list', { state: 'visible', timeout: 5000 });

    const lastTaskName = page.locator('#tasks-list .task-item').last().locator('.task-name');
    await expect(lastTaskName).toHaveText(taskName);
  });

  test('applies completed styling to completed task', async ({ page }) => {
    const taskName = `Opacity Check ${Date.now()}`;
    const taskItem = await createTodayTask(page, taskName);

    await toggleTask(taskItem, page);
    await expect(taskItem).toHaveClass(/completed/);

    await expect.poll(async () => {
      return taskItem.evaluate(el => window.getComputedStyle(el).opacity);
    }).toBe('0.5');
  });

  test('keeps completed tasks grouped at the bottom after reload', async ({ page }) => {
    const taskName = `Grouping ${Date.now()}`;
    const taskItem = await createTodayTask(page, taskName);

    await toggleTask(taskItem, page);

    await page.reload();
    await page.waitForSelector('#tasks-list', { state: 'visible', timeout: 5000 });

    const classes = await page
      .locator('#tasks-list .task-item')
      .evaluateAll(elements => elements.map(element => element.classList.contains('completed')));

    const firstCompletedIndex = classes.indexOf(true);
    if (firstCompletedIndex !== -1) {
      expect(classes.slice(0, firstCompletedIndex).every(value => !value)).toBeTruthy();
      expect(classes.slice(firstCompletedIndex).every(value => value)).toBeTruthy();
    }
  });
});
