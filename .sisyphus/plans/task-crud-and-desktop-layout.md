# Task CRUD UI + Desktop Layout Improvements

## TL;DR

> **Quick Summary**: Add full task management (Create, Read, Update, Delete) for both Daily and Deep Cleaning tasks via modal dialogs, plus a responsive two-column desktop layout.
> 
> **Deliverables**:
> - REST API endpoints for task CRUD + queue reordering
> - Modal dialog for creating/editing tasks
> - Edit/delete buttons on task cards
> - Drag-and-drop reordering for deep cleaning (desktop) + up/down buttons (mobile fallback)
> - Two-column layout on desktop (≥1024px)
> - Playwright E2E tests for new functionality
> 
> **Estimated Effort**: Medium-Large
> **Parallel Execution**: YES - 3 waves
> **Critical Path**: API endpoints → Frontend CRUD → E2E Tests

---

## Context

### Original Request
"How to add new tasks via the UI? If that's not possible let's add this. Also the UI looks pretty ugly on desktop."

### Interview Summary
**Key Discussions**:
- **Task types**: Both Daily Tasks and Deep Cleaning need full CRUD
- **CRUD level**: Full Create, Read, Update, Delete
- **UI pattern**: Modal dialog for create/edit forms
- **Delete UX**: Simple confirm() dialog
- **Edit trigger**: Edit button on each task card
- **Deep cleaning queue**: Auto-rotate on complete + manual reordering via drag-drop + up/down buttons
- **Daily task fields**: Name, description (optional), day_of_week, zone (optional)
- **Deep cleaning fields**: Name, description (optional), zone (optional)
- **Add button placement**: In section header (e.g., "+ Add" next to "Today's Tasks")
- **Desktop layout**: Two-column (Daily tasks left, Deep cleaning right)
- **Mobile reorder**: Up/down arrow buttons (HTML5 DND unreliable on touch)
- **API convention**: REST verbs (PUT/DELETE)
- **Testing**: Extend existing Playwright E2E suite

**Research Findings**:
- Backend: Rust (warp + sqlx), SQLite at `data/schweinehund.db`
- Frontend: Vanilla JS + Pico.css (classless), no framework
- No existing CRUD endpoints - only read + toggle/complete
- Tasks hardcoded in `migrations/001_initial.sql`
- Single-column layout with max-width: 640px
- E2E tests use Playwright with ID/class selectors

### Metis Review
**Identified Gaps** (addressed):
- Deep cleaning fields: Resolved → name, description, zone (all fields)
- Mobile reorder fallback: Resolved → up/down arrow buttons
- API verb convention: Resolved → REST verbs (PUT/DELETE)
- Validation rules: Applied default → max 255 chars, name required
- Edit modal pre-population: Yes, show current values
- Desktop breakpoint: 1024px threshold
- Click outside modal: Closes with discard (standard dialog behavior)

---

## Work Objectives

### Core Objective
Enable users to create, edit, and delete tasks directly in the UI, and improve the desktop experience with a two-column layout.

### Concrete Deliverables
1. **API Endpoints**:
   - `POST /api/tasks` - Create daily task
   - `PUT /api/tasks/:id` - Update daily task
   - `DELETE /api/tasks/:id` - Delete daily task
   - `POST /api/deep-cleaning` - Create deep cleaning task
   - `PUT /api/deep-cleaning/:id` - Update deep cleaning task
   - `DELETE /api/deep-cleaning/:id` - Delete deep cleaning task
   - `POST /api/deep-cleaning/reorder` - Reorder queue

2. **Frontend Components**:
   - Task create/edit modal dialog (`<dialog>` element)
   - Edit/delete buttons on task cards
   - Add task button in section headers
   - Drag-and-drop for deep cleaning list (desktop)
   - Up/down arrow buttons for reordering (mobile)

3. **CSS Changes**:
   - Two-column grid layout for desktop (≥1024px)
   - Modal styling
   - Reorder button styling

4. **E2E Tests**:
   - `e2e/tests/crud.spec.ts` - Task CRUD operations
   - `e2e/tests/desktop-layout.spec.ts` - Layout verification

### Definition of Done
- [ ] All CRUD operations work for both task types
- [ ] Modal opens/closes correctly, form validates
- [ ] Drag-drop reordering persists after reload
- [ ] Up/down buttons work on narrow viewports
- [ ] Two-column layout visible at ≥1024px
- [ ] Mobile layout unchanged at <768px
- [ ] All E2E tests pass: `cd e2e && npx playwright test`
- [ ] `cargo build` succeeds with no warnings

### Must Have
- Full CRUD for Daily Tasks and Deep Cleaning
- Modal dialog pattern (using native `<dialog>`)
- Edit and delete buttons on task cards
- Drag-drop reordering on desktop
- Up/down buttons for mobile reordering
- Two-column desktop layout
- E2E test coverage for new features

### Must NOT Have (Guardrails)
- NO JavaScript frameworks (React, Vue, etc.) - keep vanilla JS
- NO npm dependencies for drag-drop - use native HTML5 DND
- NO bulk operations (multi-select, bulk delete)
- NO task search/filter functionality
- NO changes to existing toggle/complete behavior
- NO modifications to `migrations/001_initial.sql`
- NO authentication/authorization
- NO changes to Settings page
- NO "undo" for delete - confirm() is final

---

## Verification Strategy

> **UNIVERSAL RULE: ZERO HUMAN INTERVENTION**
>
> ALL tasks in this plan MUST be verifiable WITHOUT any human action.

### Test Decision
- **Infrastructure exists**: YES (Playwright in `e2e/`)
- **Automated tests**: Tests-after (E2E for integration)
- **Framework**: Playwright

### Agent-Executed QA Scenarios (MANDATORY)

Each task includes specific Playwright or curl verification scenarios.

---

## Execution Strategy

### Parallel Execution Waves

```
Wave 1 (Start Immediately):
├── Task 1: Backend - Daily Task CRUD endpoints
└── Task 2: Backend - Deep Cleaning CRUD + Reorder endpoints

Wave 2 (After Wave 1):
├── Task 3: Frontend - Modal dialog component
├── Task 4: Frontend - Edit/Delete buttons on cards
└── Task 5: CSS - Two-column desktop layout

Wave 3 (After Wave 2):
├── Task 6: Frontend - Drag-drop + Up/Down reordering
├── Task 7: Frontend - Add Task buttons in headers
└── Task 8: E2E Tests - CRUD + Layout verification

Critical Path: Task 1,2 → Task 3,4 → Task 8
Parallel Speedup: ~35% faster than sequential
```

### Dependency Matrix

| Task | Depends On | Blocks | Can Parallelize With |
|------|------------|--------|---------------------|
| 1 | None | 3, 4, 7, 8 | 2 |
| 2 | None | 3, 4, 6, 7, 8 | 1 |
| 3 | 1, 2 | 7, 8 | 4, 5 |
| 4 | 1, 2 | 8 | 3, 5 |
| 5 | None | 8 | 3, 4 |
| 6 | 2 | 8 | 7 |
| 7 | 1, 2, 3 | 8 | 6 |
| 8 | 1-7 | None | None (final) |

---

## TODOs

- [x] 1. Backend: Daily Task CRUD Endpoints

  **What to do**:
  - Add CORS support for PUT and DELETE methods in `src/routes.rs`
  - Implement `POST /api/tasks` - Create new daily task
    - Request body: `{ "name": string, "description"?: string, "day_of_week": number, "zone"?: string }`
    - Response: Created task with ID (HTTP 201)
  - Implement `PUT /api/tasks/:id` - Update daily task
    - Request body: Same fields as create
    - Response: Updated task (HTTP 200)
  - Implement `DELETE /api/tasks/:id` - Delete daily task
    - Response: HTTP 204 No Content
  - Add corresponding database functions in `src/db.rs`
  - Validate: name required, max 255 chars; day_of_week 1-7 or -1 (daily)

  **Must NOT do**:
  - Do NOT modify existing toggle endpoint behavior
  - Do NOT add authentication
  - Do NOT modify `001_initial.sql`

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: Backend Rust work requiring careful async/await and sqlx patterns
  - **Skills**: [`git-master`]
    - `git-master`: For atomic commits of API changes

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Task 2)
  - **Blocks**: Tasks 3, 4, 7, 8
  - **Blocked By**: None

  **References**:

  **Pattern References** (existing code to follow):
  - `src/routes.rs:43-61` - Existing route definitions with warp filters
  - `src/routes.rs:73-95` - Handler function pattern (`handle_toggle_task`)
  - `src/db.rs:48-72` - Query patterns using `sqlx::query_as` and `fetch_one/all`

  **API/Type References** (contracts to implement against):
  - `src/db.rs:8-16` - `DailyTask` struct definition
  - `src/routes.rs:46-49` - CORS configuration (add PUT, DELETE)

  **Test References** (testing patterns to follow):
  - `e2e/tests/tasks.spec.ts` - HTTP request patterns for verification

  **Acceptance Criteria**:

  **Agent-Executed QA Scenarios:**

  ```
  Scenario: Create daily task returns 201 with new ID
    Tool: Bash (curl)
    Preconditions: Server running on localhost:3000
    Steps:
      1. curl -s -w "\n%{http_code}" -X POST http://localhost:3000/api/tasks \
           -H "Content-Type: application/json" \
           -d '{"name":"Test Task","description":"Test desc","day_of_week":1,"zone":"Kitchen"}'
      2. Assert: Last line (HTTP status) is "201"
      3. Assert: Response body contains "id" field with integer value
      4. Assert: Response body contains "name": "Test Task"
    Expected Result: Task created, returns with ID
    Evidence: Response body saved to .sisyphus/evidence/task-1-create.json

  Scenario: Update daily task returns 200 with updated data
    Tool: Bash (curl)
    Preconditions: Task with ID 1 exists
    Steps:
      1. curl -s -w "\n%{http_code}" -X PUT http://localhost:3000/api/tasks/1 \
           -H "Content-Type: application/json" \
           -d '{"name":"Updated Task","description":"Updated","day_of_week":2,"zone":"Bath"}'
      2. Assert: Last line is "200"
      3. Assert: Response contains "name": "Updated Task"
    Expected Result: Task updated successfully
    Evidence: Response body saved

  Scenario: Delete daily task returns 204
    Tool: Bash (curl)
    Preconditions: Task exists
    Steps:
      1. Create a task first via POST
      2. curl -s -w "%{http_code}" -X DELETE http://localhost:3000/api/tasks/{id}
      3. Assert: HTTP status is "204"
      4. GET /api/tasks/today → Assert deleted task not in list
    Expected Result: Task deleted, no longer appears
    Evidence: GET response showing absence

  Scenario: Create with missing name returns 400
    Tool: Bash (curl)
    Preconditions: Server running
    Steps:
      1. curl -s -w "\n%{http_code}" -X POST http://localhost:3000/api/tasks \
           -H "Content-Type: application/json" \
           -d '{"description":"No name","day_of_week":1}'
      2. Assert: HTTP status is "400"
    Expected Result: Validation error returned
    Evidence: Error response saved
  ```

  **Commit**: YES
  - Message: `feat(api): add CRUD endpoints for daily tasks`
  - Files: `src/routes.rs`, `src/db.rs`
  - Pre-commit: `cargo build && cargo clippy`

---

- [x] 2. Backend: Deep Cleaning CRUD + Reorder Endpoints

  **What to do**:
  - Implement `POST /api/deep-cleaning` - Create new deep cleaning task
    - Request body: `{ "name": string, "description"?: string, "zone"?: string }`
    - New tasks added to END of queue (highest queue_position + 1)
    - Response: Created task with ID (HTTP 201)
  - Implement `PUT /api/deep-cleaning/:id` - Update deep cleaning task
    - Request body: Same fields (NOT queue_position - that's via reorder)
    - Response: Updated task (HTTP 200)
  - Implement `DELETE /api/deep-cleaning/:id` - Delete deep cleaning task
    - Response: HTTP 204 No Content
    - Adjust queue_positions of remaining tasks to stay sequential
  - Implement `POST /api/deep-cleaning/reorder` - Reorder queue
    - Request body: `{ "order": [3, 1, 2, 4] }` (array of task IDs in new order)
    - Update queue_position for each task based on array index
    - Response: HTTP 200 with updated queue

  **Must NOT do**:
  - Do NOT modify existing complete endpoint behavior
  - Do NOT allow setting queue_position directly in create/update

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: Backend Rust work with queue position logic
  - **Skills**: [`git-master`]
    - `git-master`: For atomic commits

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Task 1)
  - **Blocks**: Tasks 3, 4, 6, 7, 8
  - **Blocked By**: None

  **References**:

  **Pattern References**:
  - `src/routes.rs:97-120` - `handle_complete_deep_cleaning` pattern
  - `src/db.rs:91-108` - `complete_deep_cleaning_task` with queue rotation logic

  **API/Type References**:
  - `src/db.rs:18-26` - `DeepCleaningTask` struct (add `zone` field if not present)

  **Acceptance Criteria**:

  **Agent-Executed QA Scenarios:**

  ```
  Scenario: Create deep cleaning task appends to queue end
    Tool: Bash (curl)
    Preconditions: Server running, existing tasks in queue
    Steps:
      1. GET /api/deep-cleaning → Note current last position
      2. POST /api/deep-cleaning with {"name":"New Deep Task","zone":"Garage"}
      3. Assert: HTTP 201
      4. GET /api/deep-cleaning → Assert new task is LAST in list
    Expected Result: New task at end of queue
    Evidence: Before/after queue snapshots

  Scenario: Reorder deep cleaning queue persists order
    Tool: Bash (curl)
    Preconditions: At least 3 deep cleaning tasks exist
    Steps:
      1. GET /api/deep-cleaning → Get current order, note IDs [A, B, C]
      2. POST /api/deep-cleaning/reorder with {"order": [C, A, B]}
      3. Assert: HTTP 200
      4. GET /api/deep-cleaning → Assert order is now [C, A, B]
    Expected Result: Queue reordered as specified
    Evidence: Response bodies saved

  Scenario: Delete adjusts queue positions
    Tool: Bash (curl)
    Preconditions: Queue has [A, B, C] with positions [1, 2, 3]
    Steps:
      1. DELETE /api/deep-cleaning/{B}
      2. Assert: HTTP 204
      3. GET /api/deep-cleaning → Assert C now has position 2
    Expected Result: No gaps in queue positions
    Evidence: GET response showing sequential positions
  ```

  **Commit**: YES
  - Message: `feat(api): add CRUD and reorder endpoints for deep cleaning`
  - Files: `src/routes.rs`, `src/db.rs`
  - Pre-commit: `cargo build && cargo clippy`

---

- [x] 3. Frontend: Modal Dialog Component

  **What to do**:
  - Create reusable modal using native `<dialog>` element in `static/index.html`
  - Add modal HTML structure:
    ```html
    <dialog id="task-modal">
      <article>
        <header><h3 id="modal-title">Add Task</h3></header>
        <form id="task-form">
          <input type="hidden" name="task-type" />
          <input type="hidden" name="task-id" />
          <label>Name<input type="text" name="name" required maxlength="255" /></label>
          <label>Description<textarea name="description" maxlength="1000"></textarea></label>
          <label>Zone<input type="text" name="zone" maxlength="100" placeholder="e.g., Kitchen" /></label>
          <label id="day-of-week-field">Day of Week
            <select name="day_of_week">
              <option value="-1">Every day</option>
              <option value="1">Monday</option>
              <option value="2">Tuesday</option>
              <option value="3">Wednesday</option>
              <option value="4">Thursday</option>
              <option value="5">Friday</option>
              <option value="6">Saturday</option>
              <option value="7">Sunday</option>
            </select>
          </label>
          <footer>
            <button type="button" class="secondary" data-close-modal>Cancel</button>
            <button type="submit">Save</button>
          </footer>
        </form>
      </article>
    </dialog>
    ```
  - Add `data-testid` attributes for E2E testing
  - In `static/app.js`:
    - Add `openModal(type, task = null)` function
      - type: 'daily' or 'deep-cleaning'
      - If task provided, pre-populate form (edit mode)
      - Show/hide day_of_week field based on type
    - Add `closeModal()` function
    - Add form submit handler that calls appropriate API endpoint
    - On successful save, close modal and refresh task list
  - Style modal in `static/style.css` (Pico.css provides base dialog styling)

  **Must NOT do**:
  - Do NOT use any JS library for modal
  - Do NOT add complex animations

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
    - Reason: Frontend UI component work with form handling
  - **Skills**: [`frontend-ui-ux`]
    - `frontend-ui-ux`: Modal design and form UX patterns

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 4, 5)
  - **Blocks**: Tasks 7, 8
  - **Blocked By**: Tasks 1, 2

  **References**:

  **Pattern References**:
  - `static/app.js:147-175` - Settings form handling pattern
  - `static/app.js:65-92` - DOM manipulation and template patterns
  - `static/index.html:54-65` - Form structure pattern

  **External References**:
  - MDN `<dialog>`: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dialog

  **Acceptance Criteria**:

  **Agent-Executed QA Scenarios:**

  ```
  Scenario: Modal opens for creating daily task
    Tool: Playwright (playwright skill)
    Preconditions: Dev server running on localhost:3000
    Steps:
      1. Navigate to: http://localhost:3000
      2. Click: [data-testid="add-daily-task-btn"]
      3. Wait for: dialog[open] visible (timeout: 3s)
      4. Assert: #modal-title text is "Add Task" or "Add Daily Task"
      5. Assert: [name="day_of_week"] select is visible
      6. Assert: [name="name"] input is empty
      7. Screenshot: .sisyphus/evidence/task-3-modal-open.png
    Expected Result: Modal opens with empty form, day selector visible
    Evidence: .sisyphus/evidence/task-3-modal-open.png

  Scenario: Modal pre-populates for editing existing task
    Tool: Playwright (playwright skill)
    Preconditions: At least one daily task exists
    Steps:
      1. Navigate to: http://localhost:3000
      2. Click: .task-item:first-child [data-testid="edit-btn"]
      3. Wait for: dialog[open] visible
      4. Assert: [name="name"] value is NOT empty
      5. Assert: [name="task-id"] value is NOT empty
      6. Screenshot: .sisyphus/evidence/task-3-modal-edit.png
    Expected Result: Modal shows existing task data
    Evidence: .sisyphus/evidence/task-3-modal-edit.png

  Scenario: Cancel closes modal without saving
    Tool: Playwright (playwright skill)
    Steps:
      1. Open modal via add button
      2. Fill: [name="name"] with "Should Not Save"
      3. Click: [data-close-modal] button
      4. Wait for: dialog[open] not visible (timeout: 2s)
      5. Assert: No task named "Should Not Save" in list
    Expected Result: Modal closes, no task created
    Evidence: Screenshot of task list without new task

  Scenario: Deep cleaning modal hides day_of_week field
    Tool: Playwright (playwright skill)
    Steps:
      1. Click: [data-testid="add-deep-cleaning-btn"]
      2. Wait for: dialog[open] visible
      3. Assert: #day-of-week-field is NOT visible or display:none
    Expected Result: Day selector hidden for deep cleaning
    Evidence: Screenshot showing hidden field
  ```

  **Commit**: YES
  - Message: `feat(ui): add modal dialog component for task create/edit`
  - Files: `static/index.html`, `static/app.js`, `static/style.css`
  - Pre-commit: `cargo build` (embeds static files)

---

- [x] 4. Frontend: Edit/Delete Buttons on Task Cards

  **What to do**:
  - Modify `renderTasks()` in `static/app.js` to include edit/delete buttons
  - Modify `renderDeepCleaning()` similarly
  - Button HTML:
    ```html
    <div class="task-actions">
      <button data-testid="edit-btn" data-task-id="${task.id}" class="icon-btn edit-btn" aria-label="Edit task">
        <svg><!-- pencil icon --></svg>
      </button>
      <button data-testid="delete-btn" data-task-id="${task.id}" class="icon-btn delete-btn" aria-label="Delete task">
        <svg><!-- trash icon --></svg>
      </button>
    </div>
    ```
  - Add click handlers:
    - Edit: Call `openModal(type, task)`
    - Delete: `if (confirm('Delete this task?')) { deleteTask(type, id); }`
  - Add `deleteTask(type, id)` function that calls DELETE endpoint
  - Style buttons in `static/style.css`:
    - Small icon buttons, positioned in task card corner
    - Hover states, accessible focus states

  **Must NOT do**:
  - Do NOT change checkbox toggle behavior
  - Do NOT use icon library - use inline SVG

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
    - Reason: UI component work with event handling
  - **Skills**: [`frontend-ui-ux`]
    - `frontend-ui-ux`: Icon button design, hover states

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 3, 5)
  - **Blocks**: Task 8
  - **Blocked By**: Tasks 1, 2

  **References**:

  **Pattern References**:
  - `static/app.js:65-92` - `renderTasks()` template structure
  - `static/app.js:94-118` - `renderDeepCleaning()` template structure
  - `static/app.js:120-145` - Event listener attachment pattern

  **Acceptance Criteria**:

  **Agent-Executed QA Scenarios:**

  ```
  Scenario: Edit button opens modal with task data
    Tool: Playwright (playwright skill)
    Steps:
      1. Navigate to http://localhost:3000
      2. Wait for: .task-item visible
      3. Click: .task-item:first-child [data-testid="edit-btn"]
      4. Wait for: dialog[open] visible
      5. Assert: [name="name"] value equals first task's name
    Expected Result: Modal shows correct task data
    Evidence: Screenshot

  Scenario: Delete button removes task after confirm
    Tool: Playwright (playwright skill)
    Steps:
      1. Navigate and wait for tasks
      2. Count: .task-item elements → store as initialCount
      3. Note: First task's name
      4. Click: .task-item:first-child [data-testid="delete-btn"]
      5. (Playwright auto-accepts confirm dialogs)
      6. Wait for: task list to update
      7. Count: .task-item elements → Assert equals initialCount - 1
      8. Assert: Deleted task name no longer in list
    Expected Result: Task deleted, list updated
    Evidence: Before/after screenshots

  Scenario: Toggle still works with new buttons present
    Tool: Playwright (playwright skill)
    Steps:
      1. Navigate, wait for tasks
      2. Check: First task checkbox is unchecked
      3. Click: .task-item:first-child .task-checkbox
      4. Assert: Checkbox is now checked
      5. Assert: .task-item:first-child has class "completed"
    Expected Result: Existing toggle behavior unchanged
    Evidence: Screenshot showing completed state
  ```

  **Commit**: YES
  - Message: `feat(ui): add edit and delete buttons to task cards`
  - Files: `static/app.js`, `static/style.css`
  - Pre-commit: `cargo build`

---

- [x] 5. CSS: Two-Column Desktop Layout

  **What to do**:
  - Add media query in `static/style.css` for ≥1024px viewport
  - Create two-column grid layout:
    ```css
    @media (min-width: 1024px) {
      main.container {
        display: grid;
        grid-template-columns: 1fr 1fr;
        gap: 2rem;
        max-width: 1200px;
      }
      
      #today-section {
        grid-column: 1;
      }
      
      #deep-cleaning-section {
        grid-column: 2;
      }
      
      #settings-section {
        grid-column: 1 / -1; /* Full width */
      }
    }
    ```
  - Ensure task cards don't exceed comfortable reading width within columns
  - Keep mobile layout unchanged (<768px)
  - Test at various viewport sizes: 1024px, 1440px, 1920px

  **Must NOT do**:
  - Do NOT change mobile layout
  - Do NOT add horizontal scrolling
  - Do NOT exceed 600px content width per column

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
    - Reason: Pure CSS layout work
  - **Skills**: [`frontend-ui-ux`]
    - `frontend-ui-ux`: Responsive grid patterns

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 3, 4)
  - **Blocks**: Task 8
  - **Blocked By**: None (CSS only, no API dependency)

  **References**:

  **Pattern References**:
  - `static/style.css:322-334` - Existing media query pattern
  - `static/style.css:122-130` - `.container` styling

  **Acceptance Criteria**:

  **Agent-Executed QA Scenarios:**

  ```
  Scenario: Desktop shows two-column layout at 1024px
    Tool: Playwright (playwright skill)
    Steps:
      1. page.setViewportSize({ width: 1024, height: 768 })
      2. Navigate to http://localhost:3000
      3. Wait for: #today-section and #deep-cleaning-section visible
      4. Get bounding box of #today-section → dailyBox
      5. Get bounding box of #deep-cleaning-section → deepBox
      6. Assert: dailyBox.x + dailyBox.width < deepBox.x (daily LEFT of deep)
      7. Assert: Both sections have similar Y position (same row)
      8. Screenshot: .sisyphus/evidence/task-5-desktop-1024.png
    Expected Result: Two columns side by side
    Evidence: .sisyphus/evidence/task-5-desktop-1024.png

  Scenario: Wide desktop (1920px) maintains readable column width
    Tool: Playwright (playwright skill)
    Steps:
      1. page.setViewportSize({ width: 1920, height: 1080 })
      2. Navigate to http://localhost:3000
      3. Get bounding box of #today-section → dailyBox
      4. Assert: dailyBox.width <= 600 (readable column)
      5. Screenshot: .sisyphus/evidence/task-5-desktop-1920.png
    Expected Result: Columns don't stretch too wide
    Evidence: .sisyphus/evidence/task-5-desktop-1920.png

  Scenario: Mobile layout unchanged at 768px
    Tool: Playwright (playwright skill)
    Steps:
      1. page.setViewportSize({ width: 768, height: 1024 })
      2. Navigate to http://localhost:3000
      3. Get bounding box of #today-section → dailyBox
      4. Get bounding box of #deep-cleaning-section → deepBox
      5. Assert: dailyBox.x roughly equals deepBox.x (same column)
      6. Assert: deepBox.y > dailyBox.y + dailyBox.height (stacked vertically)
      7. Screenshot: .sisyphus/evidence/task-5-mobile.png
    Expected Result: Single column, stacked sections
    Evidence: .sisyphus/evidence/task-5-mobile.png
  ```

  **Commit**: YES
  - Message: `feat(ui): add two-column desktop layout for wider screens`
  - Files: `static/style.css`
  - Pre-commit: `cargo build`

---

- [x] 6. Frontend: Drag-Drop + Up/Down Reordering

  **What to do**:
  - Implement HTML5 native drag-and-drop for deep cleaning list on desktop:
    - Add `draggable="true"` attribute to deep cleaning task items
    - Add dragstart, dragover, drop event handlers
    - On drop, calculate new order and call `POST /api/deep-cleaning/reorder`
    - Visual feedback: drag ghost, drop target highlight
  - Implement up/down arrow buttons for mobile fallback:
    - Add up/down buttons to each deep cleaning task
    - Show only on viewports <1024px (CSS media query)
    - Click handler: swap position with adjacent task, call reorder API
  - Hide up/down buttons on desktop where drag-drop works
  - Add `data-testid` attributes for testing

  **Must NOT do**:
  - Do NOT use any external drag-drop library (SortableJS, etc.)
  - Do NOT add complex animations
  - Do NOT enable drag-drop for daily tasks

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
    - Reason: Complex interaction pattern with DND
  - **Skills**: [`frontend-ui-ux`]
    - `frontend-ui-ux`: Drag-drop UX patterns

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3 (with Task 7)
  - **Blocks**: Task 8
  - **Blocked By**: Task 2

  **References**:

  **Pattern References**:
  - `static/app.js:94-118` - `renderDeepCleaning()` to modify

  **External References**:
  - MDN Drag and Drop API: https://developer.mozilla.org/en-US/docs/Web/API/HTML_Drag_and_Drop_API

  **Acceptance Criteria**:

  **Agent-Executed QA Scenarios:**

  ```
  Scenario: Desktop drag-drop reorders deep cleaning queue
    Tool: Playwright (playwright skill)
    Steps:
      1. page.setViewportSize({ width: 1280, height: 800 })
      2. Navigate to http://localhost:3000
      3. Wait for: #deep-cleaning-list .task-item (at least 2)
      4. Note: Text of first task → originalFirst
      5. Note: Text of second task → originalSecond
      6. Drag: #deep-cleaning-list .task-item:first-child
      7. Drop: to #deep-cleaning-list .task-item:nth-child(2) position
      8. Wait for: API call to complete
      9. Assert: First task text is now originalSecond
      10. page.reload()
      11. Assert: Order persisted (first task still originalSecond)
      12. Screenshot: .sisyphus/evidence/task-6-dragdrop.png
    Expected Result: Drag-drop reorders and persists
    Evidence: .sisyphus/evidence/task-6-dragdrop.png

  Scenario: Mobile shows up/down buttons, no drag
    Tool: Playwright (playwright skill)
    Steps:
      1. page.setViewportSize({ width: 375, height: 667 })
      2. Navigate to http://localhost:3000
      3. Wait for: #deep-cleaning-list .task-item
      4. Assert: [data-testid="move-up-btn"] is visible
      5. Assert: [data-testid="move-down-btn"] is visible
      6. Click: Second task's [data-testid="move-up-btn"]
      7. Wait for: API response
      8. Assert: Second task is now first
      9. Screenshot: .sisyphus/evidence/task-6-updown.png
    Expected Result: Arrow buttons work for reordering
    Evidence: .sisyphus/evidence/task-6-updown.png

  Scenario: Desktop hides up/down buttons
    Tool: Playwright (playwright skill)
    Steps:
      1. page.setViewportSize({ width: 1280, height: 800 })
      2. Navigate to http://localhost:3000
      3. Assert: [data-testid="move-up-btn"] is NOT visible (display:none)
    Expected Result: Clean desktop UI without redundant buttons
    Evidence: Screenshot
  ```

  **Commit**: YES
  - Message: `feat(ui): add drag-drop and up/down reordering for deep cleaning`
  - Files: `static/app.js`, `static/style.css`
  - Pre-commit: `cargo build`

---

- [x] 7. Frontend: Add Task Buttons in Section Headers

  **What to do**:
  - Modify `static/index.html` section headers to include add buttons:
    ```html
    <section id="today-section">
      <div class="section-header">
        <h2>Today's Tasks</h2>
        <button data-testid="add-daily-task-btn" class="add-btn" aria-label="Add daily task">
          + Add
        </button>
      </div>
      <!-- ... -->
    </section>
    ```
  - Same pattern for deep cleaning section
  - Style `.section-header` as flexbox with space-between
  - Style `.add-btn` to match app aesthetic (primary color, compact)
  - Wire up click handlers to `openModal('daily')` and `openModal('deep-cleaning')`

  **Must NOT do**:
  - Do NOT change h2 styling/sizing
  - Do NOT add floating action button (FAB)

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Simple UI addition, mostly HTML/CSS
  - **Skills**: [`frontend-ui-ux`]
    - `frontend-ui-ux`: Button styling patterns

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3 (with Task 6)
  - **Blocks**: Task 8
  - **Blocked By**: Tasks 1, 2, 3

  **References**:

  **Pattern References**:
  - `static/index.html:33-39` - Section structure
  - `static/style.css:298-320` - Button styling patterns

  **Acceptance Criteria**:

  **Agent-Executed QA Scenarios:**

  ```
  Scenario: Add button visible in section headers
    Tool: Playwright (playwright skill)
    Steps:
      1. Navigate to http://localhost:3000
      2. Assert: [data-testid="add-daily-task-btn"] visible in #today-section
      3. Assert: [data-testid="add-deep-cleaning-btn"] visible in #deep-cleaning-section
      4. Assert: Buttons are horizontally aligned with h2 headings
      5. Screenshot: .sisyphus/evidence/task-7-headers.png
    Expected Result: Add buttons visible in both section headers
    Evidence: .sisyphus/evidence/task-7-headers.png

  Scenario: Add daily task button opens modal
    Tool: Playwright (playwright skill)
    Steps:
      1. Navigate to http://localhost:3000
      2. Click: [data-testid="add-daily-task-btn"]
      3. Wait for: dialog[open] visible
      4. Assert: Day of week selector is visible (daily task form)
    Expected Result: Correct modal variant opens
    Evidence: Screenshot
  ```

  **Commit**: YES (groups with Task 3)
  - Message: `feat(ui): add task buttons in section headers`
  - Files: `static/index.html`, `static/app.js`, `static/style.css`
  - Pre-commit: `cargo build`

---

- [x] 8. E2E Tests: CRUD + Layout Verification

  **What to do**:
  - Create `e2e/tests/crud.spec.ts` with comprehensive CRUD tests:
    - Create daily task via modal
    - Edit daily task via modal
    - Delete daily task with confirmation
    - Create deep cleaning task
    - Edit deep cleaning task
    - Delete deep cleaning task
    - Reorder deep cleaning via drag-drop
    - Verify persistence after page reload
  - Create `e2e/tests/desktop-layout.spec.ts`:
    - Verify two-column at 1024px+
    - Verify single-column at <1024px
    - Verify mobile layout at 375px
  - Follow existing test patterns from `e2e/tests/tasks.spec.ts`
  - Use `data-testid` selectors for reliability

  **Must NOT do**:
  - Do NOT modify existing test files
  - Do NOT test features not implemented in this plan

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: Comprehensive E2E test suite requiring careful assertion design
  - **Skills**: [`playwright`]
    - `playwright`: Playwright test patterns and assertions

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 3 (after Tasks 1-7)
  - **Blocks**: None (final task)
  - **Blocked By**: Tasks 1, 2, 3, 4, 5, 6, 7

  **References**:

  **Pattern References**:
  - `e2e/tests/tasks.spec.ts` - Existing test structure, selector patterns
  - `e2e/playwright.config.ts` - Configuration

  **Acceptance Criteria**:

  **Agent-Executed QA Scenarios:**

  ```
  Scenario: All E2E tests pass
    Tool: Bash
    Preconditions: Server running, dependencies installed
    Steps:
      1. cd e2e && npx playwright test
      2. Assert: Exit code is 0
      3. Assert: Output shows all tests passed
    Expected Result: Full green test suite
    Evidence: Test output log saved

  Scenario: New test files exist and have expected tests
    Tool: Bash
    Steps:
      1. ls e2e/tests/crud.spec.ts
      2. Assert: File exists
      3. grep -c "test\(" e2e/tests/crud.spec.ts
      4. Assert: At least 6 test cases
      5. ls e2e/tests/desktop-layout.spec.ts
      6. Assert: File exists
    Expected Result: Test files created with coverage
    Evidence: File listing and grep count
  ```

  **Commit**: YES
  - Message: `test(e2e): add CRUD and desktop layout test suites`
  - Files: `e2e/tests/crud.spec.ts`, `e2e/tests/desktop-layout.spec.ts`
  - Pre-commit: `cd e2e && npx playwright test`

---

## Commit Strategy

| After Task | Message | Files | Verification |
|------------|---------|-------|--------------|
| 1 | `feat(api): add CRUD endpoints for daily tasks` | `src/routes.rs`, `src/db.rs` | `cargo build && cargo clippy` |
| 2 | `feat(api): add CRUD and reorder endpoints for deep cleaning` | `src/routes.rs`, `src/db.rs` | `cargo build && cargo clippy` |
| 3 | `feat(ui): add modal dialog component for task create/edit` | `static/*` | `cargo build` |
| 4 | `feat(ui): add edit and delete buttons to task cards` | `static/*` | `cargo build` |
| 5 | `feat(ui): add two-column desktop layout for wider screens` | `static/style.css` | `cargo build` |
| 6 | `feat(ui): add drag-drop and up/down reordering for deep cleaning` | `static/*` | `cargo build` |
| 7 | `feat(ui): add task buttons in section headers` | `static/*` | `cargo build` |
| 8 | `test(e2e): add CRUD and desktop layout test suites` | `e2e/tests/*` | `npx playwright test` |

---

## Success Criteria

### Verification Commands
```bash
# Build succeeds
cargo build
# Expected: Compiles with no errors

# Linting passes
cargo clippy -- -D warnings
# Expected: No warnings

# E2E tests pass
cd e2e && npx playwright test
# Expected: All tests pass

# Server starts
cargo run &
# Expected: Listening on http://localhost:3000

# API endpoints work
curl -X POST http://localhost:3000/api/tasks \
  -H "Content-Type: application/json" \
  -d '{"name":"Test","day_of_week":1}'
# Expected: HTTP 201 with task object
```

### Final Checklist
- [ ] Can create daily task via modal
- [ ] Can edit daily task via modal
- [ ] Can delete daily task with confirmation
- [ ] Can create deep cleaning task via modal
- [ ] Can edit deep cleaning task via modal
- [ ] Can delete deep cleaning task
- [ ] Can reorder deep cleaning via drag-drop (desktop)
- [ ] Can reorder deep cleaning via up/down buttons (mobile)
- [ ] Two-column layout at ≥1024px viewport
- [ ] Single-column layout at <768px viewport
- [ ] All E2E tests pass
- [ ] Existing toggle behavior unchanged
- [ ] No JS framework added
- [ ] No npm dependencies added
