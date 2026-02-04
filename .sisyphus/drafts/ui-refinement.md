# Draft: Schweinehund UI Refinement

## Requirements (confirmed)
- Refine/improve UI across both desktop and mobile views
- Mobile should feel native/app-like (it's a PWA)
- Desktop should feel spacious and well-balanced
- Preserve warm coral Schweinehund identity (#FF7F66, mascot, minimalist)
- Keep vanilla JS/CSS (no frameworks beyond PicoCSS classless)
- Only modify: `static/style.css`, `static/index.html`, `static/app.js`

## Design Decisions (from user)
1. **Mobile navigation**: Bottom tab bar (3 tabs: Today, Deep Cleaning, Settings) - scroll-to-section
2. **Progress indicator**: Header progress bar (slim bar under section header + "3/7 done" text)
3. **Settings visibility**: Collapsible by default
4. **Empty states**: Mascot-based (reuse pig-dog SVG with contextual messages)

## E2E Test Constraint: Settings Collapse BLOCKED
- `ui-basic.spec.ts` explicitly waits for `#settings-form` to be visible
- `desktop-layout.spec.ts` checks settings section full-width with form visible
- Both tests run on Pixel 5 AND Desktop Chrome projects
- **CANNOT collapse settings by default** - tests would fail
- **Adjusted approach**: Visually de-emphasize settings with compact card styling instead

## E2E Test Constraint: Desktop Column Width
- `desktop-layout.spec.ts` line 48: `todayBox.width ≤ 700`
- Cannot make columns wider than 700px each
- Current max-width 1200px → each column ~568px after gap
- Safe to increase to ~1400px max-width (columns still ≤ 700 at 1920px)

## Technical Decisions
- Bottom tab bar: `<nav>` element added to index.html, CSS-only hide on desktop (≥1024px)
- Progress bar: JS-driven, updates after each toggle/fetch, uses CSS custom property for width
- Task cards: Change from absolute-positioned actions to flexbox inline layout
- Empty states: Inline mascot SVG (small) in JS render functions
- All new CSS uses existing variable system, expanded with tokens

## Scope Boundaries
- INCLUDE: All 3 static files (style.css, index.html, app.js)
- INCLUDE: Mobile bottom nav, progress bar, empty states, card redesign, modal polish, micro-interactions
- EXCLUDE: E2E tests, backend Rust, service worker, manifest
- EXCLUDE: New npm dependencies or JS libraries
- EXCLUDE: Settings collapse (E2E constraint - will do visual compact instead)

## Research Findings
- Mascot SVG is a simple pig-dog hybrid with .primary/.secondary/.accent/.dark classes
- PicoCSS classless is the CSS base layer (CDN loaded)
- E2E tests run on Pixel 5 (412x869) and Desktop Chrome (1280x720)
- Playwright config has `cargo run` as webServer

## Open Questions
- None remaining
