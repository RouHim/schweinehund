import { test, expect } from './fixtures';
import { type Page } from '@playwright/test';

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
    await expect(firstItem.locator('[data-testid="complete-btn"]')).toBeVisible();
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
      .locator('[data-testid="complete-btn"]')
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
      .locator('[data-testid="complete-btn"]')
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

  test.describe('Reordering', () => {
    test('grip handle is visible on each task', async ({ page }) => {
      const taskCount = await page.locator('#deep-cleaning-list .task-item').count();
      if (taskCount === 0) {
        await createDeepCleaningTask(page, `Grip Test ${Date.now()}`);
      }
      
      const handles = page.locator('[data-testid="drag-handle"]');
      const handleCount = await handles.count();
      expect(handleCount).toBeGreaterThan(0);
      
      const firstHandle = handles.first();
      const box = await firstHandle.boundingBox();
      expect(box).not.toBeNull();
      expect(box!.width).toBeGreaterThanOrEqual(44);
      expect(box!.height).toBeGreaterThanOrEqual(44);
    });

    test('arrow buttons reorder tasks', async ({ page }) => {
      const taskA = `Arrow A ${Date.now()}`;
      const taskB = `Arrow B ${Date.now()}`;
      const taskC = `Arrow C ${Date.now()}`;
      
      await createDeepCleaningTask(page, taskA);
      await createDeepCleaningTask(page, taskB);
      await createDeepCleaningTask(page, taskC);
      
      const before = await taskNames(page);
      expect(before.indexOf(taskA)).toBeLessThan(before.indexOf(taskB));
      
      const responsePromise = page.waitForResponse(r => r.url().includes('/api/deep-cleaning/reorder'));
      await page
        .locator('#deep-cleaning-list .task-item', { has: page.locator('.task-name', { hasText: taskA }) })
        .locator('[data-testid="move-down-btn"]')
        .click();
      await responsePromise;
      
      const after = await taskNames(page);
      expect(after.indexOf(taskB)).toBeLessThan(after.indexOf(taskA));
      
      const positions = await page.locator('.deep-cleaning-position').allTextContents();
      expect(positions).toEqual(['#1', '#2', '#3']);
    });

    test('first item move-up is disabled, last item move-down is disabled', async ({ page }) => {
      const taskCount = await page.locator('#deep-cleaning-list .task-item').count();
      if (taskCount < 2) {
        await createDeepCleaningTask(page, `Disabled A ${Date.now()}`);
        await createDeepCleaningTask(page, `Disabled B ${Date.now()}`);
      }
      
      const firstMoveUp = page.locator('#deep-cleaning-list .task-item').first().locator('[data-testid="move-up-btn"]');
      await expect(firstMoveUp).toBeDisabled();
      
      const lastMoveDown = page.locator('#deep-cleaning-list .task-item').last().locator('[data-testid="move-down-btn"]');
      await expect(lastMoveDown).toBeDisabled();
    });

    test('reorder persists after page reload', async ({ page }) => {
      const taskA = `Persist A ${Date.now()}`;
      const taskB = `Persist B ${Date.now()}`;
      
      await createDeepCleaningTask(page, taskA);
      await createDeepCleaningTask(page, taskB);
      
      const responsePromise = page.waitForResponse(r => r.url().includes('/api/deep-cleaning/reorder'));
      await page
        .locator('#deep-cleaning-list .task-item', { has: page.locator('.task-name', { hasText: taskA }) })
        .locator('[data-testid="move-down-btn"]')
        .click();
      await responsePromise;
      
      const afterReorder = await taskNames(page);
      expect(afterReorder.indexOf(taskB)).toBeLessThan(afterReorder.indexOf(taskA));
      
      await page.reload();
      await page.waitForSelector('#deep-cleaning-list', { state: 'visible', timeout: 5000 });
      
      const afterReload = await taskNames(page);
      expect(afterReload.indexOf(taskB)).toBeLessThan(afterReload.indexOf(taskA));
    });

    test('drag-and-drop reorders tasks', async ({ page }) => {
      const taskA = `Drag A ${Date.now()}`;
      const taskB = `Drag B ${Date.now()}`;
      const taskC = `Drag C ${Date.now()}`;
      
      await createDeepCleaningTask(page, taskA);
      await createDeepCleaningTask(page, taskB);
      await createDeepCleaningTask(page, taskC);
      
      const before = await taskNames(page);
      const cIdx = before.indexOf(taskC);
      const aIdx = before.indexOf(taskA);
      expect(cIdx).toBeGreaterThan(aIdx);
      
      const responsePromise = page.waitForResponse(r => r.url().includes('/api/deep-cleaning/reorder'));
      
      const handleC = page
        .locator('#deep-cleaning-list .task-item', { has: page.locator('.task-name', { hasText: taskC }) })
        .locator('[data-testid="drag-handle"]');
      const handleA = page
        .locator('#deep-cleaning-list .task-item', { has: page.locator('.task-name', { hasText: taskA }) })
        .locator('[data-testid="drag-handle"]');
      
      await handleC.dragTo(handleA);
      await responsePromise;
      
      const after = await taskNames(page);
      expect(after.indexOf(taskC)).toBeLessThan(after.indexOf(taskA));
    });
  });
});
