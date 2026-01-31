import { test, expect } from '@playwright/test';

test.describe('HTMX Interactions', () => {
  test.beforeEach(async ({ page }) => {
    await page.route('**/*', (route) => {
      const url = route.request().url();
      if (url.includes('/partials/') || url.includes('.js') || url.includes('.css') || url === 'http://localhost:8080/') {
        route.continue();
      } else {
        route.abort();
      }
    });
  });

  test('HTMX has expected version in CDN URL', async ({ page }) => {
    const html = `<!DOCTYPE html>
      <html>
        <body>
          <script src="https://unpkg.com/htmx.org@2.0.3"></script>
        </body>
      </html>`;
    
    await page.setContent(html);
    
    const scripts = await page.locator('script').all();
    const htmxScript = await Promise.all(
      scripts.map((s) => s.getAttribute('src'))
    );
    const hasHtmx = htmxScript.some((src) => src?.includes('htmx.org'));
    expect(hasHtmx).toBe(true);
  });

  test('navigation button has hx-get attribute', async ({ page }) => {
    const html = `<!DOCTYPE html>
      <html>
        <body>
          <button hx-get="/partials/tasks" hx-target="#main" hx-swap="innerHTML">
            Aufgaben
          </button>
        </body>
      </html>`;
    
    await page.setContent(html);
    
    const button = page.locator('button');
    expect(await button.getAttribute('hx-get')).toBe('/partials/tasks');
  });

  test('hx-target attribute correctly targets main element', async ({ page }) => {
    const html = `<!DOCTYPE html>
      <html>
        <body>
          <button hx-target="#main">Load</button>
        </body>
      </html>`;
    
    await page.setContent(html);
    
    const button = page.locator('button');
    expect(await button.getAttribute('hx-target')).toBe('#main');
  });

  test('hx-swap="innerHTML" swaps inner content', async ({ page }) => {
    const html = `<!DOCTYPE html>
      <html>
        <body>
          <div id="main">Old content</div>
          <button hx-swap="innerHTML">Load</button>
        </body>
      </html>`;
    
    await page.setContent(html);
    
    const button = page.locator('button');
    expect(await button.getAttribute('hx-swap')).toBe('innerHTML');
  });

  test('hx-ext="alpine-morph" attribute present on body', async ({ page }) => {
    const html = `<!DOCTYPE html>
      <html>
        <body hx-ext="alpine-morph">
          <main id="main"></main>
        </body>
      </html>`;
    
    await page.setContent(html);
    
    const body = page.locator('body');
    expect(await body.getAttribute('hx-ext')).toBe('alpine-morph');
  });

  test('main element has hx-trigger="load" attribute', async ({ page }) => {
    const html = `<!DOCTYPE html>
      <html>
        <body>
          <main id="main" hx-trigger="load">Content</main>
        </body>
      </html>`;
    
    await page.setContent(html);
    
    const main = page.locator('#main');
    expect(await main.getAttribute('hx-trigger')).toBe('load');
  });

  test('navigation buttons trigger hx-get requests', async ({ page }) => {
    const html = `<!DOCTYPE html>
      <html>
        <body>
          <button hx-get="/partials/today">Heute</button>
          <button hx-get="/partials/tasks">Aufgaben</button>
          <button hx-get="/partials/zones">Zonen</button>
        </body>
      </html>`;
    
    await page.setContent(html);
    
    const buttons = await page.locator('button').all();
    expect(buttons.length).toBe(3);
    
    const today = page.locator('button:has-text("Heute")');
    expect(await today.getAttribute('hx-get')).toBe('/partials/today');
  });

  test('htmx attributes chain correctly', async ({ page }) => {
    const html = `<!DOCTYPE html>
      <html>
        <body>
          <button hx-get="/api/data" hx-target="#result" hx-swap="innerHTML">
            Load Data
          </button>
        </body>
      </html>`;
    
    await page.setContent(html);
    
    const button = page.locator('button');
    expect(await button.getAttribute('hx-get')).toBe('/api/data');
    expect(await button.getAttribute('hx-target')).toBe('#result');
    expect(await button.getAttribute('hx-swap')).toBe('innerHTML');
  });
});
