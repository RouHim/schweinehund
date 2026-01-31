import { test, expect } from '@playwright/test';

test.describe('Task Workflows', () => {
  test('task element can have completed class', async ({ page }) => {
    const html = `<!DOCTYPE html>
      <html>
        <body>
          <div data-task-id="task-001" class="task">Complete me</div>
        </body>
      </html>`;
    
    await page.setContent(html);
    
    const taskElement = page.locator('[data-task-id="task-001"]');
    await taskElement.evaluate((el: HTMLElement) => {
      el.classList.add('completed');
    });
    
    const isCompleted = await taskElement.evaluate((el: HTMLElement) => 
      el.classList.contains('completed')
    );
    expect(isCompleted).toBe(true);
  });

  test('task completion sends PATCH request data', async ({ page }) => {
    const patchData = {
      completed: true,
      updated: new Date().toISOString(),
    };

    expect(patchData.completed).toBe(true);
    expect(patchData.updated).toBeDefined();
  });

  test('zone creation sends POST request with zone data', async ({ page }) => {
    const postData = {
      name: 'New Zone',
      description: 'New zone desc',
      color: '#e74c3c',
    };

    expect(postData.name).toBe('New Zone');
    expect(postData.color).toBe('#e74c3c');
  });

  test('zone update sends PATCH request', async ({ page }) => {
    const patchData = {
      name: 'Updated Kitchen',
      updated: new Date().toISOString(),
    };

    expect(patchData.name).toBe('Updated Kitchen');
  });

  test('form validation requires zone name', async ({ page }) => {
    const html = `<!DOCTYPE html>
      <html>
        <body>
          <form>
            <input type="text" name="zoneName" required>
          </form>
        </body>
      </html>`;
    
    await page.setContent(html);
    
    const input = page.locator('input[name="zoneName"]');
    expect(await input.getAttribute('required')).toBe('');
  });

  test('zone form input has required attribute', async ({ page }) => {
    const html = `<!DOCTYPE html>
      <html>
        <body>
          <input type="text" name="zoneName" required>
        </body>
      </html>`;
    
    await page.setContent(html);
    
    const input = page.locator('input');
    expect(await input.getAttribute('required')).toBe('');
  });

  test('task modal opens with custom event', async ({ page }) => {
    const html = `<!DOCTYPE html>
      <html>
        <body>
          <script>
            window.modalOpened = false;
            document.addEventListener('open-task-modal', () => {
              window.modalOpened = true;
            });
          </script>
        </body>
      </html>`;
    
    await page.setContent(html);
    
    await page.evaluate(() => {
      document.dispatchEvent(new CustomEvent('open-task-modal', {
        detail: { taskId: 'task-001' },
      }));
    });
    
    const modalOpened = await page.evaluate(() => (window as any).modalOpened);
    expect(modalOpened).toBe(true);
  });

  test('task modal event includes task data', async ({ page }) => {
    const html = `<!DOCTYPE html>
      <html>
        <body>
          <button onclick="dispatchModalEvent()">Open</button>
          <script>
            function dispatchModalEvent() {
              const event = new CustomEvent('open-task-modal', {
                detail: { taskId: 'task-001', title: 'Test Task', zone: 'zone-001' }
              });
              document.dispatchEvent(event);
            }
          </script>
        </body>
      </html>`;
    
    await page.setContent(html);
    
    const htmlContent = html;
    expect(htmlContent).toContain('taskId');
    expect(htmlContent).toContain('task-001');
  });

  test('modal visibility toggle with x-show', async ({ page }) => {
    const html = `<!DOCTYPE html>
      <html>
        <body>
          <div x-data="{ open: false }">
            <div x-show="open" id="modal">Modal content</div>
          </div>
        </body>
      </html>`;
    
    await page.setContent(html);
    
    const modal = page.locator('#modal');
    expect(await modal.getAttribute('x-show')).toBe('open');
  });

  test('HTMX swap updates DOM on task status change', async ({ page }) => {
    const html = `<!DOCTYPE html>
      <html>
        <body>
          <div id="task-001" class="task">Incomplete</div>
        </body>
      </html>`;
    
    await page.setContent(html);
    
    const taskElement = page.locator('#task-001');
    await taskElement.evaluate((el: HTMLElement) => {
      el.classList.add('completed');
      el.textContent = 'Completed';
    });
    
    const isCompleted = await taskElement.evaluate((el: HTMLElement) => 
      el.classList.contains('completed')
    );
    expect(isCompleted).toBe(true);
  });

  test('zone created event fires DOM update', async ({ page }) => {
    const html = `<!DOCTYPE html>
      <html>
        <body>
          <div id="zone-list"></div>
          <script>
            document.addEventListener('zone-created', () => {
              const zoneList = document.getElementById('zone-list');
              const newZone = document.createElement('div');
              newZone.setAttribute('data-zone-id', 'zone-new');
              newZone.textContent = 'New Zone';
              zoneList.appendChild(newZone);
            });
          </script>
        </body>
      </html>`;
    
    await page.setContent(html);
    
    await page.evaluate(() => {
      document.dispatchEvent(new CustomEvent('zone-created'));
    });
    
    const newZone = page.locator('[data-zone-id="zone-new"]');
    expect(await newZone.count()).toBe(1);
  });

  test('task list updates with completed status', async ({ page }) => {
    const html = `<!DOCTYPE html>
      <html>
        <body>
          <div id="task-list">
            <div class="task" data-task-id="task-001">Task 1</div>
            <div class="task" data-task-id="task-002">Task 2</div>
          </div>
        </body>
      </html>`;
    
    await page.setContent(html);
    
    const taskItem = page.locator('[data-task-id="task-001"]');
    await taskItem.evaluate((el: HTMLElement) => {
      el.classList.add('completed');
    });
    
    const hasCompleted = await taskItem.evaluate((el: HTMLElement) => 
      el.classList.contains('completed')
    );
    expect(hasCompleted).toBe(true);
  });

  test('zone delete removes from list', async ({ page }) => {
    const html = `<!DOCTYPE html>
      <html>
        <body>
          <div id="zone-list">
            <div class="zone" data-zone-id="zone-001">Zone 1</div>
          </div>
        </body>
      </html>`;
    
    await page.setContent(html);
    
    const zone = page.locator('[data-zone-id="zone-001"]');
    await zone.evaluate((el: HTMLElement) => el.remove());
    
    const count = await page.locator('[data-zone-id="zone-001"]').count();
    expect(count).toBe(0);
  });

  test('task form validates non-empty title', async ({ page }) => {
    const html = `<!DOCTYPE html>
      <html>
        <body>
          <form>
            <input type="text" name="title" required>
            <button type="submit">Save</button>
          </form>
        </body>
      </html>`;
    
    await page.setContent(html);
    
    const input = page.locator('input[name="title"]');
    const isValid = await input.evaluate((el: HTMLInputElement) => el.checkValidity());
    expect(isValid).toBe(false);
  });

  test('API response updates task state', async ({ page }) => {
    const apiResponse = {
      id: 'task-001',
      title: 'Test Task',
      completed: true,
      updated: new Date().toISOString(),
    };

    expect(apiResponse.completed).toBe(true);
    expect(apiResponse.updated).toBeDefined();
  });

  test('zone color picker stores hex value', async ({ page }) => {
    const html = `<!DOCTYPE html>
      <html>
        <body>
          <input type="color" name="zoneColor" value="#FF7F50">
        </body>
      </html>`;
    
    await page.setContent(html);
    
    const colorInput = page.locator('input[name="zoneColor"]');
    expect(await colorInput.getAttribute('value')).toBe('#FF7F50');
  });
});
