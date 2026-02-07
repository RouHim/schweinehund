import { test, expect, type Page } from '@playwright/test';

async function createDeepCleaningTask(page: Page, name: string) {
  await page.locator('[data-testid="add-deep-cleaning-btn"]').click();
  await expect(page.locator('#task-modal')).toBeVisible();
  await page.locator('[data-testid="task-name-input"]').fill(name);
  await page.locator('[data-testid="modal-save-btn"]').click();
  await expect(page.locator('#task-modal')).not.toBeVisible({ timeout: 10000 });
  await expect(page.locator('#deep-cleaning-list .task-name', { hasText: name })).toBeVisible();
}

async function taskNames(page: Page): Promise<string[]> {
  const items = page.locator('#deep-cleaning-list .task-item .task-name');
  const count = await items.count();
  const names: string[] = [];

  for (let i = 0; i < count; i++) {
    const name = await items.nth(i).textContent();
    names.push(name ?? '');
  }

  return names;
}

test.describe('Deep Cleaning Queue', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('#deep-cleaning-list', { state: 'visible', timeout: 5000 });
  });

  test('displays deep cleaning tasks', async ({ page }) => {
    const deepCleaningList = page.locator('#deep-cleaning-list');
    await expect(deepCleaningList).toBeVisible();
    
    const deepCleaningItems = deepCleaningList.locator('.task-item');
    const count = await deepCleaningItems.count();
    expect(count).toBeGreaterThan(0);
    
    const firstItem = deepCleaningItems.first();
    await expect(firstItem.locator('.task-name')).toBeVisible();
    await expect(firstItem.locator('.task-checkbox')).toBeVisible();
  });

  test('completes deep cleaning task and rotates queue', async ({ page }) => {
    const taskA = `Deep Rotate A ${Date.now()}`;
    const taskB = `Deep Rotate B ${Date.now()}`;

    await createDeepCleaningTask(page, taskA);
    await createDeepCleaningTask(page, taskB);

    const before = await taskNames(page);
    expect(before.indexOf(taskA)).toBeLessThan(before.indexOf(taskB));

    const responsePromise = page.waitForResponse(
      response =>
        response.url().includes('/api/deep-cleaning/') &&
        response.url().includes('/complete') &&
        response.request().method() === 'POST',
    );
    await page
      .locator('#deep-cleaning-list .task-item', { has: page.locator('.task-name', { hasText: taskA }) })
      .locator('.task-checkbox')
      .click();
    await responsePromise;

    const after = await taskNames(page);
    expect(after.indexOf(taskB)).toBeLessThan(after.indexOf(taskA));
  });

  test('completed task moves to end of queue', async ({ page }) => {
    const taskA = `Deep End A ${Date.now()}`;
    const taskB = `Deep End B ${Date.now()}`;

    await createDeepCleaningTask(page, taskA);
    await createDeepCleaningTask(page, taskB);

    const responsePromise = page.waitForResponse(
      response =>
        response.url().includes('/api/deep-cleaning/') &&
        response.url().includes('/complete') &&
        response.request().method() === 'POST',
    );
    await page
      .locator('#deep-cleaning-list .task-item', { has: page.locator('.task-name', { hasText: taskA }) })
      .locator('.task-checkbox')
      .click();
    await responsePromise;

    const names = await taskNames(page);
    expect(names[names.length - 1]).toBe(taskA);
  });

  test('queue state persists after page reload', async ({ page }) => {
    const deepCleaningList = page.locator('#deep-cleaning-list');
    const deepCleaningItems = deepCleaningList.locator('.task-item');
    
    const beforeReloadState = await deepCleaningItems.all().then(async items => {
      return Promise.all(items.map(item => item.locator('.task-name').textContent()));
    });
    
    await page.reload();
    await page.waitForSelector('#deep-cleaning-list', { state: 'visible', timeout: 5000 });
    
    const reloadedItems = page.locator('#deep-cleaning-list .task-item');
    const afterReloadState = await reloadedItems.all().then(async items => {
      return Promise.all(items.map(item => item.locator('.task-name').textContent()));
    });
    
    expect(afterReloadState).toEqual(beforeReloadState);
  });
});
