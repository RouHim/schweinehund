# Draft: Simplify Local Network Infrastructure

## Requirements (confirmed)
- Remove Caddy reverse proxy entirely (not needed for local network)
- Remove HTTPS/TLS infrastructure (certificates, mkcert references)
- Configure PocketBase to serve frontend directly via `pb_public` directory
- Keep ntfy service unchanged (working as-is on port 8091)
- Rename git branch: `master` → `main` (local + remote)
- Update all documentation to reflect HTTP-only, direct access

## Technical Decisions

### PocketBase Static File Serving
- PocketBase automatically serves static files from `pb_public` directory
- Frontend files need to be mounted to `/pb/pb_public` in container
- PocketBase already exposed on port 8090 (currently also used directly)
- No Dockerfile changes needed - just volume mount adjustment

### Port Changes
| Service | Current | After |
|---------|---------|-------|
| PocketBase | 8090 (internal) + via Caddy 8080/8443 | 8090 (direct access) |
| Caddy | 8080, 8443, 8081 | REMOVED |
| ntfy | 8091 | 8091 (unchanged) |

### Files Requiring Modification
1. **docker-compose.yml**
   - Remove entire `caddy` service block
   - Remove `caddy_data` and `caddy_config` volumes
   - Add frontend volume mount to pocketbase: `./frontend:/pb/pb_public:ro`
   - Keep pocketbase port 8090 exposed
   
2. **README.md** (extensive changes)
   - Remove mkcert prerequisites
   - Remove certificate generation steps
   - Remove /etc/hosts setup for schweinehund.local
   - Update all URLs from https://schweinehund.local:8080 to http://localhost:8090
   - Remove Caddy from tech stack
   - Remove Caddy service from architecture section
   - Remove caddy_data/caddy_config from volumes
   - Remove certs/ from project structure
   - Remove Caddyfile reference
   - Remove Caddy troubleshooting sections
   - Remove HTTPS production certificates section
   
3. **.env.example**
   - Remove DOMAIN variable (not needed without Caddy)
   - Update NTFY_BASE_URL comment to use localhost
   
4. **Caddyfile**
   - DELETE entire file
   
5. **frontend/js/app.js**
   - Update PocketBase URL: `http://localhost:8080` → `http://localhost:8090`
   
6. **playwright.config.ts**
   - Update baseURL: `http://localhost:8080` → `http://localhost:8090`
   
7. **e2e/htmx-interactions.spec.ts**
   - Update hardcoded URL reference

8. **Git branch rename**
   - Local: `git branch -m master main`
   - Remote: `git push -u origin main && git push origin --delete master`
   - Update remote HEAD: `git remote set-head origin main`

## Research Findings
- PocketBase serves static files from `pb_public` directory automatically
- No special configuration needed - just place files in the directory
- Single-page app routing works with PocketBase's default fallback behavior
- Port 8090 is the default PocketBase HTTP port

## Open Questions
- [ ] None - all requirements are clear

## Scope Boundaries
- **INCLUDE**: 
  - Remove Caddy service from docker-compose.yml
  - Add pb_public volume mount for frontend
  - Update all URLs in code and docs
  - Delete Caddyfile
  - Remove DOMAIN env variable
  - Update README comprehensively
  - Git branch rename master → main

- **EXCLUDE**: 
  - DO NOT delete certs/ directory (user will do manually)
  - DO NOT modify ntfy service
  - DO NOT modify PocketBase Dockerfile
  - DO NOT modify pb_hooks
  - DO NOT modify frontend content (only the PocketBase URL reference)
