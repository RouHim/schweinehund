# Schweinehund Feature Bundle: Task Reordering, Daily Reset, Notifications & Fun-Facts

## TL;DR

> **Quick Summary**: 5 zusammenhängende Features für die Putz-App: Erledigte Daily Tasks rutschen ans Ende (+ visuell gedimmt), Deep Cleaning Rotation verifizieren, täglicher Reset um Mitternacht, verbesserte Notifications mit Task-Liste, und Fun-Fact Popup nach letztem Daily Task.
>
> **Deliverables**:
> - Frontend: Task reordering logic + gedimmte Styles + Fun-Fact Popup Dialog
> - Backend: Daily reset scheduler + erweiterte Notification-Inhalte + Day-of-week Bug-Fix
> - Tests: E2E Playwright tests für alle neuen Features
>
> **Estimated Effort**: Medium (1-2 Tage)
> **Parallel Execution**: YES - 3 waves
> **Critical Path**: Task 0 (Bug-Fix) → Task 1 (Reordering) → Task 5 (Fun-Fact)

---

## Context

### Original Request
Der User möchte 5 Features für die Schweinehund Haushalts-App:
1. Erledigte Daily Tasks sollen ans Ende rutschen
2. Deep Cleaning Tasks sollen beim Abhaken ans Ende der Queue rotieren
3. Täglicher Reset der Daily Tasks um Mitternacht
4. Tägliche Notification mit Übersicht der heutigen Tasks + aktuellste Deep Clean Aufgabe
5. Fun-Fact/Witz Popup nach dem letzten erledigten Daily Task (auf Deutsch)

### Interview Summary
**Key Discussions**:
- Reset-Zeitpunkt: Mitternacht (00:00)
- Erledigte Tasks: Ans Ende rutschen + visuell gedimmt (nicht versteckt)
- Day-of-week Bug: Soll mitgefixt werden (DB nutzt 1-7, Code nutzt 0-6)
- Fun-Fact Content: JokeAPI mit `lang=de&safe-mode`

**Research Findings**:
- Deep Cleaning Rotation ist bereits implementiert in `complete_deep_task()` - nur verifizieren
- Daily Tasks haben kein queue_position Feld - Frontend-only Sortierung ist KISS-konform
- `send_daily_reminder()` existiert aber ist nicht im Scheduler eingebunden
- Modal pattern (`<dialog>`) existiert und kann wiederverwendet werden
- Scheduler nutzt aktuell `get_next_monday_midnight()` - muss auf `get_next_midnight()` geändert werden

### Metis Review
**Identified Gaps** (addressed):
- Popup Trigger definiert: Letzter DAILY task (nicht Deep Clean)
- Notification Format festgelegt: Liste mit Task-Namen + Deep Clean Highlight
- Task reordering als Frontend-only (keine Migration nötig)
- JokeAPI Fallback: Silent fail bei Netzwerkfehler (kein Popup)
- Mini-Routines (day=-1): Werden täglich mit zurückgesetzt

---

## Work Objectives

### Core Objective
5 zusammenhängende Features implementieren, die das tägliche Putzerlebnis verbessern: bessere visuelle Unterscheidung, zuverlässiger Reset, informativere Notifications, und Belohnung nach getaner Arbeit.

### Concrete Deliverables
- `src/scheduler.rs`: Daily reset statt weekly + Notification scheduling
- `src/notifications.rs`: Erweiterte Notification mit Task-Liste
- `src/routes.rs` + `src/db.rs`: Day-of-week Bug-Fix (+1 Mapping)
- `static/app.js`: Task reordering + Fun-Fact Popup + gedimmte Styles
- `static/styles.css`: `.task-item.completed` opacity styling
- `e2e/tests/`: Neue Playwright tests für alle Features

### Definition of Done
- [ ] Erledigte Daily Tasks erscheinen am Ende der Liste und sind visuell gedimmt
- [ ] Deep Cleaning Tasks rotieren beim Abhaken (bereits implementiert - verifiziert)
- [ ] Tasks werden täglich um Mitternacht zurückgesetzt
- [ ] Daily Notification enthält Task-Liste + aktuellste Deep Clean Aufgabe
- [ ] Fun-Fact Popup erscheint nach letztem Daily Task (auto-close nach 15s)
- [ ] Alle E2E Tests grün: `cd e2e && npx playwright test`
- [ ] Alle Unit Tests grün: `cargo test`
- [ ] Keine Clippy Warnings: `cargo clippy -- -D warnings`

### Must Have
- Frontend-only Sortierung für Daily Tasks (keine DB-Migration)
- JokeAPI mit `lang=de&safe-mode` für familienfreundliche Inhalte
- Graceful degradation bei API-Fehlern (silent fail)
- Konsistente Patterns mit existierendem Code

### Must NOT Have (Guardrails)
- KEINE neue DB-Migration für queue_position bei Daily Tasks
- KEINE Joke-Caching oder History-Tracking
- KEINE Animations/Transitions beim Reordering (KISS)
- KEINE Notification-Preferences UI
- KEINE Retry-Logic für JokeAPI
- KEINE Änderungen am Deep Cleaning rotation logic (nur verifizieren)
- KEINE Timezone-Konfiguration (Server local time)

---

## Verification Strategy

> **UNIVERSAL RULE: ZERO HUMAN INTERVENTION**
>
> ALL tasks in this plan MUST be verifiable WITHOUT any human action.
> All verification is executed by the agent using tools (Playwright, Bash, curl).

### Test Decision
- **Infrastructure exists**: YES (Playwright E2E + Cargo test)
- **Automated tests**: TDD (Tests-after for integration, verify behavior)
- **Framework**: Playwright (E2E), Cargo test (unit)

### Agent-Executed QA Scenarios (MANDATORY — ALL tasks)

Every task includes detailed QA scenarios using:
- **Frontend/UI**: Playwright (playwright skill)
- **API/Backend**: Bash (curl)
- **TUI/CLI**: Not applicable

---

## Execution Strategy

### Parallel Execution Waves

```
Wave 1 (Start Immediately):
├── Task 0: Day-of-week Bug-Fix [no dependencies]
├── Task 2: Daily Reset Scheduler [no dependencies]
└── Task 3: Enhanced Notifications [no dependencies]

Wave 2 (After Wave 1):
├── Task 1: Daily Task Reordering [depends: 0 - needs correct day mapping]
└── Task 4: Verify Deep Cleaning [depends: 0]

Wave 3 (After Wave 2):
└── Task 5: Fun-Fact Popup [depends: 1 - needs reordering to detect "last task"]
```

### Dependency Matrix

| Task | Depends On | Blocks | Can Parallelize With |
|------|------------|--------|---------------------|
| 0 | None | 1, 4 | 2, 3 |
| 1 | 0 | 5 | 4 |
| 2 | None | None | 0, 3 |
| 3 | None | None | 0, 2 |
| 4 | 0 | None | 1 |
| 5 | 1 | None | None (final) |

### Agent Dispatch Summary

| Wave | Tasks | Recommended Agents |
|------|-------|-------------------|
| 1 | 0, 2, 3 | delegate_task(category="quick") for each |
| 2 | 1, 4 | delegate_task(category="quick") for 1, category="quick" for 4 |
| 3 | 5 | delegate_task(category="unspecified-low") |

---

## TODOs

---

- [ ] 0. Fix Day-of-Week Mapping Bug

  **What to do**:
  - In `src/routes.rs`: Bei `handle_get_today_tasks` den Default-Wert von `num_days_from_monday()` auf `num_days_from_monday() + 1` ändern
  - In `src/notifications.rs`: Bei `send_daily_reminder` die day_of_week Berechnung auf `+ 1` ändern
  - In `src/db.rs` Tests: Verifizieren dass Tests weiterhin 1-7 Mapping nutzen

  **Must NOT do**:
  - DB-Werte ändern (Migration auf 0-6 wäre größerer Change)
  - Andere Stellen als routes.rs und notifications.rs ändern

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Kleine, fokussierte Änderung in 2 Dateien
  - **Skills**: [`git-master`]
    - `git-master`: Für atomaren Commit nach Bug-Fix

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 2, 3)
  - **Blocks**: Tasks 1, 4
  - **Blocked By**: None (can start immediately)

  **References**:

  **Pattern References**:
  - `src/routes.rs:handle_get_today_tasks` - Hier wird `num_days_from_monday()` als Default verwendet
  - `src/notifications.rs:send_daily_reminder` - day_of_week Berechnung für Notification

  **API/Type References**:
  - `chrono::Weekday::num_days_from_monday()` - Gibt 0 für Montag, 6 für Sonntag zurück
  - DB Schema: `day_of_week` in `daily_tasks` verwendet 1 (Mo) bis 7 (So), -1 für Mini-Routines

  **Test References**:
  - `src/db.rs:tests::test_get_today_tasks` - Nutzt 1 für Montag (korrekt)

  **WHY Each Reference Matters**:
  - `routes.rs` ist der Haupt-Einstiegspunkt für `/api/tasks/today` - hier muss +1 addiert werden
  - `notifications.rs` nutzt die gleiche Logik für tägliche Reminders - konsistent halten

  **Acceptance Criteria**:

  **Agent-Executed QA Scenarios:**

  ```
  Scenario: Monday tasks appear on Monday
    Tool: Bash (curl)
    Preconditions: Server running on localhost:3000, Monday tasks exist in DB with day_of_week=1
    Steps:
      1. curl -s http://localhost:3000/api/tasks/today | jq '.[] | select(.day_of_week == 1) | .name'
      2. Assert: At least one Monday task returned
      3. curl -s http://localhost:3000/api/tasks/today | jq 'length'
      4. Assert: Count includes mini-routines (day_of_week=-1) + Monday tasks
    Expected Result: Monday tasks correctly returned on Monday
    Evidence: Response body captured

  Scenario: Mini-routines appear every day
    Tool: Bash (curl)
    Preconditions: Mini-routine tasks exist with day_of_week=-1
    Steps:
      1. curl -s http://localhost:3000/api/tasks/today | jq '.[] | select(.day_of_week == -1)'
      2. Assert: Mini-routines present in response
    Expected Result: Mini-routines always included
    Evidence: Response body captured
  ```

  **Commit**: YES
  - Message: `fix(api): correct day-of-week mapping from 0-6 to 1-7`
  - Files: `src/routes.rs`, `src/notifications.rs`
  - Pre-commit: `cargo test && cargo clippy -- -D warnings`

---

- [ ] 1. Daily Task Reordering (Completed Tasks to Bottom + Dimmed)

  **What to do**:
  - In `static/app.js`: `renderTasks()` Funktion erweitern um Tasks vor dem Rendern zu sortieren:
    - Uncompleted tasks zuerst, dann completed tasks
    - Innerhalb jeder Gruppe: Original-Reihenfolge beibehalten (stable sort)
  - In `static/styles.css`: `.task-item.completed` Styling verstärken:
    - `opacity: 0.5` (aktuell 0.65 - mehr dimmen)
    - Optional: leicht grauer Hintergrund

  **Must NOT do**:
  - DB-Migration für queue_position hinzufügen
  - API-Response ändern
  - Animations oder Transitions hinzufügen

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Frontend-only Änderung, klar definiert
  - **Skills**: [`frontend-ui-ux`]
    - `frontend-ui-ux`: Für CSS Styling und UX-konsistentes Verhalten

  **Parallelization**:
  - **Can Run In Parallel**: NO (depends on Task 0)
  - **Parallel Group**: Wave 2 (with Task 4)
  - **Blocks**: Task 5
  - **Blocked By**: Task 0

  **References**:

  **Pattern References**:
  - `static/app.js:renderTasks()` - Aktuelle Render-Logik (lines ~239-286)
  - `static/app.js:handleTaskToggle()` - Optimistic UI update pattern (lines ~14-63)
  - `static/app.js:renderDeepCleaning()` - Ähnliches Pattern für Deep Cleaning

  **API/Type References**:
  - Task object: `{ id, name, description, zone, day_of_week, completed, completed_at }`

  **Test References**:
  - `e2e/tests/tasks.spec.ts` - Existierende E2E Tests für Task-Interaktion

  **WHY Each Reference Matters**:
  - `renderTasks()` ist der Ort wo Sortierung eingefügt werden muss - vor dem `.map()`
  - `handleTaskToggle()` zeigt das Optimistic UI Pattern - nach Toggle muss re-render erfolgen

  **Acceptance Criteria**:

  **Agent-Executed QA Scenarios:**

  ```
  Scenario: Completed daily task moves to bottom of list
    Tool: Playwright (playwright skill)
    Preconditions: Dev server running on localhost:3000, at least 3 daily tasks exist
    Steps:
      1. Navigate to: http://localhost:3000
      2. Wait for: .task-item visible (timeout: 5s)
      3. Note: first task's data-task-id attribute value
      4. Click: first .task-item .task-checkbox
      5. Wait for: 500ms (optimistic update)
      6. Assert: .task-item:first-child does NOT have the noted data-task-id
      7. Assert: .task-item.completed:last-child HAS the noted data-task-id
      8. Screenshot: .sisyphus/evidence/task-1-reorder-after-complete.png
    Expected Result: Completed task now at bottom of list
    Evidence: .sisyphus/evidence/task-1-reorder-after-complete.png

  Scenario: Completed task has dimmed styling
    Tool: Playwright (playwright skill)
    Preconditions: At least one completed task visible
    Steps:
      1. Navigate to: http://localhost:3000
      2. Click: first uncompleted .task-checkbox
      3. Wait for: .task-item.completed visible
      4. Assert: .task-item.completed has CSS opacity <= 0.6
      5. Screenshot: .sisyphus/evidence/task-1-dimmed-styling.png
    Expected Result: Completed task visually dimmed
    Evidence: .sisyphus/evidence/task-1-dimmed-styling.png

  Scenario: Page refresh maintains sort order
    Tool: Playwright (playwright skill)
    Preconditions: Mix of completed and uncompleted tasks
    Steps:
      1. Navigate to: http://localhost:3000
      2. Complete first task
      3. Reload page
      4. Assert: All .task-item:not(.completed) appear before .task-item.completed
    Expected Result: Sort order persists after refresh
    Evidence: Response body captured

  Scenario: Uncompleting task moves it back to top
    Tool: Playwright (playwright skill)
    Preconditions: At least one completed task at bottom
    Steps:
      1. Navigate to: http://localhost:3000
      2. Note: data-task-id of last .task-item.completed
      3. Click: .task-item.completed:last-child .task-checkbox
      4. Wait for: 500ms
      5. Assert: Task with noted ID is no longer .completed
      6. Assert: Task is no longer last in list (moved up)
    Expected Result: Uncompleted task returns to correct position
    Evidence: Screenshot captured
  ```

  **Commit**: YES
  - Message: `feat(ui): reorder completed daily tasks to bottom with dimmed styling`
  - Files: `static/app.js`, `static/styles.css`
  - Pre-commit: `cargo clippy -- -D warnings`

---

- [ ] 2. Daily Reset Scheduler (Midnight)

  **What to do**:
  - In `src/scheduler.rs`: 
    - `get_next_monday_midnight()` zu `get_next_midnight()` ändern/hinzufügen
    - Scheduler loop: Statt Monday 00:00 → jeden Tag 00:00
    - `startup_reconciliation()`: Check ob heute schon Reset lief (statt diese Woche)
  - In `src/db.rs`:
    - `reset_daily_tasks()` bleibt unverändert (setzt alle completed=0)
    - `get_last_reset()` / `set_last_reset()` bleiben unverändert

  **Must NOT do**:
  - Deep Cleaning Reset-Cadence ändern (bleibt weekly/bei Completion)
  - Timezone-Konfiguration hinzufügen
  - Zusätzliche State-Tracking Tabellen

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Scheduler-Logik ändern, klar definiertes Pattern
  - **Skills**: []
    - Keine speziellen Skills nötig - Standard Rust

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 0, 3)
  - **Blocks**: None
  - **Blocked By**: None

  **References**:

  **Pattern References**:
  - `src/scheduler.rs:get_next_monday_midnight()` - Aktuelle Logik für nächsten Montag
  - `src/scheduler.rs:start_scheduler()` - Haupt-Loop mit sleep_until Pattern
  - `src/scheduler.rs:startup_reconciliation()` - Catch-up Reset bei Server-Start
  - `src/scheduler.rs:execute_reset()` - Ruft db::reset_daily_tasks auf

  **API/Type References**:
  - `chrono::Local::now()` - Lokale Zeit für Scheduler
  - `tokio::time::sleep_until()` - Async sleep Pattern

  **Test References**:
  - `src/db.rs:tests::test_reset_daily_tasks` - Verifiziert DB-Reset Logik

  **WHY Each Reference Matters**:
  - `get_next_monday_midnight()` muss zu `get_next_midnight()` werden - Kernänderung
  - `startup_reconciliation()` muss prüfen ob HEUTE schon reset lief, nicht diese Woche

  **Acceptance Criteria**:

  **Agent-Executed QA Scenarios:**

  ```
  Scenario: Reset function clears all daily task completion flags
    Tool: Bash (curl)
    Preconditions: Server running, some tasks marked completed
    Steps:
      1. curl -X POST http://localhost:3000/api/tasks/1/toggle (mark complete)
      2. curl -s http://localhost:3000/api/tasks/today | jq '[.[] | select(.completed==true)] | length'
      3. Assert: At least 1 completed task
      4. curl -X POST http://localhost:3000/api/debug/reset
      5. curl -s http://localhost:3000/api/tasks/today | jq '[.[] | select(.completed==true)] | length'
      6. Assert: 0 completed tasks
    Expected Result: All daily tasks reset to incomplete
    Evidence: Response bodies captured

  Scenario: Scheduler calculates next midnight correctly
    Tool: Bash (cargo test)
    Preconditions: None
    Steps:
      1. cargo test test_get_next_midnight -- --nocapture
      2. Assert: Test passes
      3. Assert: Output shows next midnight is within 24 hours
    Expected Result: get_next_midnight returns correct instant
    Evidence: Test output captured
  ```

  **Commit**: YES
  - Message: `feat(scheduler): change from weekly to daily reset at midnight`
  - Files: `src/scheduler.rs`
  - Pre-commit: `cargo test && cargo clippy -- -D warnings`

---

- [ ] 3. Enhanced Daily Notification with Task List

  **What to do**:
  - In `src/notifications.rs`: `send_daily_reminder()` erweitern:
    - Task-Namen auflisten (nicht nur Count)
    - Deep Cleaning: Nur die erste Aufgabe (niedrigste queue_position) hinzufügen
    - Format:
      ```
      🧹 Heute zu tun:
      • [Task 1 Name]
      • [Task 2 Name]
      
      🔷 Deep Clean: [Name der aktuellsten Aufgabe]
      ```
  - In `src/db.rs`: `get_top_deep_cleaning_task()` Funktion hinzufügen (SELECT ... ORDER BY queue_position LIMIT 1)
  - Notification in Scheduler einbinden (nach Reset oder zu konfigurierter Zeit)

  **Must NOT do**:
  - Notification-Preferences UI bauen
  - Mehrere Deep Cleaning Tasks auflisten
  - Notification-History tracken

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Erweitern existierender Notification-Logik
  - **Skills**: []
    - Keine speziellen Skills nötig

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 0, 2)
  - **Blocks**: None
  - **Blocked By**: None

  **References**:

  **Pattern References**:
  - `src/notifications.rs:send_daily_reminder()` - Aktuelle Notification-Logik
  - `src/notifications.rs:NtfyClient::send_reminder()` - Low-level ntfy Send
  - `src/db.rs:get_deep_cleaning_queue()` - Deep Cleaning Queue Query

  **API/Type References**:
  - `DailyTask { id, name, description, zone, day_of_week, completed, completed_at }`
  - `DeepCleaningTask { id, name, description, zone, queue_position, completed_at }`
  - ntfy.sh Headers: Title, Priority, Tags

  **WHY Each Reference Matters**:
  - `send_daily_reminder()` baut aktuell nur Count-Message - muss erweitert werden
  - `get_deep_cleaning_queue()` sortiert nach queue_position - erste = aktuellste

  **Acceptance Criteria**:

  **Agent-Executed QA Scenarios:**

  ```
  Scenario: Test notification contains task names
    Tool: Bash (curl to trigger + capture)
    Preconditions: Server running, ntfy topic accessible, tasks exist
    Steps:
      1. Set up ntfy listener or use curl to trigger test
      2. curl -X POST http://localhost:3000/api/debug/notify-daily (new endpoint or extend existing)
      3. Check ntfy topic for message
      4. Assert: Message contains task names (not just count)
      5. Assert: Message contains "Deep Clean:" section
    Expected Result: Notification includes task list + deep clean
    Evidence: Notification body captured

  Scenario: Notification includes only top deep cleaning task
    Tool: Bash (cargo test)
    Preconditions: None
    Steps:
      1. cargo test test_get_top_deep_cleaning_task -- --nocapture
      2. Assert: Returns single task with lowest queue_position
    Expected Result: Only most current deep clean task returned
    Evidence: Test output captured
  ```

  **Commit**: YES
  - Message: `feat(notifications): include task list and top deep cleaning in daily reminder`
  - Files: `src/notifications.rs`, `src/db.rs`
  - Pre-commit: `cargo test && cargo clippy -- -D warnings`

---

- [ ] 4. Verify Deep Cleaning Rotation (Test Only)

  **What to do**:
  - E2E Test schreiben der verifiziert:
    - Deep Cleaning Task abhaken → Task rutscht ans Ende der Queue
    - Queue-Position wird korrekt aktualisiert
  - Unit Test falls nicht vorhanden:
    - `complete_deep_task()` setzt queue_position = max + 1

  **Must NOT do**:
  - Bestehende Deep Cleaning Logik ändern
  - Neue Features hinzufügen

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Nur Tests schreiben, keine Code-Änderungen
  - **Skills**: [`playwright`]
    - `playwright`: Für E2E Test Erstellung

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Task 1)
  - **Blocks**: None
  - **Blocked By**: Task 0 (needs correct day mapping)

  **References**:

  **Pattern References**:
  - `src/db.rs:complete_deep_task()` - Setzt queue_position = max + 1
  - `static/app.js:handleDeepCleaningToggle()` - Frontend Handler
  - `e2e/tests/` - Existierende Test-Patterns

  **Test References**:
  - `src/db.rs:tests` - Bestehende Unit Test Patterns

  **WHY Each Reference Matters**:
  - `complete_deep_task()` ist die Kernfunktion - muss verifiziert werden dass sie korrekt rotiert

  **Acceptance Criteria**:

  **Agent-Executed QA Scenarios:**

  ```
  Scenario: Deep cleaning task rotates to end of queue on complete
    Tool: Playwright (playwright skill)
    Preconditions: Server running, at least 3 deep cleaning tasks in queue
    Steps:
      1. Navigate to: http://localhost:3000
      2. Scroll to deep cleaning section
      3. Note: Name of first deep cleaning task
      4. Click: first deep cleaning .task-checkbox
      5. Wait for: 1s (refetch happens)
      6. Assert: First deep cleaning task has DIFFERENT name than noted
      7. Assert: Last deep cleaning task has SAME name as noted
      8. Screenshot: .sisyphus/evidence/task-4-deep-clean-rotation.png
    Expected Result: Completed deep clean task moved to end
    Evidence: .sisyphus/evidence/task-4-deep-clean-rotation.png

  Scenario: Queue positions are correctly updated in DB
    Tool: Bash (curl)
    Preconditions: At least 2 deep cleaning tasks
    Steps:
      1. curl -s http://localhost:3000/api/deep-cleaning | jq '.[0]'
      2. Note: id and queue_position of first task
      3. curl -X POST http://localhost:3000/api/deep-cleaning/{id}/complete
      4. curl -s http://localhost:3000/api/deep-cleaning | jq '.[-1]'
      5. Assert: Last task has the noted id
      6. Assert: Last task has highest queue_position
    Expected Result: DB queue_position correctly updated
    Evidence: Response bodies captured
  ```

  **Commit**: YES (tests only)
  - Message: `test(deep-clean): verify rotation behavior on task completion`
  - Files: `e2e/tests/deep-cleaning.spec.ts` (new or extend existing)
  - Pre-commit: `cd e2e && npx playwright test`

---

- [ ] 5. Fun-Fact Popup After Last Daily Task

  **What to do**:
  - In `static/app.js`:
    - Nach `handleTaskToggle()`: Wenn ALLE daily tasks completed → JokeAPI fetch
    - Neues `<dialog id="fun-fact-modal">` erstellen oder existierendes erweitern
    - Auto-close nach 15 Sekunden (setTimeout)
    - Manuelles Schließen via Button
  - In `static/index.html`: Dialog Element hinzufügen
  - In `static/styles.css`: Fun-Fact Modal Styling

  **JokeAPI Integration**:
  - URL: `https://v2.jokeapi.dev/joke/Any?lang=de&safe-mode&type=single`
  - Response: `{ joke: "..." }` oder `{ setup: "...", delivery: "..." }`
  - Fallback bei Fehler: Kein Popup (silent fail)

  **Must NOT do**:
  - Jokes cachen oder History tracken
  - Categories/Preferences hinzufügen
  - Retry-Logic bei API-Fehler
  - Popup bei Deep Cleaning completion (nur Daily Tasks)

  **Recommended Agent Profile**:
  - **Category**: `unspecified-low`
    - Reason: Frontend Feature mit API-Integration, aber klar definiert
  - **Skills**: [`frontend-ui-ux`, `playwright`]
    - `frontend-ui-ux`: Für Modal UX und Styling
    - `playwright`: Für E2E Test des Popups

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 3 (final)
  - **Blocks**: None
  - **Blocked By**: Task 1 (needs reordering to detect last task)

  **References**:

  **Pattern References**:
  - `static/app.js:openModal()` / `closeModal()` - Existierendes Modal Pattern
  - `static/app.js:handleTaskToggle()` - Wo der Check eingefügt werden muss
  - `static/index.html:<dialog id="task-modal">` - Existierendes Dialog Element
  - `static/styles.css:dialog` - Modal Styling Patterns

  **External References**:
  - JokeAPI Docs: https://v2.jokeapi.dev/
  - Endpoint: `https://v2.jokeapi.dev/joke/Any?lang=de&safe-mode`

  **WHY Each Reference Matters**:
  - `openModal()`/`closeModal()` Pattern wiederverwenden für konsistente UX
  - `handleTaskToggle()` ist der Ort wo der "alle fertig?" Check eingefügt werden muss

  **Acceptance Criteria**:

  **Agent-Executed QA Scenarios:**

  ```
  Scenario: Fun-fact popup appears after completing all daily tasks
    Tool: Playwright (playwright skill)
    Preconditions: Server running, at least 2 daily tasks exist, none completed
    Steps:
      1. Navigate to: http://localhost:3000
      2. Wait for: .task-item visible
      3. Count: uncompleted daily tasks
      4. Complete all daily tasks one by one (click each .task-checkbox)
      5. Wait for: dialog#fun-fact-modal[open] (timeout: 5s)
      6. Assert: dialog#fun-fact-modal is visible
      7. Assert: dialog contains text (joke content)
      8. Screenshot: .sisyphus/evidence/task-5-fun-fact-popup.png
    Expected Result: Fun-fact popup appears with German joke
    Evidence: .sisyphus/evidence/task-5-fun-fact-popup.png

  Scenario: Popup auto-closes after 15 seconds
    Tool: Playwright (playwright skill)
    Preconditions: Fun-fact popup is open
    Steps:
      1. Complete all daily tasks to trigger popup
      2. Wait for: dialog#fun-fact-modal[open]
      3. Note: current time
      4. Wait for: dialog#fun-fact-modal:not([open]) (timeout: 20s)
      5. Note: time when dialog closed
      6. Assert: elapsed time is approximately 15s (±2s tolerance)
    Expected Result: Popup auto-closes after 15 seconds
    Evidence: Timing captured

  Scenario: Popup can be closed manually
    Tool: Playwright (playwright skill)
    Preconditions: Fun-fact popup is open
    Steps:
      1. Trigger popup by completing all tasks
      2. Wait for: dialog#fun-fact-modal[open]
      3. Click: dialog#fun-fact-modal [data-close-modal] button
      4. Assert: dialog#fun-fact-modal:not([open])
    Expected Result: Manual close works
    Evidence: Screenshot captured

  Scenario: No popup when some daily tasks remain
    Tool: Playwright (playwright skill)
    Preconditions: At least 3 daily tasks, none completed
    Steps:
      1. Navigate to: http://localhost:3000
      2. Complete only the first daily task
      3. Wait: 2s
      4. Assert: dialog#fun-fact-modal is NOT visible
    Expected Result: No popup when tasks remain
    Evidence: Page state captured

  Scenario: API failure results in no popup (graceful degradation)
    Tool: Playwright (playwright skill)
    Preconditions: Network request to jokeapi.dev will fail (mock or block)
    Steps:
      1. Use page.route to block requests to *jokeapi.dev*
      2. Complete all daily tasks
      3. Wait: 3s
      4. Assert: No error dialog or alert shown
      5. Assert: App continues to function normally
    Expected Result: Silent failure, no popup
    Evidence: Console logs captured
  ```

  **Commit**: YES
  - Message: `feat(ui): add fun-fact popup after completing all daily tasks`
  - Files: `static/app.js`, `static/index.html`, `static/styles.css`
  - Pre-commit: `cargo clippy -- -D warnings`

---

## Commit Strategy

| After Task | Message | Files | Verification |
|------------|---------|-------|--------------|
| 0 | `fix(api): correct day-of-week mapping from 0-6 to 1-7` | routes.rs, notifications.rs | cargo test |
| 1 | `feat(ui): reorder completed daily tasks to bottom with dimmed styling` | app.js, styles.css | playwright test |
| 2 | `feat(scheduler): change from weekly to daily reset at midnight` | scheduler.rs | cargo test |
| 3 | `feat(notifications): include task list and top deep cleaning in daily reminder` | notifications.rs, db.rs | cargo test |
| 4 | `test(deep-clean): verify rotation behavior on task completion` | deep-cleaning.spec.ts | playwright test |
| 5 | `feat(ui): add fun-fact popup after completing all daily tasks` | app.js, index.html, styles.css | playwright test |

---

## Success Criteria

### Verification Commands
```bash
# All Rust tests pass
cargo test

# No Clippy warnings
cargo clippy -- -D warnings

# All E2E tests pass
cd e2e && npx playwright test

# Server starts without errors
cargo run &
sleep 3
curl -s http://localhost:3000/api/tasks/today | jq 'length'  # Expected: > 0
```

### Final Checklist
- [ ] Erledigte Daily Tasks erscheinen am Ende der Liste (visuell gedimmt)
- [ ] Deep Cleaning Rotation funktioniert (E2E verifiziert)
- [ ] Scheduler resetet täglich um Mitternacht
- [ ] Daily Notification enthält Task-Liste + aktuellste Deep Clean Aufgabe
- [ ] Fun-Fact Popup erscheint nach letztem Daily Task (auto-close 15s)
- [ ] Day-of-week Bug behoben (korrekte Tasks an korrektem Tag)
- [ ] Keine neuen Clippy Warnings
- [ ] Alle Tests grün
