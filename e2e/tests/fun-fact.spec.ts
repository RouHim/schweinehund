import { Page } from '@playwright/test';
import { test, expect } from './fixtures';

async function completeAllDailyTasks(page: Page) {
  const uncheckedCheckboxes = page.locator('#tasks-list .task-checkbox:not(:checked)');
  const doneCounter = page.locator('#tasks-done');
  const totalCounter = page.locator('#tasks-total');

  const totalTasks = Number((await totalCounter.textContent()) || '0');
  if (totalTasks <= 0) {
    return;
  }

  let doneTasks = Number((await doneCounter.textContent()) || '0');
  let guard = 0;

  while (doneTasks < totalTasks) {
    if ((await uncheckedCheckboxes.count()) === 0) {
      break;
    }

    await uncheckedCheckboxes.first().check();
    await expect.poll(async () => Number((await doneCounter.textContent()) || '0')).toBeGreaterThan(doneTasks);
    doneTasks = Number((await doneCounter.textContent()) || '0');
    guard += 1;

    if (guard > 200) {
      throw new Error('Could not complete all daily tasks within guard limit');
    }
  }
}

test.describe('Fun Fact Popup', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('#tasks-list', { state: 'visible' });
    await page.evaluate(() => fetch('/api/debug/reset-all', { method: 'POST' }));
    await page.reload();
    await page.waitForSelector('#tasks-list', { state: 'visible' });
  });

  test.afterEach(async ({ page }) => {
    await page.evaluate(() => fetch('/api/debug/reset-all', { method: 'POST' }));
  });

  test('shows fun-fact popup after completing all daily tasks', async ({ page }) => {
    const checkboxes = page.locator('#tasks-list .task-checkbox');
    const count = await checkboxes.count();

    expect(count).toBeGreaterThan(0);

    for (let i = 0; i < count - 1; i++) {
      await checkboxes.nth(i).check();
    }

    await expect(page.locator('[data-testid="fun-fact-modal"]')).not.toBeVisible();

    await completeAllDailyTasks(page);

    await expect(page.locator('[data-testid="fun-fact-modal"]')).toBeVisible();
    await expect(page.locator('[data-testid="fun-fact-text"]')).not.toBeEmpty();
  });

  test('fun-fact popup auto-closes after 15 seconds', async ({ page }) => {
    test.setTimeout(60_000);

    await completeAllDailyTasks(page);

    await expect(page.locator('[data-testid="fun-fact-modal"]')).toBeVisible();

    await page.waitForTimeout(15500);

    await expect(page.locator('[data-testid="fun-fact-modal"]')).not.toBeVisible();
  });

  test('fun-fact popup can be closed manually', async ({ page }) => {
    await completeAllDailyTasks(page);

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

  test('API failure shows fallback message', async ({ page }) => {
    await page.route('https://v2.jokeapi.dev/joke/Any*', (route) => {
      route.abort('failed');
    });

    await completeAllDailyTasks(page);

    await expect(page.locator('[data-testid="fun-fact-modal"]')).toBeVisible();
    await expect(page.locator('[data-testid="fun-fact-text"]')).toHaveText('Gut gemacht! 🎉');
  });

  test('displays German joke content from JokeAPI', async ({ page }) => {
    await completeAllDailyTasks(page);

    await expect(page.locator('[data-testid="fun-fact-modal"]')).toBeVisible();
    
    const jokeText = await page.locator('[data-testid="fun-fact-text"]').textContent();
    expect(jokeText).toBeTruthy();
    expect(jokeText!.length).toBeGreaterThan(10);
  });

  test('clicking backdrop does not close fun-fact modal', async ({ page }) => {
    await completeAllDailyTasks(page);

    await expect(page.locator('[data-testid="fun-fact-modal"]')).toBeVisible();

    await page.locator('[data-testid="fun-fact-modal"]').click({ position: { x: 10, y: 10 } });

    await expect(page.locator('[data-testid="fun-fact-modal"]')).toBeVisible();
  });

  test('unchecking last task does not trigger popup', async ({ page }) => {
    await completeAllDailyTasks(page);

    const checkboxes = page.locator('#tasks-list .task-checkbox');

    await expect(page.locator('[data-testid="fun-fact-modal"]')).toBeVisible();
    await page.locator('[data-testid="fun-fact-close-btn"]').click();

    await checkboxes.first().uncheck();
    await page.waitForTimeout(1000);

    await expect(page.locator('[data-testid="fun-fact-modal"]')).not.toBeVisible();
  });
});
