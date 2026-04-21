# StoryVote Implementation Plan

## Purpose
Build a minimal agile story point estimation application.

## Core Product Decisions
- Single Rust executable hosted on a teammate laptop.
- Browser clients only (no client install).
- One shared estimation session only (no rooms).
- No persistence: all state is in memory and is lost on process exit.
- Participants enter their display name on join.
- Host controls reveal and reset for each round.

## Networking and Hosting
- Default bind mode: LAN-accessible.
- Default bind address: 0.0.0.0.
- Optional restricted mode: localhost only.
- Default port: fixed port for reliability (example: 8787).
- Optional mode: random port.
- Mandatory behavior: always display a shareable URL at startup.

## Mandatory UX Requirements
- Join screen must require a display name.
- Voting deck includes Fibonacci values.
- Vote reveal and round reset are clear and fast.
- Mobile-friendly browser layout.
- Shareable URL is visible and easy to copy.

## Security and Safety Constraints
- No authentication required for MVP.
- Do not store historical voting data.
- Keep control actions (reveal/reset) host-only when possible.
- If local IP detection fails, show localhost URL and a clear warning.

## Dependencies and Licensing
- Runtime dependencies on host: only the executable.
- Build-time dependencies: Rust toolchain on build machine.
- Prefer minimal crates for networking, websocket, JSON, CLI, and logging.
- Keep all project code and dependencies compatible with permissive licenses (MIT, BSD, Apache).

## Suggested Rust Modules
- src/main.rs: flags, bind config, startup, URL display.
- src/http.rs: HTTP routes and static UI.
- src/ws.rs: real-time vote events.
- src/state.rs: in-memory participant and vote state.
- src/ui/: embedded static assets (HTML/CSS/JS) if used.

## Architecture and Code Quality Principles
- Keep modules small and focused with clear ownership of behavior.
- Keep state transitions deterministic and straightforward to reason about.
- Maintain strict separation between HTTP handling, websocket events, and domain state.
- Prefer simple, readable code paths over complex abstractions.

## Verification Checklist
1. Start host on Windows and confirm shareable URL is printed.
2. Join from multiple devices on same LAN/VPN.
3. Run repeated vote, reveal, and reset cycles.
4. Restart app and confirm session data is gone.
5. Validate localhost-only mode and LAN mode.

## Testing Strategy
- Automated unit tests for state transitions and validation:
	- Participant join and duplicate-name handling.
	- Vote submission and overwrite behavior.
	- Reveal and reset transitions.
	- Disconnect/reconnect presence handling.
- Automated integration tests for critical flows:
	- Join, vote, reveal, reset, reconnect over websocket event paths.
	- Host-only reveal/reset authorization behavior.
	- Startup URL generation for LAN and localhost-only modes.
- Manual smoke tests before release:
	- Multi-device LAN session run with at least three participants.
	- Browser refresh/reconnect checks during active session.
- Minimum release gate:
	- All unit and integration tests pass.
	- No regressions in join, vote, reveal, reset, reconnect flows.

## Definition of Done
- Scope complete: single shared session for point estimation with self-entered participant names.
- Networking complete: LAN-accessible default mode works, localhost-only mode works.
- UX complete: users can join, vote, reveal, and reset without page errors on desktop and mobile browsers.
- URL requirement complete: app always shows a shareable join URL at startup.
- Data handling complete: no persistence is implemented; restarting the app clears all session data.
- Security baseline complete: input validation is in place and reveal/reset is host-controlled.
- Quality complete: required unit and integration tests pass, and smoke checks succeed.
- Documentation complete: plan and instructions remain aligned with delivered behavior.

## Future Enhancements (Out of MVP Scope)
- Enterprise identity integration through reverse proxy or OIDC.
- Roster loading from file or startup flag.
- Optional QR code display for join URL.
- Optional tunnel mode for cross-network access.
