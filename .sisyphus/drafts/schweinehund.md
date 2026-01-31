# Draft: Schweinehund Putzplan-App

## Requirements (confirmed)
- Aufgaben abhaken
- Tägliche Aufgaben: Sonntag abends/nachts automatisch zurücksetzen
- Größere Aufgaben: Kein Reset, stattdessen Rotation (erledigte wandern ans Ende)
- Mobile-first / PWA
- Push-Benachrichtigungen aufs Handy
- Lokal gehostet (Home-Server, kein Internet)
- Keine Authentifizierung
- App-Name: "Schweinehund"
- Fancy Icon/Logo/UI Theme gewünscht

## Aufgaben aus Beispiel-Putzplan
### Tägliche Mini-Routine (20-30 Min)
- Spülmaschine an/aus
- Küche grob aufräumen
- 1 Wäschegang oder Wäsche falten
- 5 Min gemeinsames Aufräumen mit Kind
- Oberflächen frei machen

### Wöchentliche Zonen
- **Montag (HO)**: EG - Küche, Esstisch, WC, Wischen
- **Dienstag (HO)**: KG - Waschmaschine, Müll, Ordnung
- **Mittwoch (HO)**: OG - Bad, Betten, Wäsche, Saugen
- **Donnerstag (Büro)**: Leicht - Staub, Papierkram, Ordnung
- **Freitag (Büro)**: Reset - Müll, Wäsche falten, Bad-Check

### Größere Aufgaben (rotierend, kein Reset)
- Bad gründlicher
- Kühlschrank ausputzen
- Fenster putzen
- Schrank/Spielzeug aussortieren
- Bad Grundreinigung

## Research Findings

### Push Notifications - KRITISCH!
- **Problem**: Standard Web Push API braucht Internet (Google/Apple Push-Server)
- **Lösung für lokal**: WebSocket-basierte Notifications statt echtem Push
- **iOS**: Mindestens iOS 16.4+, App MUSS auf Home Screen installiert sein
- **HTTPS**: Auch lokal erforderlich - mkcert für lokale Zertifikate

### Tech Stack Empfehlung
1. **PocketBase + HTMX/Alpine.js** (am einfachsten)
   - Single 12MB Binary
   - SQLite eingebaut
   - Realtime via WebSocket
   - Admin-UI inklusive
   
2. **Go stdlib + SvelteKit** (beste Performance)
   - True single binary
   - Mehr Kontrolle
   - Mehr Aufwand

3. **Bun + SQLite + vanilla JS** (JavaScript-only)
   - Wenn JavaScript bevorzugt

## Environment
- **Server OS**: TrueNAS Scale (Linux-based)
- **Docker**: Ja, verfügbar
- **Deployment**: Docker Container
- **Handy**: Android (volle PWA-Unterstützung)
- **Benachrichtigungs-Zeit**: 09:00 Uhr morgens
- **Mehrere Geräte**: Ja, unabhängig (shared state, keine Konflikt-Logik nötig)
- **Editierbarkeit**: Voll editierbar (Zonen + Wochentage + Aufgaben)

## Final Decisions
- **Farbschema**: Ich wähle - warme Töne, verspielt (Schweinehund-Theme)
- **Test-Strategie**: Minimal (grundlegende Funktionsprüfung)

## Metis Gap Analysis
**Kritische Erkenntnisse:**
1. **09:00 Benachrichtigung**: PWA kann keine präzisen geplanten Notifications senden wenn App geschlossen ist!
   - Option A: Notifications nur wenn App offen (PWA-Limitation)
   - Option B: ntfy.sh (self-hosted Push-Service) einbinden
   
2. **PocketBase nutzt SSE statt WebSocket** - Einfacher für Service Worker

3. **iOS Support**: Eingeschränkt (kein auto-install prompt, manuelles "Add to Home Screen" nötig)

4. **Offline-Verhalten**: Was passiert ohne Netz?

## Metis Gap Resolutions
- **09:00 Notification**: ntfy.sh (self-hosted Push-Service) einbinden für echte Push-Notifications
- **iOS Support**: Nur Android (optimiert), iOS best-effort ohne Extra-Aufwand
- **Offline-Verhalten**: Read-only cached (letzte Liste anzeigen, keine Edits offline)

## Technical Decisions
- **Tech Stack**: PocketBase + HTMX/Alpine.js
- **Aufgaben**: Editierbar (CRUD für Aufgaben)
- **Reset-Zeit**: Montag 00:00 Uhr
- **Benachrichtigungen**: Alle 4 Typen gewünscht
  - Tägliche Erinnerung
  - Zonen-Erinnerung (wochentags-basiert)
  - Wochenend-Aufgabe
  - Reset-Bestätigung
- **UI Design**: Verspielt & farbenfroh (Emojis, bunte Akzente, motivierend)

## Scope Boundaries
- INCLUDE: 
  - Aufgaben-CRUD (anlegen, bearbeiten, löschen)
  - Tägliche Aufgaben mit Reset
  - Rotierende größere Aufgaben
  - Push via WebSocket
  - PWA mit Offline-Support
  - Farbenfrohe, motivierende UI
- EXCLUDE: 
  - Authentifizierung
  - Multi-User
  - Cloud-Dienste
  - Standard Web Push (braucht Internet)
