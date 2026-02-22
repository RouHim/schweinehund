import { test, expect } from './fixtures';
import * as fs from 'fs';
import * as os from 'os';
import * as path from 'path';

test.describe('Export/Import', () => {
  test.beforeEach(async ({ page }) => {
    const response = await page.request.post('/api/debug/reset-all');
    expect(response.ok()).toBeTruthy();

    await page.goto('/');
    // Wait for both the task list and the settings form to be visible (settings loads async)
    await page.waitForSelector('#tasks-list', { state: 'visible', timeout: 10000 });
    await page.waitForSelector('#settings-form', { state: 'visible', timeout: 10000 });
  });

  test('import and export buttons are visible in settings', async ({ page }) => {
    const exportBtn = page.locator('[data-testid="export-btn"]');
    const importBtn = page.locator('[data-testid="import-btn"]');
    await exportBtn.scrollIntoViewIfNeeded();
    await expect(exportBtn).toBeVisible();
    await expect(importBtn).toBeVisible();
  });

  test('export button triggers JSON file download', async ({ page }) => {
    await page.locator('[data-testid="export-btn"]').scrollIntoViewIfNeeded();

    const downloadPromise = page.waitForEvent('download');
    await page.locator('[data-testid="export-btn"]').click();
    const download = await downloadPromise;

    expect(download.suggestedFilename()).toMatch(/schweinehund-export-\d{4}-\d{2}-\d{2}\.json/);

    const exportFilePath = path.join(os.tmpdir(), `${Date.now()}-${download.suggestedFilename()}`);
    await download.saveAs(exportFilePath);
    const content = fs.readFileSync(exportFilePath, 'utf-8');
    const data = JSON.parse(content) as { daily_tasks: unknown[]; deep_cleaning_tasks: unknown[] };
    expect(data).toHaveProperty('daily_tasks');
    expect(data).toHaveProperty('deep_cleaning_tasks');
    expect(Array.isArray(data.daily_tasks)).toBe(true);
    expect(Array.isArray(data.deep_cleaning_tasks)).toBe(true);
  });

  test('import via API replaces all tasks', async ({ page }) => {
    const exportResponse = await page.request.get('/api/tasks/all');
    const originalData = await exportResponse.json() as {
      daily_tasks: Array<{ name: string }>;
      deep_cleaning_tasks: unknown[];
    };
    const originalDailyCount = originalData.daily_tasks.length;
    expect(originalDailyCount).toBeGreaterThan(0);

    // Add a canary task that should be gone after import
    const createResponse = await page.request.post('/api/tasks', {
      data: { name: 'CANARY_TASK_DELETE_ME', day_of_week: -1 },
    });
    expect(createResponse.ok()).toBeTruthy();

    // Import original data (without canary)
    const importResponse = await page.request.post('/api/import', {
      data: originalData,
    });
    expect(importResponse.ok()).toBeTruthy();

    // Verify canary is gone and count matches original
    const afterImport = await page.request.get('/api/tasks/all');
    const afterData = await afterImport.json() as {
      daily_tasks: Array<{ name: string }>;
      deep_cleaning_tasks: unknown[];
    };
    expect(afterData.daily_tasks.length).toBe(originalDailyCount);

    const canary = afterData.daily_tasks.find((t: { name: string }) => t.name === 'CANARY_TASK_DELETE_ME');
    expect(canary).toBeUndefined();
  });

  test('import API rejects invalid day_of_week', async ({ page }) => {
    // Send a fully-formed task (all required fields) but with an invalid day_of_week
    // This ensures serde deserialization succeeds, then our validation rejects it with 400
    const response = await page.request.post('/api/import', {
      data: {
        daily_tasks: [{
          id: 0,
          name: 'Bad Task',
          description: null,
          zone: null,
          day_of_week: 99,
          completed: false,
          completed_at: null,
          interval_weeks: 1,
          start_date: null,
        }],
        deep_cleaning_tasks: [],
      },
    });
    expect(response.ok()).toBeFalsy();
    expect(response.status()).toBe(400);
  });

  test('import via UI shows file chooser', async ({ page }) => {
    page.on('dialog', dialog => dialog.dismiss());

    await page.locator('[data-testid="import-btn"]').scrollIntoViewIfNeeded();
    const fileChooserPromise = page.waitForEvent('filechooser');
    await page.locator('[data-testid="import-btn"]').click();
    const fileChooser = await fileChooserPromise;
    expect(fileChooser).toBeTruthy();
  });
});
