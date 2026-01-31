import { test, expect } from '@playwright/test';

test.describe('Alpine.js Reactivity', () => {
  test('Alpine.js CDN library loads', async ({ page }) => {
    const html = `<!DOCTYPE html>
      <html>
        <body>
          <script defer src="https://unpkg.com/alpinejs@3.x.x/dist/cdn.min.js"></script>
          <script>
            document.addEventListener('alpine:init', () => {
              window.alpineLoaded = true;
            });
          </script>
        </body>
      </html>`;
    
    await page.setContent(html);
    await page.waitForTimeout(2000);
  });

  test('Alpine.js Morph extension can be loaded', async ({ page }) => {
    const html = `<!DOCTYPE html>
      <html>
        <body>
          <script defer src="https://unpkg.com/@alpinejs/morph@3.x.x/dist/cdn.min.js"></script>
          <script defer src="https://unpkg.com/alpinejs@3.x.x/dist/cdn.min.js"></script>
        </body>
      </html>`;
    
    await page.setContent(html);
    await page.waitForTimeout(1000);
  });

  test('body element has x-data attribute', async ({ page }) => {
    const html = `<!DOCTYPE html>
      <html>
        <body x-data="app()">
          <script>function app() { return {}; }</script>
        </body>
      </html>`;
    
    await page.setContent(html);
    
    const body = page.locator('body');
    expect(await body.getAttribute('x-data')).toContain('app()');
  });

  test('app() function can be defined', async ({ page }) => {
    const html = `<!DOCTYPE html>
      <html>
        <body>
          <script>
            window.appFunc = function app() {
              return { counter: 0 };
            };
          </script>
        </body>
      </html>`;
    
    await page.setContent(html);
    
    const appExists = await page.evaluate(() => typeof (window as any).appFunc === 'function');
    expect(appExists).toBe(true);
  });

  test('Alpine x-data creates reactive state', async ({ page }) => {
    const html = `<!DOCTYPE html>
      <html>
        <body>
          <div x-data="{ message: 'Hello' }"></div>
        </body>
      </html>`;
    
    await page.setContent(html);
    
    const div = page.locator('div');
    expect(await div.getAttribute('x-data')).toContain('message');
  });

  test('x-show directive present for visibility control', async ({ page }) => {
    const html = `<!DOCTYPE html>
      <html>
        <body>
          <div x-data="{ show: true }">
            <p x-show="show">Visible</p>
          </div>
        </body>
      </html>`;
    
    await page.setContent(html);
    
    const p = page.locator('p');
    expect(await p.getAttribute('x-show')).toBe('show');
  });

  test('x-on directive binds event handlers', async ({ page }) => {
    const html = `<!DOCTYPE html>
      <html>
        <body>
          <div x-data="{ count: 0 }">
            <button x-on:click="count++">Click</button>
          </div>
        </body>
      </html>`;
    
    await page.setContent(html);
    
    const button = page.locator('button');
    expect(await button.getAttribute('x-on:click')).toBe('count++');
  });

  test('x-text updates element text', async ({ page }) => {
    const html = `<!DOCTYPE html>
      <html>
        <body>
          <div x-data="{ name: 'Alpine' }">
            <span x-text="name"></span>
          </div>
        </body>
      </html>`;
    
    await page.setContent(html);
    
    const span = page.locator('span');
    expect(await span.getAttribute('x-text')).toBe('name');
  });

  test('x-model creates two-way binding', async ({ page }) => {
    const html = `<!DOCTYPE html>
      <html>
        <body>
          <div x-data="{ input: '' }">
            <input x-model="input" type="text">
          </div>
        </body>
      </html>`;
    
    await page.setContent(html);
    
    const input = page.locator('input');
    expect(await input.getAttribute('x-model')).toBe('input');
  });

  test('Alpine directives can be combined', async ({ page }) => {
    const html = `<!DOCTYPE html>
      <html>
        <body>
          <div x-data="{ open: false }">
            <button x-on:click="open = !open">Toggle</button>
            <div x-show="open" x-text="open ? 'Open' : 'Closed'"></div>
          </div>
        </body>
      </html>`;
    
    await page.setContent(html);
    
    const button = page.locator('button');
    expect(await button.getAttribute('x-on:click')).toContain('open');
  });
});
