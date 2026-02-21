import { test, expect } from './fixtures';

test.describe('Multiple notification times settings', () => {
  test.beforeEach(async ({ page, request }) => {
    const response = await request.post('http://localhost:9666/api/debug/reset-all');
    expect(response.ok()).toBeTruthy();

    const settingsResponse = await request.post('http://localhost:9666/api/settings', {
      data: {
        notification_enabled: false,
        notification_times: ['09:00'],
      },
    });
    expect(settingsResponse.ok()).toBeTruthy();

    await page.goto('/');
    await page.waitForSelector('#settings-form', { state: 'visible', timeout: 5000 });
  });

  test('default state shows one 09:00 row and add button', async ({ page }) => {
    const rows = page.locator('[data-testid="notification-time-row"]');
    await expect(rows).toHaveCount(1);

    const input = page.locator('[data-testid="notification-time-input"]').first();
    await expect(input).toHaveValue('09:00');

    await expect(page.locator('[data-testid="add-notification-time"]')).toBeVisible();
  });

  test('adds another notification time slot', async ({ page }) => {
    await page.locator('[data-testid="add-notification-time"]').click();
    await expect(page.locator('[data-testid="notification-time-row"]')).toHaveCount(2);
  });

  test('hides add button when maximum of three slots is reached', async ({ page }) => {
    const addButton = page.locator('[data-testid="add-notification-time"]');
    await addButton.click();
    await addButton.click();

    await expect(page.locator('[data-testid="notification-time-row"]')).toHaveCount(3);
    await expect(addButton).not.toBeVisible();
  });

  test('removes slot and shows add button again', async ({ page }) => {
    const addButton = page.locator('[data-testid="add-notification-time"]');
    await addButton.click();
    await expect(page.locator('[data-testid="notification-time-row"]')).toHaveCount(2);

    await page.locator('[data-testid="remove-notification-time"]').first().click();

    await expect(page.locator('[data-testid="notification-time-row"]')).toHaveCount(1);
    await expect(addButton).toBeVisible();
  });

  test('saves and persists two notification times after reload', async ({ page }) => {
    await page.locator('[data-testid="add-notification-time"]').click();

    const timeInputs = page.locator('[data-testid="notification-time-input"]');
    await timeInputs.nth(1).fill('14:00');

    page.once('dialog', (dialog) => {
      dialog.accept();
    });
    await page.locator('#settings-form button[type="submit"]').click();

    await page.reload();
    await page.waitForSelector('#settings-form', { state: 'visible', timeout: 5000 });

    const persistedInputs = page.locator('[data-testid="notification-time-input"]');
    await expect(persistedInputs).toHaveCount(2);
    await expect(persistedInputs.nth(0)).toHaveValue('09:00');
    await expect(persistedInputs.nth(1)).toHaveValue('14:00');
  });

  test('allows saving with empty notification time array', async ({ page }) => {
    await page.locator('[data-testid="remove-notification-time"]').first().click();
    await expect(page.locator('[data-testid="notification-time-row"]')).toHaveCount(0);

    page.once('dialog', (dialog) => {
      dialog.accept();
    });
    await page.locator('#settings-form button[type="submit"]').click();

    await expect(page.locator('#settings-form')).toBeVisible();
    await expect(page.locator('#settings-error')).toBeHidden();

    await page.locator('[data-testid="add-notification-time"]').click();
    await expect(page.locator('[data-testid="notification-time-row"]')).toHaveCount(1);
  });

  test('submits settings payload with notification_times array key', async ({ page }) => {
    await page.locator('[data-testid="add-notification-time"]').click();

    const requestPromise = page.waitForRequest((req) => {
      return req.url().includes('/api/settings') && req.method() === 'POST';
    });

    page.once('dialog', (dialog) => {
      dialog.accept();
    });
    await page.locator('#settings-form button[type="submit"]').click();

    const settingsRequest = await requestPromise;
    const body = settingsRequest.postDataJSON() as Record<string, unknown>;

    expect(Array.isArray(body.notification_times)).toBeTruthy();
    expect(body).not.toHaveProperty('notification_time');
  });
});
