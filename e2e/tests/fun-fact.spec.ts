import { test, expect } from '@playwright/test';

test.describe('Fun Fact Popup', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('#tasks-list', { state: 'visible' });
  });

  test.afterEach(async ({ page }) => {
    await page.evaluate(() => fetch('/api/debug/reset', { method: 'POST' }));
  });

  test('shows fun-fact popup after completing all daily tasks', async ({ page }) => {
    const checkboxes = page.locator('#tasks-list .task-checkbox');
    const count = await checkboxes.count();

    expect(count).toBeGreaterThan(0);

    for (let i = 0; i < count - 1; i++) {
      await checkboxes.nth(i).check();
    }

    await expect(page.locator('[data-testid="fun-fact-modal"]')).not.toBeVisible();

    await checkboxes.nth(count - 1).check();

    await expect(page.locator('[data-testid="fun-fact-modal"]')).toBeVisible();
    await expect(page.locator('[data-testid="fun-fact-text"]')).not.toBeEmpty();
  });

  test('fun-fact popup auto-closes after 15 seconds', async ({ page }) => {
    const checkboxes = page.locator('#tasks-list .task-checkbox');
    const count = await checkboxes.count();

    for (let i = 0; i < count; i++) {
      await checkboxes.nth(i).check();
    }

    await expect(page.locator('[data-testid="fun-fact-modal"]')).toBeVisible();

    await page.waitForTimeout(15500);

    await expect(page.locator('[data-testid="fun-fact-modal"]')).not.toBeVisible();
  });

  test('fun-fact popup can be closed manually', async ({ page }) => {
    const checkboxes = page.locator('#tasks-list .task-checkbox');
    const count = await checkboxes.count();

    for (let i = 0; i < count; i++) {
      await checkboxes.nth(i).check();
    }

    await expect(page.locator('[data-testid="fun-fact-modal"]')).toBeVisible();

    await page.locator('[data-testid="fun-fact-close-btn"]').click();

    await expect(page.locator('[data-testid="fun-fact-modal"]')).not.toBeVisible();
  });

  test('no popup when some daily tasks remain uncompleted', async ({ page }) => {
    const checkboxes = page.locator('#tasks-list .task-checkbox');
    const count = await checkboxes.count();

    expect(count).toBeGreaterThan(1);

    await checkboxes.first().check();
    await page.waitForTimeout(1000);

    await expect(page.locator('[data-testid="fun-fact-modal"]')).not.toBeVisible();
  });

  test('API failure results in no popup (silent fail)', async ({ page }) => {
    await page.route('https://v2.jokeapi.dev/joke/Any*', (route) => {
      route.abort('failed');
    });

    const checkboxes = page.locator('#tasks-list .task-checkbox');
    const count = await checkboxes.count();

    for (let i = 0; i < count; i++) {
      await checkboxes.nth(i).check();
    }

    await page.waitForTimeout(2000);

    await expect(page.locator('[data-testid="fun-fact-modal"]')).not.toBeVisible();
  });

  test('displays German joke content from JokeAPI', async ({ page }) => {
    const checkboxes = page.locator('#tasks-list .task-checkbox');
    const count = await checkboxes.count();

    for (let i = 0; i < count; i++) {
      await checkboxes.nth(i).check();
    }

    await expect(page.locator('[data-testid="fun-fact-modal"]')).toBeVisible();
    
    const jokeText = await page.locator('[data-testid="fun-fact-text"]').textContent();
    expect(jokeText).toBeTruthy();
    expect(jokeText!.length).toBeGreaterThan(10);
  });

  test('clicking backdrop does not close fun-fact modal', async ({ page }) => {
    const checkboxes = page.locator('#tasks-list .task-checkbox');
    const count = await checkboxes.count();

    for (let i = 0; i < count; i++) {
      await checkboxes.nth(i).check();
    }

    await expect(page.locator('[data-testid="fun-fact-modal"]')).toBeVisible();

    await page.locator('[data-testid="fun-fact-modal"]').click({ position: { x: 10, y: 10 } });

    await expect(page.locator('[data-testid="fun-fact-modal"]')).toBeVisible();
  });

  test('unchecking last task does not trigger popup', async ({ page }) => {
    const checkboxes = page.locator('#tasks-list .task-checkbox');
    const count = await checkboxes.count();

    for (let i = 0; i < count; i++) {
      await checkboxes.nth(i).check();
    }

    await expect(page.locator('[data-testid="fun-fact-modal"]')).toBeVisible();
    await page.locator('[data-testid="fun-fact-close-btn"]').click();

    await checkboxes.first().uncheck();
    await page.waitForTimeout(1000);

    await expect(page.locator('[data-testid="fun-fact-modal"]')).not.toBeVisible();
  });
});
