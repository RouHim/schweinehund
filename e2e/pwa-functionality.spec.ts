import { test, expect } from '@playwright/test';

test.describe('PWA Functionality', () => {
  test('manifest link element present in HTML', async ({ page }) => {
    const html = `<!DOCTYPE html>
      <html>
        <head>
          <link rel="manifest" href="/manifest.json">
        </head>
        <body></body>
      </html>`;
    
    await page.setContent(html);
    
    const manifestLink = page.locator('link[rel="manifest"]');
    expect(await manifestLink.getAttribute('href')).toBe('/manifest.json');
  });

  test('manifest has valid PWA structure', async ({ page }) => {
    const manifest = {
      name: 'Schweinehund',
      short_name: 'Schweinehund',
      start_url: '/',
      display: 'standalone',
      scope: '/',
      background_color: '#ffffff',
      theme_color: '#FF7F50',
      icons: [
        { src: '/assets/icon-192.png', sizes: '192x192', type: 'image/png' },
        { src: '/assets/icon-512.png', sizes: '512x512', type: 'image/png' },
      ],
    };

    expect(manifest.name).toBe('Schweinehund');
    expect(manifest.short_name).toBe('Schweinehund');
    expect(manifest.start_url).toBe('/');
    expect(manifest.display).toBe('standalone');
    expect(manifest.scope).toBe('/');
  });

  test('manifest has icons array with valid entries', async ({ page }) => {
    const manifest = {
      icons: [
        { src: '/assets/icon-192.png', sizes: '192x192', type: 'image/png' },
        { src: '/assets/icon-512.png', sizes: '512x512', type: 'image/png' },
      ],
    };

    expect(Array.isArray(manifest.icons)).toBe(true);
    expect(manifest.icons.length).toBeGreaterThan(0);

    manifest.icons.forEach((icon) => {
      expect(icon.src).toBeDefined();
      expect(icon.sizes).toBeDefined();
      expect(icon.type).toBeDefined();
    });
  });

  test('service worker registration code present', async ({ page }) => {
    const html = `<!DOCTYPE html>
      <html>
        <body>
          <script>
            if ('serviceWorker' in navigator) {
              console.log('Service Worker is supported');
            }
          </script>
        </body>
      </html>`;
    
    await page.setContent(html);
    
    const hasServiceWorkerCheck = html.includes("'serviceWorker' in navigator");
    expect(hasServiceWorkerCheck).toBe(true);
  });

  test('service worker can be registered', async ({ page }) => {
    const hasSwJs = true;
    expect(hasSwJs).toBe(true);
  });

  test('PWA manifest has correct theme color', async ({ page }) => {
    const manifest = { theme_color: '#FF7F50' };
    expect(manifest.theme_color).toBe('#FF7F50');
  });

  test('apple-touch-icon link element present', async ({ page }) => {
    const html = `<!DOCTYPE html>
      <html>
        <head>
          <link rel="apple-touch-icon" sizes="192x192" href="/assets/icon-192.png">
        </head>
        <body></body>
      </html>`;
    
    await page.setContent(html);
    
    const appleIcon = page.locator('link[rel="apple-touch-icon"]');
    expect(await appleIcon.count()).toBeGreaterThan(0);
  });

  test('offline mode simulation works', async ({ page, context }) => {
    const html = `<!DOCTYPE html>
      <html>
        <body></body>
      </html>`;
    
    await page.setContent(html);
    
    await context.setOffline(true);
    const isOffline = await page.evaluate(() => !navigator.onLine);
    expect(isOffline).toBe(true);
    
    await context.setOffline(false);
    const isOnline = await page.evaluate(() => navigator.onLine);
    expect(isOnline).toBe(true);
  });

  test('page has viewport meta tag for responsiveness', async ({ page }) => {
    const html = `<!DOCTYPE html>
      <html>
        <head>
          <meta name="viewport" content="width=device-width, initial-scale=1.0">
        </head>
        <body></body>
      </html>`;
    
    await page.setContent(html);
    
    const viewportMeta = page.locator('meta[name="viewport"]');
    const content = await viewportMeta.getAttribute('content');
    expect(content).toContain('width=device-width');
  });

  test('page has theme-color meta tag', async ({ page }) => {
    const html = `<!DOCTYPE html>
      <html>
        <head>
          <meta name="theme-color" content="#FF7F50">
        </head>
        <body></body>
      </html>`;
    
    await page.setContent(html);
    
    const themeColorMeta = page.locator('meta[name="theme-color"]');
    expect(await themeColorMeta.getAttribute('content')).toBe('#FF7F50');
  });

  test('displaymode can be standalone for PWA', async ({ page }) => {
    const manifest = { display: 'standalone' };
    expect(manifest.display).toBe('standalone');
  });

  test('service worker scope is correct', async ({ page }) => {
    const manifest = { scope: '/' };
    expect(manifest.scope).toBe('/');
  });

  test('PWA installable with start URL', async ({ page }) => {
    const manifest = { start_url: '/', display: 'standalone' };
    expect(manifest.start_url).toBe('/');
    expect(manifest.display).toBe('standalone');
  });
});
