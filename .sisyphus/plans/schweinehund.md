# Schweinehund - Putzplan PWA

## TL;DR

> **Quick Summary**: Eine self-hosted Putzplan-App als PWA mit PocketBase Backend. Tägliche Aufgaben setzen sich wöchentlich zurück, größere Aufgaben rotieren. Push-Notifications via ntfy.sh.
> 
> **Deliverables**:
> - Funktionierende PWA unter `https://schweinehund.local:8080`
> - Docker Compose Setup für TrueNAS Scale
> - Editierbare Aufgaben, Zonen und Wochentage
> - Push-Notifications um 09:00 und bei Events
> - Verspieltes, farbenfrohe UI mit Schweinehund-Logo
> 
> **Estimated Effort**: Large (3-5 Tage)
> **Parallel Execution**: YES - 3 Waves
> **Critical Path**: Task 1 → Task 3 → Task 5 → Task 8 → Task 11

---

## Context

### Original Request
Benutzer möchte eine lokale Putzplan-Web-App namens "Schweinehund" mit:
- Abhakbare Aufgaben (täglich mit Reset, größere mit Rotation)
- Push-Benachrichtigungen aufs Handy
- Mobile-first, lokal gehostet
- Keine Authentifizierung
- Fancy UI

### Interview Summary
**Key Discussions**:
- **Tech Stack**: PocketBase + HTMX/Alpine.js (empfohlen wegen Einfachheit)
- **Reset-Zeit**: Montag 00:00 Uhr
- **Benachrichtigungen**: Alle 4 Typen (täglich, Zonen, Wochenende, Reset)
- **Editierbarkeit**: Voll editierbar (Zonen + Wochentage + Aufgaben)
- **Notification-Lösung**: ntfy.sh (self-hosted) für echte Push
- **iOS**: Nur Android optimiert
- **Offline**: Read-only cached

**Research Findings**:
- PocketBase nutzt SSE für Realtime (einfacher als WebSocket)
- Service Worker muss `/api/realtime` ignorieren
- mkcert für lokale HTTPS-Zertifikate
- alpine-morph Extension für HTMX+Alpine Integration

### Metis Review
**Identified Gaps** (addressed):
- 09:00 Notification braucht Server-Side + ntfy.sh → ntfy.sh Integration
- iOS Limitations → Scope auf Android reduziert
- Offline-Verhalten unklar → Read-only cached definiert

---

## Work Objectives

### Core Objective
Eine voll funktionsfähige, self-hosted Putzplan-PWA die auf Android-Handys als App installiert werden kann, tägliche Aufgaben mit wöchentlichem Reset verwaltet, und Push-Notifications via ntfy.sh sendet.

### Concrete Deliverables
- `docker-compose.yml` für TrueNAS Scale Deployment
- PocketBase mit konfiguriertem Schema
- PWA Frontend mit HTMX + Alpine.js
- ntfy.sh Container für Push-Notifications
- Service Worker für Offline-Support
- Schweinehund Logo (SVG + Favicon)
- Scheduler für automatischen Reset und Notifications

### Definition of Done
- [ ] App erreichbar unter `https://schweinehund.local:8080`
- [ ] Aufgaben können abgehakt werden
- [ ] Reset am Montag 00:00 setzt tägliche Aufgaben zurück
- [ ] Erledigte große Aufgaben wandern ans Ende
- [ ] Push-Notification auf Android funktioniert
- [ ] App kann auf Home Screen installiert werden

### Must Have
- Docker Compose Deployment
- PocketBase Backend mit SQLite
- HTMX + Alpine.js Frontend
- ntfy.sh für Push
- Service Worker für PWA
- Responsive, mobile-first Design
- Verspieltes Farbschema

### Must NOT Have (Guardrails)
- Keine Authentifizierung/Login
- Keine Multi-User Trennung
- Keine Cloud-Dienste
- Kein komplexes State-Management (Redux etc.)
- Keine Offline-Edits (nur read-only cache)
- Keine iOS-spezifischen Optimierungen
- Keine Standard Web Push API (nur ntfy.sh)

---

## Verification Strategy (MANDATORY)

### Test Decision
- **Infrastructure exists**: NEIN (Greenfield)
- **User wants tests**: NEIN (Minimal testing)
- **Framework**: none
- **QA approach**: Manuelle Verifikation + automatisierte Checks

### Automated Verification
Jede Task hat Bash-Commands zur Verifikation.

---

## Execution Strategy

### Parallel Execution Waves

```
Wave 1 (Start Immediately):
├── Task 1: Project Setup (Docker, Struktur)
├── Task 2: ntfy.sh Container Setup
└── Task 12: Logo/Icon Design

Wave 2 (After Wave 1):
├── Task 3: PocketBase Schema
├── Task 4: Service Worker + Manifest
├── Task 13: UI Theme & CSS

Wave 3 (After Tasks 3, 4):
├── Task 5: Frontend Base Layout
├── Task 6: Task List Component
├── Task 7: Task Edit Modal
├── Task 8: Zone Management

Wave 4 (After Wave 3):
├── Task 9: Notification Integration (ntfy)
├── Task 10: Weekly Reset Scheduler
└── Task 11: Rotation Logic

Wave 5 (Final):
└── Task 14: Docker Compose & Deployment
```

### Dependency Matrix

| Task | Depends On | Blocks | Can Parallelize With |
|------|------------|--------|---------------------|
| 1 | None | 3, 4, 5 | 2, 12 |
| 2 | None | 9 | 1, 12 |
| 3 | 1 | 5, 6, 7, 8, 10, 11 | 4, 13 |
| 4 | 1 | 5 | 3, 13 |
| 5 | 3, 4 | 6, 7, 8, 9 | 13 |
| 6 | 5 | 11 | 7, 8 |
| 7 | 5 | - | 6, 8 |
| 8 | 5 | - | 6, 7 |
| 9 | 2, 5 | - | 10, 11 |
| 10 | 3 | - | 9, 11 |
| 11 | 3, 6 | - | 9, 10 |
| 12 | None | 5 | 1, 2 |
| 13 | 1 | 5 | 3, 4 |
| 14 | ALL | - | None |

---

## TODOs

### Wave 1: Foundation

- [x] 1. Project Setup & Docker Structure

  **What to do**:
  - Erstelle Projektstruktur:
    ```
    schweinehund/
    ├── docker-compose.yml
    ├── pocketbase/
    │   └── pb_hooks/
    ├── frontend/
    │   ├── index.html
    │   ├── manifest.json
    │   ├── sw.js
    │   ├── css/
    │   └── js/
    └── ntfy/
    ```
  - Basis docker-compose.yml mit PocketBase und nginx/caddy
  - mkcert Zertifikate für `schweinehund.local`

  **Must NOT do**:
  - Keine komplexe Build-Pipeline
  - Kein Webpack/Vite (vanilla JS reicht)

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Straightforward file creation and Docker setup
  - **Skills**: [`git-master`]
    - `git-master`: For initial git setup if needed

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 2, 12)
  - **Blocks**: Tasks 3, 4, 5, 13
  - **Blocked By**: None

  **References**:
  
  **External References**:
  - PocketBase Docker: https://pocketbase.io/docs/going-to-production/
  - mkcert: https://github.com/FiloSottile/mkcert

  **Acceptance Criteria**:
  ```bash
  # Verzeichnisstruktur existiert
  ls -la schweinehund/
  # Expected: docker-compose.yml, pocketbase/, frontend/, ntfy/

  # Docker Compose ist valide
  docker compose config
  # Expected: No errors, services listed
  ```

  **Commit**: YES
  - Message: `feat(setup): initial project structure with Docker`
  - Files: `docker-compose.yml`, `pocketbase/`, `frontend/`, `ntfy/`

---

- [x] 2. ntfy.sh Container Setup

  **What to do**:
  - ntfy.sh Container in docker-compose.yml hinzufügen
  - Konfiguration für lokales Netzwerk (kein Auth)
  - Test-Topic "schweinehund" erstellen
  - Dokumentation wie man ntfy auf Android einrichtet

  **Must NOT do**:
  - Keine Authentifizierung für ntfy
  - Keine externen ntfy.sh Server

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Container configuration is straightforward
  - **Skills**: []
    - No special skills needed

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 12)
  - **Blocks**: Task 9
  - **Blocked By**: None

  **References**:
  
  **External References**:
  - ntfy.sh Self-hosting: https://docs.ntfy.sh/install/
  - ntfy Docker: https://docs.ntfy.sh/install/#docker

  **Acceptance Criteria**:
  ```bash
  # ntfy Container läuft
  docker compose up -d ntfy
  curl -s http://localhost:8090/schweinehund/json | head -1
  # Expected: Connection established (or empty JSON line)

  # Test-Notification senden
  curl -d "Test message" http://localhost:8090/schweinehund
  # Expected: {"id":"...", "event":"message", ...}
  ```

  **Commit**: YES (groups with 1)
  - Message: `feat(ntfy): add ntfy.sh container for push notifications`
  - Files: `docker-compose.yml`, `ntfy/`

---

- [x] 12. Logo/Icon Design

  **What to do**:
  - Schweinehund Logo erstellen (verspielt, farbenfroh)
  - Konzept: Fauler/müder Hund mit Putzutensilien (Besen, Eimer)
  - SVG für Web, PNG für Favicon/PWA Icons
  - Farben: Warme Töne (Orange, Rosa, evtl. Türkis Akzente)
  - Icon Sizes: 192x192, 512x512 für PWA

  **Must NOT do**:
  - Keine fotorealistischen Grafiken
  - Keine externen Icon-Services

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
    - Reason: Creative visual design task
  - **Skills**: [`frontend-design`]
    - `frontend-design`: For high-quality visual design

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 2)
  - **Blocks**: Task 5
  - **Blocked By**: None

  **References**:
  
  **Pattern References**:
  - Flat design icons: Duolingo mascot style
  - Color palette: Warm, inviting household app colors

  **Acceptance Criteria**:
  ```bash
  # Logo-Dateien existieren
  ls -la frontend/assets/
  # Expected: logo.svg, icon-192.png, icon-512.png, favicon.ico

  # SVG ist valide
  file frontend/assets/logo.svg
  # Expected: SVG Scalable Vector Graphics image
  ```

  **Commit**: YES
  - Message: `feat(design): add Schweinehund logo and PWA icons`
  - Files: `frontend/assets/*`

---

### Wave 2: Backend & PWA Foundation

- [x] 3. PocketBase Schema Design

  **What to do**:
  - Collections in PocketBase erstellen:
    - `tasks`: id, name, emoji, zone_id, is_daily, completed, completed_at, sort_order
    - `zones`: id, name, emoji, weekday (0-6), color
    - `settings`: id, key, value (für notification times etc.)
  - API Rules: Keine Auth, alles public
  - PocketBase Hooks Ordner vorbereiten
  - Beispiel-Daten aus dem Putzplan einfügen

  **Must NOT do**:
  - Keine User-Collection
  - Keine komplexen Relationen

  **Recommended Agent Profile**:
  - **Category**: `unspecified-low`
    - Reason: Database schema design, straightforward
  - **Skills**: []
    - No special skills needed

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 4, 13)
  - **Blocks**: Tasks 5, 6, 7, 8, 10, 11
  - **Blocked By**: Task 1

  **References**:
  
  **External References**:
  - PocketBase Collections: https://pocketbase.io/docs/collections/
  - PocketBase API Rules: https://pocketbase.io/docs/api-rules-and-filters/

  **Data Model**:
  ```
  tasks:
    - id (auto)
    - name: string (required)
    - emoji: string (optional, default: "✓")
    - zone: relation -> zones (optional, for daily tasks)
    - is_daily: bool (default: true)
    - completed: bool (default: false)
    - completed_at: datetime (nullable)
    - sort_order: number (for rotation)
    - created: datetime (auto)
    - updated: datetime (auto)

  zones:
    - id (auto)
    - name: string (required)
    - emoji: string (optional)
    - weekday: number (0=So, 1=Mo, ..., 6=Sa)
    - color: string (hex color)

  settings:
    - id (auto)
    - key: string (unique)
    - value: json
  ```

  **Acceptance Criteria**:
  ```bash
  # PocketBase läuft
  curl -s http://localhost:8080/api/health
  # Expected: {"code":200,"message":"API is healthy."}

  # Collections existieren
  curl -s http://localhost:8080/api/collections | jq '.items[].name'
  # Expected: "tasks", "zones", "settings"

  # Beispieldaten vorhanden
  curl -s http://localhost:8080/api/collections/tasks/records | jq '.totalItems'
  # Expected: > 0
  ```

  **Commit**: YES
  - Message: `feat(db): PocketBase schema with tasks, zones, settings`
  - Files: `pocketbase/pb_schema.json`, `pocketbase/pb_hooks/`

---

- [x] 4. Service Worker & PWA Manifest

  **What to do**:
  - `manifest.json` mit:
    - name: "Schweinehund"
    - display: "standalone"
    - theme_color: (passt zum Design)
    - icons (aus Task 12)
  - `sw.js` Service Worker:
    - Cache static assets (HTML, CSS, JS, Icons)
    - Network-first für API calls
    - SSE/Realtime Endpoints ignorieren
    - Offline fallback page

  **Must NOT do**:
  - Keine Offline-Edits/Sync-Queue
  - Nicht `/api/realtime` cachen

  **Recommended Agent Profile**:
  - **Category**: `unspecified-low`
    - Reason: Standard PWA patterns
  - **Skills**: [`playwright`]
    - `playwright`: For testing PWA install functionality

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 3, 13)
  - **Blocks**: Task 5
  - **Blocked By**: Task 1

  **References**:
  
  **External References**:
  - PWA Manifest: https://developer.mozilla.org/en-US/docs/Web/Manifest
  - Service Worker: https://developer.mozilla.org/en-US/docs/Web/API/Service_Worker_API

  **Acceptance Criteria**:
  ```bash
  # Manifest ist valide JSON
  cat frontend/manifest.json | jq '.name'
  # Expected: "Schweinehund"

  # Service Worker ist registrierbar (Syntax check)
  node -e "require('fs').readFileSync('frontend/sw.js', 'utf8')"
  # Expected: No syntax errors
  ```

  **Playwright Verification** (nach Task 5):
  ```
  1. Navigate to: https://schweinehund.local:8080
  2. Open DevTools → Application → Manifest
  3. Assert: Manifest loaded, "Add to Home Screen" available
  4. Assert: Service Worker registered
  5. Screenshot: .sisyphus/evidence/task-4-pwa.png
  ```

  **Commit**: YES
  - Message: `feat(pwa): manifest.json and service worker`
  - Files: `frontend/manifest.json`, `frontend/sw.js`

---

- [x] 13. UI Theme & CSS

  **What to do**:
  - CSS Variables für Farbschema definieren:
    - Primary: Warm Orange (#FF7F50 oder ähnlich)
    - Secondary: Rosa/Pink Akzent
    - Background: Creme/Off-white
    - Success: Grün für erledigte Tasks
  - Base styles (Typography, Spacing, Buttons)
  - Mobile-first Responsive Design
  - Checkbox-Styling (verspielt, nicht Standard)
  - Card-Design für Tasks
  - Animations (subtle, nicht übertrieben)

  **Must NOT do**:
  - Keine CSS-Frameworks (Tailwind, Bootstrap)
  - Keine Dark Mode (nur Light)
  - Keine komplexen Animationen

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
    - Reason: UI/UX styling task
  - **Skills**: [`frontend-design`, `frontend-ui-ux`]
    - `frontend-design`: For polished visual design
    - `frontend-ui-ux`: For responsive, mobile-first patterns

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with Tasks 3, 4)
  - **Blocks**: Task 5
  - **Blocked By**: Task 1

  **References**:
  
  **External References**:
  - CSS Custom Properties: https://developer.mozilla.org/en-US/docs/Web/CSS/--*
  - Modern CSS Techniques: https://moderncss.dev/

  **Acceptance Criteria**:
  ```bash
  # CSS Datei existiert
  ls frontend/css/
  # Expected: style.css (or similar)

  # CSS Variables sind definiert
  grep -c "^--" frontend/css/style.css
  # Expected: >= 5 (mindestens 5 Custom Properties)
  ```

  **Commit**: YES
  - Message: `feat(ui): playful color theme and base styles`
  - Files: `frontend/css/*`

---

### Wave 3: Frontend Components

- [x] 5. Frontend Base Layout (HTMX + Alpine)

  **What to do**:
  - `index.html` mit:
    - HTMX + Alpine.js einbinden (CDN)
    - alpine-morph Extension
    - PWA Meta Tags
    - App Shell (Header, Main, Navigation)
  - Header: Schweinehund Logo + Titel
  - Navigation: Tabs für "Heute", "Alle Aufgaben", "Zonen"
  - Main Area mit hx-target für Content Swapping
  - PocketBase JS SDK einbinden für Realtime

  **Must NOT do**:
  - Keine SPA-Router
  - Keine Build-Tools

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
    - Reason: UI layout and structure
  - **Skills**: [`frontend-ui-ux`]
    - `frontend-ui-ux`: For mobile-first responsive layout

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Sequential (start of Wave 3)
  - **Blocks**: Tasks 6, 7, 8, 9
  - **Blocked By**: Tasks 3, 4, 12, 13

  **References**:
  
  **External References**:
  - HTMX: https://htmx.org/docs/
  - Alpine.js: https://alpinejs.dev/
  - PocketBase JS SDK: https://github.com/pocketbase/js-sdk

  **Code Pattern**:
  ```html
  <!DOCTYPE html>
  <html lang="de">
  <head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <link rel="manifest" href="/manifest.json">
    <title>Schweinehund</title>
    <!-- CSS -->
    <link rel="stylesheet" href="/css/style.css">
  </head>
  <body x-data="app()" hx-ext="alpine-morph">
    <header>
      <img src="/assets/logo.svg" alt="Schweinehund">
      <h1>Schweinehund</h1>
    </header>
    
    <nav>
      <button hx-get="/partials/today" hx-target="#main">Heute</button>
      <button hx-get="/partials/tasks" hx-target="#main">Aufgaben</button>
      <button hx-get="/partials/zones" hx-target="#main">Zonen</button>
    </nav>
    
    <main id="main" hx-get="/partials/today" hx-trigger="load">
      Loading...
    </main>
    
    <!-- Scripts -->
    <script src="https://unpkg.com/htmx.org@2.0.0"></script>
    <script src="https://unpkg.com/alpinejs@3.x.x/dist/cdn.min.js"></script>
    <script src="/js/app.js"></script>
  </body>
  </html>
  ```

  **Acceptance Criteria**:
  
  **Playwright Verification**:
  ```
  1. Navigate to: https://schweinehund.local:8080
  2. Wait for: header with logo visible
  3. Assert: Navigation buttons exist ("Heute", "Aufgaben", "Zonen")
  4. Click: "Aufgaben" button
  5. Wait for: #main content changes
  6. Screenshot: .sisyphus/evidence/task-5-layout.png
  ```

  **Commit**: YES
  - Message: `feat(frontend): base layout with HTMX and Alpine.js`
  - Files: `frontend/index.html`, `frontend/js/app.js`

---

- [x] 6. Task List Component

  **What to do**:
  - "Heute" View: Zeigt Aufgaben für aktuellen Wochentag
    - Tägliche Mini-Routine (aus settings oder hardcoded)
    - Zone des Tages (basierend auf Wochentag)
  - "Alle Aufgaben" View: Alle Tasks gruppiert nach Typ
  - Checkbox zum Abhaken (HTMX POST)
  - Visuelle Unterscheidung: erledigt/offen
  - Große Aufgaben: "Nächste dran" prominent anzeigen

  **Must NOT do**:
  - Keine Inline-Editing hier (separate Modal)
  - Keine Drag-and-Drop Sortierung

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
    - Reason: Interactive UI component
  - **Skills**: [`frontend-ui-ux`]
    - `frontend-ui-ux`: For task list interactions

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3b (with Tasks 7, 8)
  - **Blocks**: Task 11
  - **Blocked By**: Task 5

  **References**:
  
  **Pattern References**:
  - HTMX Checkbox Pattern: hx-post mit hx-swap="outerHTML"
  
  **External References**:
  - HTMX Examples: https://htmx.org/examples/

  **Acceptance Criteria**:
  
  **Playwright Verification**:
  ```
  1. Navigate to: https://schweinehund.local:8080
  2. Wait for: Task list loaded
  3. Assert: At least one task visible
  4. Find: First unchecked checkbox
  5. Click: The checkbox
  6. Wait for: Checkbox becomes checked (visual change)
  7. Assert: Task marked as completed (strikethrough or green)
  8. Refresh page
  9. Assert: Task still completed (persisted)
  10. Screenshot: .sisyphus/evidence/task-6-tasklist.png
  ```

  **Commit**: YES
  - Message: `feat(ui): task list component with checkboxes`
  - Files: `frontend/partials/today.html`, `frontend/partials/tasks.html`, `frontend/js/tasks.js`

---

- [x] 7. Task Edit Modal

  **What to do**:
  - Modal/Overlay für Task bearbeiten
  - Felder: Name, Emoji, Zone (Dropdown), is_daily
  - HTMX Form Submit
  - "Neue Aufgabe" Button
  - "Löschen" mit Bestätigung
  - Alpine.js für Modal state (open/close)

  **Must NOT do**:
  - Keine Inline-Validierung (Server macht das)
  - Keine komplexe Emoji-Picker (einfaches Input reicht)

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
    - Reason: Modal UI with form interactions
  - **Skills**: [`frontend-ui-ux`]
    - `frontend-ui-ux`: For form and modal patterns

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3b (with Tasks 6, 8)
  - **Blocks**: None
  - **Blocked By**: Task 5

  **References**:
  
  **External References**:
  - Alpine.js x-show: https://alpinejs.dev/directives/show
  - HTMX Forms: https://htmx.org/examples/modal-custom/

  **Acceptance Criteria**:
  
  **Playwright Verification**:
  ```
  1. Navigate to: https://schweinehund.local:8080/partials/tasks
  2. Click: "Neue Aufgabe" button
  3. Wait for: Modal visible
  4. Fill: input[name="name"] with "Test Aufgabe"
  5. Fill: input[name="emoji"] with "🧹"
  6. Click: Submit button
  7. Wait for: Modal closes
  8. Assert: "Test Aufgabe" appears in task list
  9. Screenshot: .sisyphus/evidence/task-7-modal.png
  ```

  **Commit**: YES (groups with 6)
  - Message: `feat(ui): task edit modal with CRUD operations`
  - Files: `frontend/partials/task-modal.html`, `frontend/js/modal.js`

---

- [x] 8. Zone Management

  **What to do**:
  - "Zonen" View: Liste aller Zonen mit Wochentag-Zuordnung
  - Zone bearbeiten (Name, Emoji, Wochentag, Farbe)
  - Neue Zone erstellen
  - Zone löschen (mit Warnung wenn Aufgaben zugeordnet)
  - Farbpicker für Zone-Farbe
  - Preview wie Zone im Kalender aussieht

  **Must NOT do**:
  - Keine Drag-and-Drop Wochentag-Zuordnung
  - Keine komplexe Farb-Logik

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
    - Reason: Zone configuration UI
  - **Skills**: [`frontend-ui-ux`]
    - `frontend-ui-ux`: For configuration interfaces

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3b (with Tasks 6, 7)
  - **Blocks**: None
  - **Blocked By**: Task 5

  **References**:
  
  **External References**:
  - HTML Color Input: https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/color

  **Acceptance Criteria**:
  
  **Playwright Verification**:
  ```
  1. Navigate to: https://schweinehund.local:8080
  2. Click: "Zonen" nav button
  3. Wait for: Zone list visible
  4. Assert: At least one zone visible (from seed data)
  5. Click: Edit button on first zone
  6. Wait for: Edit form visible
  7. Change: weekday dropdown to different day
  8. Click: Save
  9. Assert: Zone shows updated weekday
  10. Screenshot: .sisyphus/evidence/task-8-zones.png
  ```

  **Commit**: YES (groups with 6, 7)
  - Message: `feat(ui): zone management with weekday assignment`
  - Files: `frontend/partials/zones.html`, `frontend/js/zones.js`

---

### Wave 4: Logic & Notifications

- [x] 9. ntfy Notification Integration

  **What to do**:
  - PocketBase Hook: Bei bestimmten Events ntfy Nachricht senden
  - Events:
    - Task completed → optional (erstmal aus)
    - Daily reminder (09:00) → "Zeit für deine Aufgaben!"
    - Zone reminder → "Heute ist [Zone] dran"
    - Weekend task → "Wochenende: [Nächste große Aufgabe] ist dran"
    - Reset complete → "Neue Woche! Aufgaben zurückgesetzt"
  - Frontend: Link zu ntfy Android App Setup
  - Test-Button um Notification zu triggern

  **Must NOT do**:
  - Keine Notification bei jedem Checkbox-Klick
  - Keine In-App Notifications (nur Push via ntfy)

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: Backend integration with external service
  - **Skills**: []
    - No special skills needed

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 4 (with Tasks 10, 11)
  - **Blocks**: None
  - **Blocked By**: Tasks 2, 5

  **References**:
  
  **External References**:
  - ntfy.sh Publishing: https://docs.ntfy.sh/publish/
  - PocketBase Hooks: https://pocketbase.io/docs/go-overview/

  **Acceptance Criteria**:
  ```bash
  # Test-Notification senden
  curl -d "Test von Schweinehund!" http://localhost:8090/schweinehund
  # Expected: Notification erscheint auf Android (wenn ntfy App installiert)

  # PocketBase kann ntfy erreichen
  docker compose exec pocketbase curl -s http://ntfy:8090/schweinehund -d "Hook test"
  # Expected: {"id":"...", ...}
  ```

  **Commit**: YES
  - Message: `feat(notify): ntfy.sh integration for push notifications`
  - Files: `pocketbase/pb_hooks/notifications.go`

---

- [x] 10. Weekly Reset Scheduler

  **What to do**:
  - PocketBase Cron/Scheduler für Montag 00:00
  - Reset-Logik:
    1. Alle Tasks mit `is_daily=true`: `completed=false`, `completed_at=null`
    2. Große Aufgaben (is_daily=false): Nicht zurücksetzen
  - ntfy Notification nach Reset
  - Log entry für Debug

  **Must NOT do**:
  - Keine manuellen Reset-Buttons
  - Keine komplexe Timezone-Logik (Server-Zeit = lokal)

  **Recommended Agent Profile**:
  - **Category**: `unspecified-low`
    - Reason: Scheduled task with simple logic
  - **Skills**: []
    - No special skills needed

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 4 (with Tasks 9, 11)
  - **Blocks**: None
  - **Blocked By**: Task 3

  **References**:
  
  **External References**:
  - PocketBase Cron: https://pocketbase.io/docs/go-jobs-scheduling/
  - Go Cron Library: github.com/robfig/cron

  **Acceptance Criteria**:
  ```bash
  # Manueller Test des Reset (Endpoint oder direkter Aufruf)
  # Nach Reset:
  curl -s http://localhost:8080/api/collections/tasks/records?filter="is_daily=true" | jq '.items[].completed' | sort | uniq
  # Expected: false (alle daily tasks sind uncompleted)

  # Große Aufgaben unberührt
  curl -s http://localhost:8080/api/collections/tasks/records?filter="is_daily=false%26%26completed=true" | jq '.totalItems'
  # Expected: >= 0 (completed große Aufgaben bleiben completed)
  ```

  **Commit**: YES
  - Message: `feat(scheduler): weekly reset for daily tasks on Monday 00:00`
  - Files: `pocketbase/pb_hooks/scheduler.go`

---

- [x] 11. Task Rotation Logic

  **What to do**:
  - Bei Completion einer großen Aufgabe (is_daily=false):
    1. `sort_order` auf MAX(sort_order) + 1 setzen
    2. Damit wandert sie ans "Ende" der Rotation
  - "Nächste dran" = große Aufgabe mit kleinstem sort_order die nicht completed ist
  - Wenn alle completed: älteste completed (nach completed_at) als nächste vorschlagen
  - Frontend: "Nächste große Aufgabe" prominent anzeigen

  **Must NOT do**:
  - Keine automatische Uncomplete von großen Aufgaben
  - Keine komplexe Priorisierung

  **Recommended Agent Profile**:
  - **Category**: `unspecified-low`
    - Reason: Business logic implementation
  - **Skills**: []
    - No special skills needed

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 4 (with Tasks 9, 10)
  - **Blocks**: None
  - **Blocked By**: Tasks 3, 6

  **References**:
  
  **Logic**:
  ```
  Bei Completion (is_daily=false):
    1. SET sort_order = (SELECT MAX(sort_order) FROM tasks WHERE is_daily=false) + 1
    2. SET completed = true, completed_at = NOW()

  "Nächste dran" Query:
    SELECT * FROM tasks 
    WHERE is_daily = false 
    ORDER BY 
      CASE WHEN completed THEN 1 ELSE 0 END,  -- Uncompleted first
      sort_order ASC
    LIMIT 1
  ```

  **Acceptance Criteria**:
  
  **Playwright Verification**:
  ```
  1. Navigate to: https://schweinehund.local:8080
  2. Assert: "Nächste große Aufgabe" section visible
  3. Note: Current "next up" task name
  4. Click: Checkbox for that task
  5. Wait for: Task marked complete
  6. Assert: Different task now shows as "Nächste große Aufgabe"
  7. Assert: Completed task moved to bottom of list
  8. Screenshot: .sisyphus/evidence/task-11-rotation.png
  ```

  **Commit**: YES (groups with 10)
  - Message: `feat(logic): task rotation for large recurring tasks`
  - Files: `pocketbase/pb_hooks/rotation.go`, `frontend/js/rotation.js`

---

### Wave 5: Deployment

- [ ] 14. Docker Compose & Final Deployment

  **What to do**:
  - Finales docker-compose.yml:
    - PocketBase Container (Port 8080)
    - ntfy Container (Port 8090)
    - Caddy/nginx für HTTPS Termination
    - Volume mounts für Persistence
  - mkcert Zertifikate einbinden
  - Health checks
  - Restart policies
  - `.env.example` mit Konfiguration
  - README.md mit Setup-Anleitung

  **Must NOT do**:
  - Keine komplexe Orchestrierung
  - Kein Kubernetes

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: Final integration and deployment
  - **Skills**: []
    - No special skills needed

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Final (after all others)
  - **Blocks**: None
  - **Blocked By**: ALL previous tasks

  **References**:
  
  **External References**:
  - Caddy Docker: https://hub.docker.com/_/caddy
  - Docker Compose Best Practices: https://docs.docker.com/compose/production/

  **Acceptance Criteria**:
  ```bash
  # Full stack startet
  docker compose up -d
  docker compose ps
  # Expected: All services "running" or "healthy"

  # App erreichbar
  curl -k https://schweinehund.local:8080
  # Expected: HTML response with "Schweinehund"

  # ntfy erreichbar
  curl -s http://schweinehund.local:8090/schweinehund/json
  # Expected: Connection established

  # Persistence test
  docker compose down
  docker compose up -d
  curl -s https://schweinehund.local:8080/api/collections/tasks/records | jq '.totalItems'
  # Expected: Same number as before (data persisted)
  ```

  **Commit**: YES
  - Message: `feat(deploy): production-ready Docker Compose setup`
  - Files: `docker-compose.yml`, `Caddyfile`, `.env.example`, `README.md`

---

## Commit Strategy

| After Task | Message | Files | Verification |
|------------|---------|-------|--------------|
| 1, 2 | `feat(setup): initial project structure with Docker and ntfy` | docker-compose.yml, dirs | docker compose config |
| 12 | `feat(design): Schweinehund logo and PWA icons` | frontend/assets/* | file check |
| 3 | `feat(db): PocketBase schema` | pocketbase/* | curl health |
| 4 | `feat(pwa): manifest and service worker` | frontend/manifest.json, sw.js | JSON parse |
| 13 | `feat(ui): playful theme and base styles` | frontend/css/* | CSS check |
| 5 | `feat(frontend): base layout with HTMX/Alpine` | frontend/index.html, js/* | Playwright |
| 6, 7, 8 | `feat(ui): task and zone management components` | frontend/partials/*, js/* | Playwright |
| 9 | `feat(notify): ntfy integration` | pocketbase/pb_hooks/* | curl ntfy |
| 10, 11 | `feat(logic): weekly reset and task rotation` | pocketbase/pb_hooks/* | API test |
| 14 | `feat(deploy): production Docker setup` | docker-compose.yml, README | full stack |

---

## Success Criteria

### Verification Commands
```bash
# Full stack health
docker compose ps --format "table {{.Name}}\t{{.Status}}"
# Expected: All services Up/Healthy

# API erreichbar
curl -s https://schweinehund.local:8080/api/health | jq '.code'
# Expected: 200

# PWA installierbar (Lighthouse)
# Run Lighthouse PWA audit in Chrome DevTools
# Expected: PWA badge, installable

# Push funktioniert
curl -d "Finale Test-Notification" http://schweinehund.local:8090/schweinehund
# Expected: Notification auf Android
```

### Final Checklist
- [ ] App unter https://schweinehund.local:8080 erreichbar
- [ ] Aufgaben abhakbar und persistiert
- [ ] Zonen editierbar
- [ ] Push-Notifications auf Android
- [ ] PWA installierbar (Add to Home Screen)
- [ ] Weekly Reset funktioniert
- [ ] Große Aufgaben rotieren
- [ ] Verspieltes, farbenfrohe Design
- [ ] Docker Compose startet alle Services
- [ ] Daten überleben Container-Restart
