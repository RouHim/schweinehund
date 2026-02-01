import { test, expect } from '@playwright/test';

test.describe('PWA Installability', () => {
  test('manifest.json is accessible and valid', async ({ request }) => {
    const response = await request.get('/manifest.json');
    expect(response.status()).toBe(200);
    
    const manifest = await response.json();
    
    expect(manifest).toHaveProperty('name');
    expect(manifest).toHaveProperty('short_name');
    expect(manifest).toHaveProperty('start_url');
    expect(manifest).toHaveProperty('display');
    expect(manifest).toHaveProperty('theme_color');
    expect(manifest).toHaveProperty('background_color');
    expect(manifest).toHaveProperty('icons');
    
    expect(Array.isArray(manifest.icons)).toBe(true);
    expect(manifest.icons.length).toBeGreaterThan(0);
    
    const icon = manifest.icons[0];
    expect(icon).toHaveProperty('src');
    expect(icon).toHaveProperty('sizes');
    expect(icon).toHaveProperty('type');
  });

  test('service worker is accessible', async ({ request }) => {
    const response = await request.get('/sw.js');
    expect(response.status()).toBe(200);
    
    const contentType = response.headers()['content-type'];
    expect(contentType).toContain('javascript');
    
    const swContent = await response.text();
    expect(swContent.length).toBeGreaterThan(0);
  });

  test('PWA meta tags present in HTML', async ({ page }) => {
    await page.goto('/');
    
    const manifestLink = page.locator('link[rel="manifest"]');
    await expect(manifestLink).toHaveCount(1);
    const manifestHref = await manifestLink.getAttribute('href');
    expect(manifestHref).toContain('manifest.json');
    
    const themeColorMeta = page.locator('meta[name="theme-color"]');
    await expect(themeColorMeta).toHaveCount(1);
    
    const viewportMeta = page.locator('meta[name="viewport"]');
    await expect(viewportMeta).toHaveCount(1);
    
    const appleCapableMeta = page.locator('meta[name="apple-mobile-web-app-capable"]');
    await expect(appleCapableMeta).toHaveCount(1);
  });

  test('manifest icons are accessible', async ({ request }) => {
    const manifestResponse = await request.get('/manifest.json');
    const manifest = await manifestResponse.json();
    
    for (const icon of manifest.icons) {
      const iconResponse = await request.get(icon.src);
      expect(iconResponse.status()).toBe(200);
      
      const contentType = iconResponse.headers()['content-type'];
      expect(contentType).toContain('image');
    }
  });

  test('service worker registers successfully', async ({ page }) => {
    await page.goto('/');
    
    const swRegistered = await page.evaluate(async () => {
      if ('serviceWorker' in navigator) {
        try {
          const registration = await navigator.serviceWorker.register('/sw.js');
          return registration !== null;
        } catch (error) {
          return false;
        }
      }
      return false;
    });
    
    expect(swRegistered).toBe(true);
  });

  test('app is installable', async ({ page, context }) => {
    await context.grantPermissions(['notifications']);
    await page.goto('/');
    
    const hasManifest = await page.locator('link[rel="manifest"]').count();
    expect(hasManifest).toBe(1);
    
    const hasServiceWorker = await page.evaluate(() => {
      return 'serviceWorker' in navigator;
    });
    expect(hasServiceWorker).toBe(true);
  });
});
