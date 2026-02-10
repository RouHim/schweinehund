import { test, expect } from './fixtures';

test.describe('Desktop Layout - Two Column Grid', () => {
  test('displays side-by-side layout at 1024px viewport', async ({ page }) => {
    await page.setViewportSize({ width: 1024, height: 768 });
    await page.goto('/');
    
    await page.waitForSelector('#tasks-list', { state: 'visible', timeout: 5000 });
    await page.waitForSelector('#deep-cleaning-list', { state: 'visible', timeout: 5000 });
    
    const todaySection = page.locator('#today-section');
    const deepCleaningSection = page.locator('#deep-cleaning-section');
    
    const todayBox = await todaySection.boundingBox();
    const deepBox = await deepCleaningSection.boundingBox();
    
    expect(todayBox).not.toBeNull();
    expect(deepBox).not.toBeNull();
    
    if (!todayBox || !deepBox) {
      throw new Error('Could not get bounding boxes');
    }
    
    const todayRight = todayBox.x + todayBox.width;
    const deepLeft = deepBox.x;
    
    expect(todayRight).toBeLessThanOrEqual(deepLeft + 50);
    
    const yDifference = Math.abs(todayBox.y - deepBox.y);
    expect(yDifference).toBeLessThan(100);
  });

  test('maintains readable column widths at 1920px viewport', async ({ page }) => {
    await page.setViewportSize({ width: 1920, height: 1080 });
    await page.goto('/');
    
    await page.waitForSelector('#tasks-list', { state: 'visible', timeout: 5000 });
    
    const todaySection = page.locator('#today-section');
    const todayBox = await todaySection.boundingBox();
    
    expect(todayBox).not.toBeNull();
    
    if (!todayBox) {
      throw new Error('Could not get bounding box');
    }
    
    expect(todayBox.width).toBeLessThanOrEqual(700);
    expect(todayBox.width).toBeGreaterThan(300);
  });

  test('displays side-by-side layout at 1280px viewport', async ({ page }) => {
    await page.setViewportSize({ width: 1280, height: 800 });
    await page.goto('/');
    
    await page.waitForSelector('#tasks-list', { state: 'visible', timeout: 5000 });
    await page.waitForSelector('#deep-cleaning-list', { state: 'visible', timeout: 5000 });
    
    const todaySection = page.locator('#today-section');
    const deepCleaningSection = page.locator('#deep-cleaning-section');
    
    const todayBox = await todaySection.boundingBox();
    const deepBox = await deepCleaningSection.boundingBox();
    
    expect(todayBox).not.toBeNull();
    expect(deepBox).not.toBeNull();
    
    if (!todayBox || !deepBox) {
      throw new Error('Could not get bounding boxes');
    }
    
    const todayRight = todayBox.x + todayBox.width;
    const deepLeft = deepBox.x;
    
    expect(todayRight).toBeLessThanOrEqual(deepLeft + 50);
  });
});

test.describe('Mobile Layout - Single Column Stack', () => {
  test('displays stacked layout at 768px viewport', async ({ page }) => {
    await page.setViewportSize({ width: 768, height: 1024 });
    await page.goto('/');
    
    await page.waitForSelector('#tasks-list', { state: 'visible', timeout: 5000 });
    await page.waitForSelector('#deep-cleaning-list', { state: 'visible', timeout: 5000 });
    
    const todaySection = page.locator('#today-section');
    const deepCleaningSection = page.locator('#deep-cleaning-section');
    
    const todayBox = await todaySection.boundingBox();
    const deepBox = await deepCleaningSection.boundingBox();
    
    expect(todayBox).not.toBeNull();
    expect(deepBox).not.toBeNull();
    
    if (!todayBox || !deepBox) {
      throw new Error('Could not get bounding boxes');
    }
    
    const xDifference = Math.abs(todayBox.x - deepBox.x);
    expect(xDifference).toBeLessThan(50);
    
    const todayBottom = todayBox.y + todayBox.height;
    expect(deepBox.y).toBeGreaterThan(todayBottom - 50);
  });

  test('displays stacked layout at 375px mobile viewport', async ({ page }) => {
    await page.setViewportSize({ width: 375, height: 667 });
    await page.goto('/');
    
    await page.waitForSelector('#tasks-list', { state: 'visible', timeout: 5000 });
    await page.waitForSelector('#deep-cleaning-list', { state: 'visible', timeout: 5000 });
    
    const todaySection = page.locator('#today-section');
    const deepCleaningSection = page.locator('#deep-cleaning-section');
    
    const todayBox = await todaySection.boundingBox();
    const deepBox = await deepCleaningSection.boundingBox();
    
    expect(todayBox).not.toBeNull();
    expect(deepBox).not.toBeNull();
    
    if (!todayBox || !deepBox) {
      throw new Error('Could not get bounding boxes');
    }
    
    const xDifference = Math.abs(todayBox.x - deepBox.x);
    expect(xDifference).toBeLessThan(50);
    
    const todayBottom = todayBox.y + todayBox.height;
    expect(deepBox.y).toBeGreaterThan(todayBottom - 50);
  });

  test('displays stacked layout at 1000px viewport (below breakpoint)', async ({ page }) => {
    await page.setViewportSize({ width: 1000, height: 800 });
    await page.goto('/');
    
    await page.waitForSelector('#tasks-list', { state: 'visible', timeout: 5000 });
    await page.waitForSelector('#deep-cleaning-list', { state: 'visible', timeout: 5000 });
    
    const todaySection = page.locator('#today-section');
    const deepCleaningSection = page.locator('#deep-cleaning-section');
    
    const todayBox = await todaySection.boundingBox();
    const deepBox = await deepCleaningSection.boundingBox();
    
    expect(todayBox).not.toBeNull();
    expect(deepBox).not.toBeNull();
    
    if (!todayBox || !deepBox) {
      throw new Error('Could not get bounding boxes');
    }
    
    const xDifference = Math.abs(todayBox.x - deepBox.x);
    expect(xDifference).toBeLessThan(50);
    
    const todayBottom = todayBox.y + todayBox.height;
    expect(deepBox.y).toBeGreaterThan(todayBottom - 50);
  });
});

test.describe('Settings Section Layout', () => {
  test('settings section spans full width on desktop', async ({ page }) => {
    await page.setViewportSize({ width: 1280, height: 800 });
    await page.goto('/');
    
    await page.waitForSelector('#settings-form', { state: 'visible', timeout: 5000 });
    
    const container = page.locator('main.container');
    const settingsSection = page.locator('#settings-section');
    
    const containerBox = await container.boundingBox();
    const settingsBox = await settingsSection.boundingBox();
    
    expect(containerBox).not.toBeNull();
    expect(settingsBox).not.toBeNull();
    
    if (!containerBox || !settingsBox) {
      throw new Error('Could not get bounding boxes');
    }
    
    const widthDifference = Math.abs(containerBox.width - settingsBox.width);
    expect(widthDifference).toBeLessThan(100);
  });

  test('settings section appears below task sections', async ({ page }) => {
    await page.setViewportSize({ width: 1280, height: 800 });
    await page.goto('/');
    
    await page.waitForSelector('#settings-form', { state: 'visible', timeout: 5000 });
    
    const todaySection = page.locator('#today-section');
    const deepCleaningSection = page.locator('#deep-cleaning-section');
    const settingsSection = page.locator('#settings-section');
    
    const todayBox = await todaySection.boundingBox();
    const deepBox = await deepCleaningSection.boundingBox();
    const settingsBox = await settingsSection.boundingBox();
    
    expect(todayBox).not.toBeNull();
    expect(deepBox).not.toBeNull();
    expect(settingsBox).not.toBeNull();
    
    if (!todayBox || !deepBox || !settingsBox) {
      throw new Error('Could not get bounding boxes');
    }
    
    const taskSectionsBottom = Math.max(
      todayBox.y + todayBox.height,
      deepBox.y + deepBox.height
    );
    
    expect(settingsBox.y).toBeGreaterThan(taskSectionsBottom - 50);
  });
});
