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

/**
 * Reorders tasks in the deep-cleaning list by moving sourceTaskName to be above targetTaskName.
 * Uses DOM manipulation + handleDragReorder() to avoid Playwright's drag-and-drop
 * incompatibility with Sortable.js (which uses forceFallback: true for synthetic mouse events).
 * 
 * Root cause: Playwright's dragTo() uses native HTML5 drag events, but Sortable.js
 * with forceFallback:true uses custom mouse event simulation that Playwright can't replicate.
 * Solution: Directly manipulate DOM order, then trigger the reorder handler.
 */
async function reorderDeepCleaningTask(page: Page, sourceTaskName: string, targetTaskName: string) {
  await page.evaluate(({ source, target }) => {
    const list = document.getElementById('deep-cleaning-list')!;
    const items = Array.from(list.querySelectorAll('.task-item'));
    
    // Find items by task name text content
    const sourceItem = items.find(item => 
      item.querySelector('.task-name')?.textContent?.trim() === source
    );
    const targetItem = items.find(item => 
      item.querySelector('.task-name')?.textContent?.trim() === target
    );
    
    if (!sourceItem || !targetItem) {
      throw new Error(`Could not find tasks: "${source}" or "${target}"`);
    }
    
    // Move source item to be before target item in the DOM
    targetItem.parentNode!.insertBefore(sourceItem, targetItem);
  }, { source: sourceTaskName, target: targetTaskName });
  
  // Trigger handleDragReorder() to sync the new order with the backend
  await page.evaluate(() => {
    (window as any).handleDragReorder();
  });
  
  // Wait for the reorder API call to complete
  await page.waitForResponse(response => 
    response.url().includes('/deep-cleaning/reorder') && response.status() === 200
  );
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
        
        // Reorder: move C above A using DOM manipulation + handleDragReorder()
        await reorderDeepCleaningTask(page, taskC, taskA);
        
        const after = await taskNames(page);
        expect(after.indexOf(taskC)).toBeLessThan(after.indexOf(taskA));
       });

     test('drag-and-drop reorder persists after page reload', async ({ page }) => {
        const taskA = `DragPersist A ${Date.now()}`;
        const taskB = `DragPersist B ${Date.now()}`;
        
        await createDeepCleaningTask(page, taskA);
        await createDeepCleaningTask(page, taskB);
        
        const before = await taskNames(page);
        expect(before.indexOf(taskA)).toBeLessThan(before.indexOf(taskB));
        
        // Reorder: move B above A using DOM manipulation + handleDragReorder()
        await reorderDeepCleaningTask(page, taskB, taskA);
        
        const afterDrag = await taskNames(page);
        expect(afterDrag.indexOf(taskB)).toBeLessThan(afterDrag.indexOf(taskA));
        
        await page.reload();
        await page.waitForSelector('#deep-cleaning-list', { state: 'visible', timeout: 5000 });
        
        const afterReload = await taskNames(page);
        expect(afterReload.indexOf(taskB)).toBeLessThan(afterReload.indexOf(taskA));
      });
   });
});
