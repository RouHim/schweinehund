# Schweinehund MVP - Learnings

## Task 9: Mobile-First PWA UI Implementation

### Key Implementation Details

**UI Architecture:**
- Semantic HTML5 structure with mobile-first approach
- Pico.css classless base for minimal styling overhead
- Custom CSS variables for Schweinehund theme (coral #FF7F66, warm neutrals)
- Vanilla JavaScript for zero-framework overhead
- PWA-ready with proper meta tags and viewport configuration

**Theme System:**
- Light/dark mode with CSS custom properties
- localStorage persistence for user preference
- Smooth transitions between themes
- Mobile-friendly 48px touch targets for all interactive elements

**API Integration Pattern:**
- Fetch API for all HTTP requests
- Optimistic UI updates for checkboxes (update UI immediately, rollback on error)
- Proper error handling with user feedback
- Loading states for all async operations

**Key API Endpoints:**
- `GET /api/tasks/today` - Returns today's tasks with completion status
- `POST /api/tasks/:id/toggle` - Toggles task completion
- `GET /api/deep-cleaning` - Returns deep cleaning queue
- `POST /api/deep-cleaning/:id/complete` - Marks deep cleaning task complete
- `GET /api/settings` - Returns app settings
- `POST /api/settings` - Updates app settings

### Critical Bug Fix: Route Ordering

**Problem:** Static files returned 404 ("Route not found") even though rust-embed was configured correctly.

**Root Cause:** The API routes had `.recover(handle_rejection)` which caught all rejections before warp could try the `.or(static_files)` alternative.

**Solution:**
1. Removed `.recover()` from API routes function
2. Changed route order to `static_files.or(api)` (static first)
3. Applied `.recover()` to the combined routes
4. Made `handle_rejection` public for use in main.rs

**Key Insight:** Warp's `.or()` combinator only tries the second filter if the first one rejects WITHOUT being recovered. Once a rejection is recovered, it becomes a successful response and the `.or()` chain stops.

### Testing Approach

**Playwright E2E Tests:**
- Test page loads with correct title and mascot
- Verify tasks render on mobile viewport (iPhone 13: 390x844)
- Test checkbox interaction and optimistic UI updates  
- Verify all sections visible (tasks, deep cleaning, settings)
- Test theme toggle functionality

**Browser Support:**
- Tests run on Chromium, Mobile Chrome, and Mobile Safari
- 6 core tests covering critical user flows
- Tests passed 100% on Chromium and Android

### Mobile-First Considerations

- Touch-friendly 48px minimum tap targets
- Responsive typography scaling from 375px+
- Sticky header for persistent navigation
- Optimized for portrait orientation
- Reduced motion support via prefers-reduced-motion

### Performance Optimizations

- Embedded static files in binary (no file system I/O)
- Optimistic UI updates (no waiting for server)
- Minimal CSS with CSS custom properties
- No heavy framework overhead
- Proper caching headers (public, max-age=3600)

