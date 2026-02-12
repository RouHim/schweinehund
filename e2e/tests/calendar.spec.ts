import { test, expect } from './fixtures';

test.describe('Calendar View', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('#tasks-list', { state: 'visible', timeout: 5000 });
    await page.request.post('/api/debug/reset-all');
    await page.reload();
    await page.waitForSelector('#tasks-list', { state: 'visible', timeout: 5000 });
  });

  test('calendar tab navigation switches to calendar view', async ({ page }) => {
    // Click calendar tab
    await page.locator('[data-testid="tab-calendar"]').click();
    
    // Assert calendar section visible, others hidden
    await expect(page.locator('[data-testid="calendar-section"]')).toBeVisible();
    await expect(page.locator('#today-section')).toBeHidden();
    await expect(page.locator('#all-tasks-section')).toBeHidden();
  });

  test('month header displays current month name in German with year', async ({ page }) => {
    await page.locator('[data-testid="tab-calendar"]').click();
    
    // Get current month/year
    const { month, year } = await page.evaluate(() => {
      const now = new Date();
      return {
        month: now.getMonth() + 1, // 1-12
        year: now.getFullYear()
      };
    });
    
    const monthNames = ['Januar', 'Februar', 'März', 'April', 'Mai', 'Juni', 
                       'Juli', 'August', 'September', 'Oktober', 'November', 'Dezember'];
    const expectedLabel = `${monthNames[month - 1]} ${year}`;
    
    const monthLabel = page.locator('[data-testid="calendar-month-label"]');
    await expect(monthLabel).toHaveText(expectedLabel);
  });

  test('day-of-week headers render correctly with Mo-So format', async ({ page }) => {
    await page.locator('[data-testid="tab-calendar"]').click();
    
    const grid = page.locator('[data-testid="calendar-grid"]');
    const weekdayHeaders = grid.locator('.calendar-weekday');
    
    // Should have exactly 7 weekday headers
    await expect(weekdayHeaders).toHaveCount(7);
    
    // First header should be Monday
    await expect(weekdayHeaders.first()).toHaveText('Mo');
    
    // Verify all headers in order
    const expectedHeaders = ['Mo', 'Di', 'Mi', 'Do', 'Fr', 'Sa', 'So'];
    for (let i = 0; i < 7; i++) {
      await expect(weekdayHeaders.nth(i)).toHaveText(expectedHeaders[i]);
    }
  });

  test('today highlighting shows exactly one cell with today class', async ({ page }) => {
    await page.locator('[data-testid="tab-calendar"]').click();
    
    // Get today's date in YYYY-MM-DD format
    const todayDate = await page.evaluate(() => {
      const now = new Date();
      const year = now.getFullYear();
      const month = (now.getMonth() + 1).toString().padStart(2, '0');
      const day = now.getDate().toString().padStart(2, '0');
      return `${year}-${month}-${day}`;
    });
    
    // Verify exactly one .today cell
    const todayCells = page.locator('.calendar-day.today');
    await expect(todayCells).toHaveCount(1);
    
    // Verify the today cell has correct data-testid
    const todayCell = todayCells.first();
    await expect(todayCell).toHaveAttribute('data-testid', `calendar-day-${todayDate}`);
  });

  test('task names displayed in correct day cells', async ({ page }) => {
    // Get today's weekday (1-7 format)
    const todayWeekday = await page.evaluate(() => {
      const jsDay = new Date().getDay();
      return jsDay === 0 ? '7' : String(jsDay);
    });
    
    const todayDate = await page.evaluate(() => {
      const now = new Date();
      const year = now.getFullYear();
      const month = (now.getMonth() + 1).toString().padStart(2, '0');
      const day = now.getDate().toString().padStart(2, '0');
      return `${year}-${month}-${day}`;
    });
    
    // Create a daily task for today's weekday
    const taskName = `Calendar Test Task ${Date.now()}`;
    const addButton = page.locator('[data-testid="add-daily-task-btn"]');
    await addButton.click();
    
    const modal = page.locator('#task-modal');
    await expect(modal).toBeVisible();
    
    await page.locator('[data-testid="task-name-input"]').fill(taskName);
    await page.locator('[data-testid="task-zone-input"]').fill('Test Zone');
    await page.locator('[data-testid="task-day-select"]').selectOption(todayWeekday);
    
    const responsePromise = page.waitForResponse(response => 
      response.url().includes('/api/tasks') && response.request().method() === 'POST'
    );
    
    await page.locator('[data-testid="modal-save-btn"]').click();
    await responsePromise;
    await expect(modal).not.toBeVisible({ timeout: 10000 });
    
    // Switch to calendar tab
    await page.locator('[data-testid="tab-calendar"]').click();
    
    // Verify task appears in today's cell - click day to open modal
    const todayCell = page.locator(`[data-testid="calendar-day-${todayDate}"]`);
    await todayCell.click();
    
    // Modal should open
    const dayModal = page.locator('[data-testid="calendar-day-modal"]');
    await expect(dayModal).toBeVisible({ timeout: 3000 });
    
    // Verify task name appears in modal
    await expect(dayModal.locator('[data-testid="calendar-day-modal-tasks"]', { hasText: taskName })).toBeVisible();
    
    // Close modal
    await page.locator('[data-testid="calendar-day-modal-close"]').click();
    await expect(dayModal).not.toBeVisible({ timeout: 3000 });
  });

  test('mini-routine appears on every day of the month', async ({ page }) => {
    // Create a mini-routine task (day_of_week = -1)
    const miniRoutineName = `Mini-Routine ${Date.now()}`;
    const addButton = page.locator('[data-testid="add-daily-task-btn"]');
    await addButton.click();
    
    const modal = page.locator('#task-modal');
    await expect(modal).toBeVisible();
    
    await page.locator('[data-testid="task-name-input"]').fill(miniRoutineName);
    await page.locator('[data-testid="task-day-select"]').selectOption('-1'); // Every day
    
    const responsePromise = page.waitForResponse(response => 
      response.url().includes('/api/tasks') && response.request().method() === 'POST'
    );
    
    await page.locator('[data-testid="modal-save-btn"]').click();
    await responsePromise;
    await expect(modal).not.toBeVisible({ timeout: 10000 });
    
    // Switch to calendar
    await page.locator('[data-testid="tab-calendar"]').click();
    
    // Count how many non-empty calendar day cells exist
    const dayCells = page.locator('.calendar-day:not(.calendar-day-empty)');
    const dayCount = await dayCells.count();
    
    // Verify mini-routine appears in ALL day cells by checking dots
    // Each day should have at least one dot indicating the mini-routine task
    for (let i = 0; i < dayCount; i++) {
      const cell = dayCells.nth(i);
      const dots = cell.locator('.calendar-task-dot');
      await expect(dots.first()).toBeVisible();
    }
  });

  test('next month navigation changes month label', async ({ page }) => {
    await page.locator('[data-testid="tab-calendar"]').click();
    
    // Get initial month label
    const monthLabel = page.locator('[data-testid="calendar-month-label"]');
    const initialLabel = await monthLabel.textContent();
    
    // Click next button
    const nextBtn = page.locator('[data-testid="calendar-next-btn"]');
    await nextBtn.click();
    
    // Wait for calendar to re-render (check for API response)
    await page.waitForResponse(response => 
      response.url().includes('/api/tasks/calendar') && response.status() === 200
    );
    
    // Month label should have changed
    const newLabel = await monthLabel.textContent();
    expect(newLabel).not.toBe(initialLabel);
    
    // Verify it's a valid German month name
    const monthNames = ['Januar', 'Februar', 'März', 'April', 'Mai', 'Juni', 
                       'Juli', 'August', 'September', 'Oktober', 'November', 'Dezember'];
    const hasValidMonth = monthNames.some(month => newLabel?.includes(month));
    expect(hasValidMonth).toBeTruthy();
  });

  test('prev button disabled on current month', async ({ page }) => {
    await page.locator('[data-testid="tab-calendar"]').click();
    
    const prevBtn = page.locator('[data-testid="calendar-prev-btn"]');
    
    // On page load (showing current month), prev button should be disabled
    await expect(prevBtn).toBeDisabled();
  });

  test('forward-only constraint: prev disabled after navigating back to current month', async ({ page }) => {
    await page.locator('[data-testid="tab-calendar"]').click();
    
    const prevBtn = page.locator('[data-testid="calendar-prev-btn"]');
    const nextBtn = page.locator('[data-testid="calendar-next-btn"]');
    
    // Initial state: prev button disabled
    await expect(prevBtn).toBeDisabled();
    
    // Navigate forward one month
    await nextBtn.click();
    await page.waitForResponse(response => 
      response.url().includes('/api/tasks/calendar')
    );
    
    // Prev button should now be enabled (not on current month)
    await expect(prevBtn).toBeEnabled();
    
    // Navigate back to current month
    await prevBtn.click();
    await page.waitForResponse(response => 
      response.url().includes('/api/tasks/calendar')
    );
    
    // Prev button should be disabled again (back on current month)
    await expect(prevBtn).toBeDisabled();
  });

  test('calendar displays task zone information', async ({ page }) => {
    // Create a task with a zone
    const taskName = `Zone Test ${Date.now()}`;
    const taskZone = 'Kitchen';
    
    const todayWeekday = await page.evaluate(() => {
      const jsDay = new Date().getDay();
      return jsDay === 0 ? '7' : String(jsDay);
    });
    
    const addButton = page.locator('[data-testid="add-daily-task-btn"]');
    await addButton.click();
    
    const modal = page.locator('#task-modal');
    await expect(modal).toBeVisible();
    
    await page.locator('[data-testid="task-name-input"]').fill(taskName);
    await page.locator('[data-testid="task-zone-input"]').fill(taskZone);
    await page.locator('[data-testid="task-day-select"]').selectOption(todayWeekday);
    
    const responsePromise = page.waitForResponse(response => 
      response.url().includes('/api/tasks') && response.request().method() === 'POST'
    );
    
    await page.locator('[data-testid="modal-save-btn"]').click();
    await responsePromise;
    await expect(modal).not.toBeVisible({ timeout: 10000 });
    
    // Switch to calendar
    await page.locator('[data-testid="tab-calendar"]').click();
    
    // Get today's date
    const todayDate = await page.evaluate(() => {
      const now = new Date();
      const year = now.getFullYear();
      const month = (now.getMonth() + 1).toString().padStart(2, '0');
      const day = now.getDate().toString().padStart(2, '0');
      return `${year}-${month}-${day}`;
    });
    
    // Click today's cell to open modal
    const todayCell = page.locator(`[data-testid="calendar-day-${todayDate}"]`);
    await todayCell.click();
    
    // Modal should open
    const dayModal = page.locator('[data-testid="calendar-day-modal"]');
    await expect(dayModal).toBeVisible({ timeout: 3000 });
    
    // Verify task name and zone appear in modal
    const modalContent = dayModal.locator('[data-testid="calendar-day-modal-tasks"]');
    await expect(modalContent, { hasText: taskName }).toBeVisible();
    await expect(modalContent, { hasText: taskZone }).toBeVisible();
    
    // Close modal
    await page.locator('[data-testid="calendar-day-modal-close"]').click();
  });

  test('days with only mini-routines render correctly', async ({ page }) => {
    await page.locator('[data-testid="tab-calendar"]').click();
    
    await page.waitForSelector('.calendar-day:not(.calendar-day-empty)', { state: 'visible', timeout: 5000 });
    
    const firstDayCell = page.locator('.calendar-day:not(.calendar-day-empty)').first();
    
    await expect(firstDayCell.locator('.calendar-day-number')).toBeVisible();
    
    const taskDots = firstDayCell.locator('.calendar-task-dot');
    const dotsCount = await taskDots.count();
    
    expect(dotsCount).toBe(3);
    
    const overflowIndicator = firstDayCell.locator('.calendar-task-more');
    const hasOverflow = await overflowIndicator.isVisible();
    
    if (hasOverflow) {
      const overflowText = await overflowIndicator.textContent();
      expect(overflowText).toMatch(/\+\d+/);
    }
  });

  test('clicking a day opens detail modal with task info', async ({ page }) => {
    const todayWeekday = await page.evaluate(() => {
      const dayOfWeek = new Date().getDay();
      return dayOfWeek === 0 ? '7' : dayOfWeek.toString();
    });
    
    const todayDate = await page.evaluate(() => {
      const now = new Date();
      const year = now.getFullYear();
      const month = (now.getMonth() + 1).toString().padStart(2, '0');
      const day = now.getDate().toString().padStart(2, '0');
      return `${year}-${month}-${day}`;
    });
    
    const taskName = `Modal Test Task ${Date.now()}`;
    const taskZone = 'Modal Test Zone';
    
    const addButton = page.locator('[data-testid="add-daily-task-btn"]');
    await addButton.click();
    
    const modal = page.locator('#task-modal');
    await expect(modal).toBeVisible();
    
    await page.locator('[data-testid="task-name-input"]').fill(taskName);
    await page.locator('[data-testid="task-zone-input"]').fill(taskZone);
    await page.locator('[data-testid="task-day-select"]').selectOption(todayWeekday);
    
    const responsePromise = page.waitForResponse(response => 
      response.url().includes('/api/tasks') && response.request().method() === 'POST'
    );
    
    await page.locator('[data-testid="modal-save-btn"]').click();
    await responsePromise;
    await expect(modal).not.toBeVisible({ timeout: 10000 });
    
    await page.locator('[data-testid="tab-calendar"]').click();
    
    const todayCell = page.locator(`[data-testid="calendar-day-${todayDate}"]`);
    await todayCell.click();
    
    const dayModal = page.locator('[data-testid="calendar-day-modal"]');
    await expect(dayModal).toBeVisible({ timeout: 3000 });
    
    const modalTitle = dayModal.locator('[data-testid="calendar-day-modal-title"]');
    await expect(modalTitle).toBeVisible();
    
    const modalContent = dayModal.locator('[data-testid="calendar-day-modal-tasks"]');
    await expect(modalContent, { hasText: taskName }).toBeVisible();
    await expect(modalContent, { hasText: taskZone }).toBeVisible();
    
    await page.locator('[data-testid="calendar-day-modal-close"]').click();
    await expect(dayModal).not.toBeVisible({ timeout: 3000 });
  });

  test('day detail modal shows mini-routine tasks', async ({ page }) => {
    await page.locator('[data-testid="tab-calendar"]').click();
    await page.waitForSelector('.calendar-day:not(.calendar-day-empty)', { state: 'visible', timeout: 5000 });
    
    const firstDayCell = page.locator('.calendar-day:not(.calendar-day-empty)').first();
    await firstDayCell.click();
    
    const dayModal = page.locator('[data-testid="calendar-day-modal"]');
    await expect(dayModal).toBeVisible({ timeout: 3000 });
    
    const modalContent = dayModal.locator('[data-testid="calendar-day-modal-tasks"]');
    
    await expect(modalContent).toContainText('Spuelmaschine');
    
    await page.locator('[data-testid="calendar-day-modal-close"]').click();
    await expect(dayModal).not.toBeVisible({ timeout: 3000 });
  });

  test('day detail modal closes via close button', async ({ page }) => {
    const todayWeekday = await page.evaluate(() => {
      const dayOfWeek = new Date().getDay();
      return dayOfWeek === 0 ? '7' : dayOfWeek.toString();
    });
    
    const todayDate = await page.evaluate(() => {
      const now = new Date();
      const year = now.getFullYear();
      const month = (now.getMonth() + 1).toString().padStart(2, '0');
      const day = now.getDate().toString().padStart(2, '0');
      return `${year}-${month}-${day}`;
    });
    
    const taskName = `Close Test Task ${Date.now()}`;
    
    const addButton = page.locator('[data-testid="add-daily-task-btn"]');
    await addButton.click();
    
    const modal = page.locator('#task-modal');
    await expect(modal).toBeVisible();
    
    await page.locator('[data-testid="task-name-input"]').fill(taskName);
    await page.locator('[data-testid="task-zone-input"]').fill('Close Test Zone');
    await page.locator('[data-testid="task-day-select"]').selectOption(todayWeekday);
    
    const responsePromise = page.waitForResponse(response => 
      response.url().includes('/api/tasks') && response.request().method() === 'POST'
    );
    
    await page.locator('[data-testid="modal-save-btn"]').click();
    await responsePromise;
    await expect(modal).not.toBeVisible({ timeout: 10000 });
    
    await page.locator('[data-testid="tab-calendar"]').click();
    
    const todayCell = page.locator(`[data-testid="calendar-day-${todayDate}"]`);
    await todayCell.click();
    
    const dayModal = page.locator('[data-testid="calendar-day-modal"]');
    await expect(dayModal).toBeVisible({ timeout: 3000 });
    
    await page.locator('[data-testid="calendar-day-modal-close"]').click();
    await expect(dayModal).not.toBeVisible({ timeout: 3000 });
  });

  test('responsive layout works on mobile viewport', async ({ page }) => {
    // Set mobile viewport
    await page.setViewportSize({ width: 375, height: 667 });
    
    await page.locator('[data-testid="tab-calendar"]').click();
    
    // Verify calendar grid is still visible and functional
    const grid = page.locator('[data-testid="calendar-grid"]');
    await expect(grid).toBeVisible();
    
    // Verify weekday headers still render
    const weekdayHeaders = grid.locator('.calendar-weekday');
    await expect(weekdayHeaders).toHaveCount(7);
    
    // Verify month label is visible
    const monthLabel = page.locator('[data-testid="calendar-month-label"]');
    await expect(monthLabel).toBeVisible();
    
    // Verify navigation buttons are clickable
    const nextBtn = page.locator('[data-testid="calendar-next-btn"]');
    await expect(nextBtn).toBeVisible();
  });
});
