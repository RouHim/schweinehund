# Simplify Local Network Infrastructure

## TL;DR

> **Quick Summary**: Remove Caddy reverse proxy and HTTPS infrastructure from the local development setup, configure PocketBase to serve frontend files directly via `pb_public`, and rename git branch from `master` to `main`.
> 
> **Deliverables**:
> - Simplified docker-compose.yml with only PocketBase + ntfy services
> - PocketBase serving frontend directly on port 8090
> - Updated README without HTTPS/Caddy instructions
> - Deleted Caddyfile
> - Git branch renamed to `main`
> 
> **Estimated Effort**: Medium (2-3 hours)
> **Parallel Execution**: YES - 3 waves
> **Critical Path**: Task 1 → Task 4 → Task 6 → Task 7

---

## Context

### Original Request
User wants to simplify their local network setup by removing unnecessary HTTPS/reverse proxy infrastructure and standardizing on the `main` git branch. The current setup uses Caddy for TLS termination and reverse proxying, which is overkill for a local network environment.

### Interview Summary
**Key Discussions**:
- Current architecture: 3 services (PocketBase:8090, Caddy:8080/8443/8081, ntfy:8091)
- PocketBase can serve static files directly from `pb_public` directory
- All URLs need updating from 8080 → 8090
- Git branch rename: master → main (local + remote)

**Research Findings**:
- PocketBase automatically serves static files from `pb_public` with SPA fallback (indexFallback)
- No configuration needed - just mount the directory
- Port 8090 is already exposed in docker-compose
- Frontend hardcodes PocketBase URL at `http://localhost:8080` in app.js

### Self-Review Gaps Identified
**Addressed**:
- manifest.json uses relative paths (`/`, `/assets/`) - no changes needed
- ntfy/ANDROID_SETUP.md has existing bug (references 8090 instead of 8091 for ntfy) - will fix as part of documentation cleanup
- Internal Docker networking (pocketbase:8090, ntfy internal) remains unchanged

---

## Work Objectives

### Core Objective
Remove all Caddy/HTTPS infrastructure and configure PocketBase to serve the frontend directly, simplifying the architecture for local network use.

### Concrete Deliverables
- Modified `docker-compose.yml` without Caddy service
- PocketBase container with frontend mounted to `/pb/pb_public`
- Updated `frontend/js/app.js` pointing to port 8090
- Updated `playwright.config.ts` with new baseURL
- Updated `e2e/htmx-interactions.spec.ts` URL reference
- Simplified `.env.example` without DOMAIN variable
- Deleted `Caddyfile`
- Comprehensively rewritten `README.md`
- Updated `ntfy/ANDROID_SETUP.md` (fix existing port bug + remove Caddy reference)
- Git branch renamed: `master` → `main`

### Definition of Done
- [ ] `docker compose up -d` starts only pocketbase and ntfy services
- [ ] `curl http://localhost:8090/` returns index.html content
- [ ] `curl http://localhost:8090/api/health` returns healthy status
- [ ] `curl http://localhost:8091/v1/health` returns ntfy healthy status
- [ ] PocketBase admin accessible at `http://localhost:8090/_/`
- [ ] `git branch` shows `main` as current branch
- [ ] `git remote show origin` shows HEAD points to `main`

### Must Have
- PocketBase serves frontend via pb_public mount
- All URLs updated to port 8090
- README reflects new simplified architecture
- Git branch is `main`

### Must NOT Have (Guardrails)
- DO NOT delete `certs/` directory (user will do manually)
- DO NOT modify ntfy service configuration (ports, volumes, environment)
- DO NOT modify PocketBase Dockerfile
- DO NOT modify `pocketbase/pb_hooks/*` files
- DO NOT add new features or services
- DO NOT change internal Docker network addressing
- DO NOT modify `pocketbase/pb_data` or `pocketbase/pb_migrations`

---

## Verification Strategy (MANDATORY)

### Test Decision
- **Infrastructure exists**: YES (Playwright E2E tests)
- **User wants tests**: Manual verification (infrastructure changes, not code logic)
- **Framework**: Playwright for E2E, but primarily manual verification for this infrastructure task

### Automated Verification (Manual Commands)

Each TODO includes EXECUTABLE verification procedures:

**For Infrastructure Changes** (using Bash):
```bash
# Verify services start correctly
docker compose up -d
docker compose ps
# Assert: Only pocketbase and ntfy services running

# Verify PocketBase serves frontend
curl -s http://localhost:8090/ | grep -q "Schweinehund"
# Assert: Returns 0 (found)

# Verify API health
curl -s http://localhost:8090/api/health
# Assert: Returns JSON with healthy status

# Verify ntfy health
curl -s http://localhost:8091/v1/health
# Assert: Returns healthy status
```

**Evidence Requirements:**
- Command output captured and compared against expected patterns
- Exit codes checked (0 = success)

---

## Execution Strategy

### Parallel Execution Waves

```
Wave 1 (Start Immediately - Independent Changes):
├── Task 1: Modify docker-compose.yml [no dependencies]
├── Task 2: Update frontend/js/app.js [no dependencies]
├── Task 3: Update test configurations [no dependencies]
└── Task 5: Update .env.example [no dependencies]

Wave 2 (After Wave 1):
├── Task 4: Rewrite README.md [depends: 1, so URLs are correct]
└── Task 6: Delete Caddyfile [depends: 1, compose no longer references it]

Wave 3 (After All Code Changes):
├── Task 7: Update ntfy/ANDROID_SETUP.md [depends: 4, for consistency]
└── Task 8: Git branch rename [depends: all, final step]

Critical Path: Task 1 → Task 4 → Task 7 → Task 8
Parallel Speedup: ~50% faster than sequential
```

### Dependency Matrix

| Task | Depends On | Blocks | Can Parallelize With |
|------|------------|--------|---------------------|
| 1 | None | 4, 6 | 2, 3, 5 |
| 2 | None | 8 | 1, 3, 5 |
| 3 | None | 8 | 1, 2, 5 |
| 4 | 1 | 7, 8 | 6 |
| 5 | None | 8 | 1, 2, 3 |
| 6 | 1 | 8 | 4 |
| 7 | 4 | 8 | None |
| 8 | All | None | None (final) |

### Agent Dispatch Summary

| Wave | Tasks | Recommended Approach |
|------|-------|---------------------|
| 1 | 1, 2, 3, 5 | Run in parallel - independent file edits |
| 2 | 4, 6 | Run in parallel after Wave 1 completes |
| 3 | 7, 8 | Sequential - 7 then 8 as final step |

---

## TODOs

- [ ] 1. Modify docker-compose.yml - Remove Caddy Service

  **What to do**:
  - Remove entire `caddy` service block (lines 33-62)
  - Remove `caddy_data` and `caddy_config` from volumes section (lines 92-95)
  - Add frontend volume mount to pocketbase service: `- ./frontend:/pb/pb_public:ro`
  - Keep pocketbase port mapping `"8090:8090"` (already correct)
  - Keep ntfy service unchanged
  - Keep network `schweinehund-net` unchanged

  **Must NOT do**:
  - Do NOT modify ntfy service
  - Do NOT change pocketbase ports or other volumes
  - Do NOT remove the network definition

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Single file modification with clear deletions and additions
  - **Skills**: None required
    - Docker compose is standard YAML, no specialized skills needed

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 2, 3, 5)
  - **Blocks**: Tasks 4, 6
  - **Blocked By**: None (can start immediately)

  **References**:
  - `docker-compose.yml:33-62` - Caddy service block to REMOVE
  - `docker-compose.yml:17` - Existing pocketbase volumes section to ADD frontend mount
  - `docker-compose.yml:92-95` - Volume definitions to REMOVE (caddy_data, caddy_config)
  - PocketBase docs: pb_public directory is auto-served at root

  **Acceptance Criteria**:
  ```bash
  # Agent runs:
  docker compose config --services
  # Assert: Output is exactly "pocketbase\nntfy" (no caddy)
  
  docker compose config | grep -c "caddy"
  # Assert: Output is "0"
  
  docker compose config | grep "pb_public"
  # Assert: Output contains "./frontend:/pb/pb_public:ro"
  ```

  **Commit**: YES
  - Message: `chore(infra): remove Caddy reverse proxy from docker-compose`
  - Files: `docker-compose.yml`
  - Pre-commit: `docker compose config` (validates YAML)

---

- [ ] 2. Update frontend/js/app.js - Change PocketBase URL

  **What to do**:
  - Change line 47: `const pb = new PocketBase('http://localhost:8080');`
  - To: `const pb = new PocketBase('http://localhost:8090');`

  **Must NOT do**:
  - Do NOT modify any other code in this file
  - Do NOT change the PocketBase client initialization logic

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Single line change, trivial edit
  - **Skills**: None required

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 3, 5)
  - **Blocks**: Task 8
  - **Blocked By**: None (can start immediately)

  **References**:
  - `frontend/js/app.js:47` - Line to modify: `const pb = new PocketBase('http://localhost:8080');`

  **Acceptance Criteria**:
  ```bash
  # Agent runs:
  grep -o "localhost:[0-9]*" frontend/js/app.js
  # Assert: Output is "localhost:8090"
  ```

  **Commit**: NO (groups with Task 3)

---

- [ ] 3. Update Test Configurations - Fix URLs

  **What to do**:
  - `playwright.config.ts` line 15: Change `baseURL: 'http://localhost:8080'` to `'http://localhost:8090'`
  - `e2e/htmx-interactions.spec.ts` line 7: Change `'http://localhost:8080/'` to `'http://localhost:8090/'`

  **Must NOT do**:
  - Do NOT modify test logic
  - Do NOT change other test configuration options

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Two simple line changes in test files
  - **Skills**: None required

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 2, 5)
  - **Blocks**: Task 8
  - **Blocked By**: None (can start immediately)

  **References**:
  - `playwright.config.ts:15` - baseURL configuration
  - `e2e/htmx-interactions.spec.ts:7` - Hardcoded URL in test assertion

  **Acceptance Criteria**:
  ```bash
  # Agent runs:
  grep "baseURL" playwright.config.ts
  # Assert: Contains "localhost:8090"
  
  grep "8080" e2e/htmx-interactions.spec.ts
  # Assert: Returns no matches (exit code 1)
  
  grep "8090" e2e/htmx-interactions.spec.ts
  # Assert: Returns match (exit code 0)
  ```

  **Commit**: YES (combined with Task 2)
  - Message: `chore: update URLs from port 8080 to 8090`
  - Files: `frontend/js/app.js`, `playwright.config.ts`, `e2e/htmx-interactions.spec.ts`
  - Pre-commit: None

---

- [ ] 4. Rewrite README.md - Remove HTTPS/Caddy Documentation

  **What to do**:
  - Remove mkcert from Prerequisites section (lines 17-28)
  - Remove "Generate local HTTPS certificates" step (lines 38-47)
  - Remove /etc/hosts setup step (lines 60-63)
  - Update "Accessing the Application" URLs (lines 76-82):
    - `https://schweinehund.local:8080` → `http://localhost:8090`
    - `https://schweinehund.local:8080/_/` → `http://localhost:8090/_/`
    - `https://schweinehund.local:8080/api/` → `http://localhost:8090/api/`
  - Update First-Time PocketBase Admin Setup URL (line 85)
  - Update Project Structure - remove Caddyfile and certs/ references (lines 92-110)
  - Update Service Architecture section (lines 172-183):
    - Remove "PocketBase: Port 8090 (internal only, accessed via Caddy reverse proxy)"
    - Change to: "PocketBase: Port 8090 (serves API and frontend directly)"
    - Remove Caddy line entirely
    - Remove caddy_data, caddy_config from Volumes list
  - Update Logs section - remove `docker compose logs -f caddy` (line 193)
  - Remove "Can't access https://schweinehund.local:8080" troubleshooting section (lines 235-252)
  - Update Tech Stack section - remove Caddy (line 9)
  - Remove Production HTTPS Certificates section (lines 300-312)
  - Remove duplicate Project Structure section at end (lines 314-337)

  **Must NOT do**:
  - Do NOT remove ntfy documentation
  - Do NOT modify Data Management section
  - Do NOT change Contributing guidelines

  **Recommended Agent Profile**:
  - **Category**: `writing`
    - Reason: Extensive documentation rewrite requiring careful editing
  - **Skills**: None required
    - Standard markdown editing

  **Parallelization**:
  - **Can Run In Parallel**: YES (with Task 6)
  - **Parallel Group**: Wave 2
  - **Blocks**: Tasks 7, 8
  - **Blocked By**: Task 1 (to ensure URLs are correct)

  **References**:
  - `README.md:9` - Tech Stack section mentioning Caddy
  - `README.md:17-28` - mkcert Prerequisites to REMOVE
  - `README.md:38-47` - Certificate generation step to REMOVE
  - `README.md:60-63` - /etc/hosts step to REMOVE
  - `README.md:76-82` - Application URLs to UPDATE
  - `README.md:85` - Admin URL to UPDATE
  - `README.md:92-110` - Project Structure to UPDATE
  - `README.md:172-183` - Service Architecture to UPDATE
  - `README.md:193` - Logs section to UPDATE
  - `README.md:235-252` - Caddy troubleshooting to REMOVE
  - `README.md:300-312` - Production HTTPS section to REMOVE
  - `README.md:314-337` - Duplicate Project Structure to REMOVE

  **Acceptance Criteria**:
  ```bash
  # Agent runs:
  grep -c "mkcert" README.md
  # Assert: Output is "0"
  
  grep -c "Caddy" README.md
  # Assert: Output is "0"
  
  grep -c "https://" README.md
  # Assert: Output is "0" (no HTTPS URLs)
  
  grep -c "8080" README.md
  # Assert: Output is "0" (no old port references)
  
  grep "localhost:8090" README.md | head -1
  # Assert: Contains frontend URL
  
  grep -c "certs/" README.md
  # Assert: Output is "0"
  ```

  **Commit**: YES
  - Message: `docs: simplify README for direct PocketBase access without reverse proxy`
  - Files: `README.md`
  - Pre-commit: None

---

- [ ] 5. Update .env.example - Remove DOMAIN Variable

  **What to do**:
  - Remove DOMAIN variable and its comments (lines 14-18)
  - Update NTFY_BASE_URL comment to reference localhost (line 10-11)

  **Must NOT do**:
  - Do NOT remove PB_ENCRYPTION_KEY
  - Do NOT remove NTFY_BASE_URL

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Small file with simple deletions
  - **Skills**: None required

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with Tasks 1, 2, 3)
  - **Blocks**: Task 8
  - **Blocked By**: None (can start immediately)

  **References**:
  - `.env.example:14-18` - DOMAIN variable section to REMOVE
  - `.env.example:10-11` - NTFY_BASE_URL comments to UPDATE

  **Acceptance Criteria**:
  ```bash
  # Agent runs:
  grep -c "DOMAIN" .env.example
  # Assert: Output is "0"
  
  grep "NTFY_BASE_URL" .env.example
  # Assert: Contains the variable (still present)
  
  grep "PB_ENCRYPTION_KEY" .env.example
  # Assert: Contains the variable (still present)
  ```

  **Commit**: YES
  - Message: `chore: remove DOMAIN env variable, not needed without reverse proxy`
  - Files: `.env.example`
  - Pre-commit: None

---

- [ ] 6. Delete Caddyfile

  **What to do**:
  - Delete the file `Caddyfile`

  **Must NOT do**:
  - Do NOT delete any other files
  - Do NOT delete certs/ directory (user will do manually)

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Single file deletion
  - **Skills**: None required

  **Parallelization**:
  - **Can Run In Parallel**: YES (with Task 4)
  - **Parallel Group**: Wave 2
  - **Blocks**: Task 8
  - **Blocked By**: Task 1 (docker-compose no longer references Caddyfile)

  **References**:
  - `Caddyfile` - File to DELETE (36 lines, reverse proxy config)

  **Acceptance Criteria**:
  ```bash
  # Agent runs:
  test -f Caddyfile && echo "EXISTS" || echo "DELETED"
  # Assert: Output is "DELETED"
  ```

  **Commit**: YES
  - Message: `chore: delete Caddyfile, no longer needed`
  - Files: `Caddyfile`
  - Pre-commit: None

---

- [ ] 7. Update ntfy/ANDROID_SETUP.md - Fix Port Bug and Remove Caddy Reference

  **What to do**:
  - Fix existing bug: All references to port 8090 for ntfy should be 8091
    - Line 13: `http://schweinehund.local:8090` → `http://localhost:8091`
    - Line 31: `http://schweinehund.local:8090` → `http://localhost:8091`
    - Line 55: `http://localhost:8090/schweinehund` → `http://localhost:8091/schweinehund`
    - Line 61: `http://192.168.1.100:8090/schweinehund` → `http://192.168.1.100:8091/schweinehund`
    - Line 76: `http://schweinehund.local:8090` → `http://localhost:8091`
    - Line 99: Port 8090 → Port 8091
    - Line 139: `http://192.168.X.X:8090` → `http://192.168.X.X:8091`
  - Remove Caddy reference at line 149: Change "set up HTTPS with self-signed certificates (Caddy/nginx reverse proxy)" to "HTTPS is not configured for this local network setup"

  **Must NOT do**:
  - Do NOT change the ntfy topic name
  - Do NOT modify notification schedule documentation

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Simple find-replace operations
  - **Skills**: None required

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 3 (sequential)
  - **Blocks**: Task 8
  - **Blocked By**: Task 4 (for documentation consistency)

  **References**:
  - `ntfy/ANDROID_SETUP.md:13,31,55,61,76,99,139` - Port 8090 references to fix (should be 8091)
  - `ntfy/ANDROID_SETUP.md:149` - Caddy/nginx reference to REMOVE

  **Acceptance Criteria**:
  ```bash
  # Agent runs:
  grep -c ":8090" ntfy/ANDROID_SETUP.md
  # Assert: Output is "0"
  
  grep -c ":8091" ntfy/ANDROID_SETUP.md
  # Assert: Output is greater than 0
  
  grep -c "Caddy" ntfy/ANDROID_SETUP.md
  # Assert: Output is "0"
  ```

  **Commit**: YES
  - Message: `docs(ntfy): fix port references (8090→8091) and remove Caddy mention`
  - Files: `ntfy/ANDROID_SETUP.md`
  - Pre-commit: None

---

- [ ] 8. Git Branch Rename - master to main

  **What to do**:
  - Rename local branch: `git branch -m master main`
  - Push new branch to remote: `git push -u origin main`
  - Delete old remote branch: `git push origin --delete master`
  - Update remote HEAD: Done automatically by GitHub when default branch changes
  
  **Important**: User may need to update GitHub default branch settings manually in repository settings if the remote doesn't auto-update.

  **Must NOT do**:
  - Do NOT force push
  - Do NOT modify git history
  - Do NOT change any other branches

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Standard git commands
  - **Skills**: `['git-master']`
    - git-master: Branch rename operations require careful execution

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 3 (final step)
  - **Blocks**: None (final task)
  - **Blocked By**: All previous tasks (must be the last operation)

  **References**:
  - Current branch: `master` (confirmed via `git branch -a`)
  - Remote: `origin` pointing to `git@github.com:RouHim/schweinehund.git`
  - Remote tracking: `remotes/origin/master`

  **Acceptance Criteria**:
  ```bash
  # Agent runs:
  git branch --show-current
  # Assert: Output is "main"
  
  git remote show origin | grep "HEAD branch"
  # Assert: Contains "main"
  
  git branch -a | grep -c "master"
  # Assert: Output is "0" (no master branches remain)
  ```

  **Commit**: NO (this IS a git operation, commits should be done before this step)
  - Note: All previous tasks should be committed before running this task

---

## Commit Strategy

| After Task | Message | Files | Verification |
|------------|---------|-------|--------------|
| 1 | `chore(infra): remove Caddy reverse proxy from docker-compose` | docker-compose.yml | `docker compose config` |
| 2+3 | `chore: update URLs from port 8080 to 8090` | frontend/js/app.js, playwright.config.ts, e2e/htmx-interactions.spec.ts | grep checks |
| 4 | `docs: simplify README for direct PocketBase access without reverse proxy` | README.md | grep checks |
| 5 | `chore: remove DOMAIN env variable, not needed without reverse proxy` | .env.example | grep check |
| 6 | `chore: delete Caddyfile, no longer needed` | Caddyfile | file existence check |
| 7 | `docs(ntfy): fix port references (8090→8091) and remove Caddy mention` | ntfy/ANDROID_SETUP.md | grep checks |
| 8 | N/A (git operation) | N/A | git branch checks |

---

## Success Criteria

### Verification Commands
```bash
# 1. Services start correctly
docker compose up -d
docker compose ps
# Expected: pocketbase (healthy), ntfy (healthy) - NO caddy

# 2. Frontend served by PocketBase
curl -s http://localhost:8090/ | head -5
# Expected: HTML content with "Schweinehund"

# 3. API health check
curl -s http://localhost:8090/api/health
# Expected: {"code":200,"message":"API is healthy."}

# 4. PocketBase admin accessible
curl -s -o /dev/null -w "%{http_code}" http://localhost:8090/_/
# Expected: 200

# 5. ntfy health check
curl -s http://localhost:8091/v1/health
# Expected: healthy response

# 6. Git branch is main
git branch --show-current
# Expected: main

# 7. No Caddy references remain
grep -r "caddy" --include="*.yml" --include="*.md" --include="*.json" . 2>/dev/null | grep -v ".sisyphus" | wc -l
# Expected: 0

# 8. No port 8080 references remain (except this plan file)
grep -r "8080" --include="*.ts" --include="*.js" --include="*.md" . 2>/dev/null | grep -v ".sisyphus" | wc -l
# Expected: 0
```

### Final Checklist
- [ ] All "Must Have" present
  - [ ] PocketBase serves frontend via pb_public
  - [ ] All URLs point to port 8090
  - [ ] README is simplified
  - [ ] Git branch is `main`
- [ ] All "Must NOT Have" absent
  - [ ] certs/ directory NOT deleted
  - [ ] ntfy service unchanged
  - [ ] PocketBase Dockerfile unchanged
  - [ ] pb_hooks unchanged
- [ ] All services healthy
- [ ] E2E tests can run (after `docker compose up -d`)

---

## Post-Completion Notes for User

After running `/start-work` and completing all tasks:

1. **Manual cleanup** (optional):
   ```bash
   rm -rf certs/
   ```

2. **Update GitHub default branch** (if needed):
   - Go to repository Settings → Branches
   - Change default branch from `master` to `main`

3. **Test the setup**:
   ```bash
   docker compose down
   docker compose up -d
   # Open http://localhost:8090 in browser
   ```

4. **Update any CI/CD** that references the `master` branch

5. **Notify collaborators** about the branch rename
