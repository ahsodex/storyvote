# StoryVote (Rust, Single-Session)

Minimal, self-hosted agile story point estimation tool.

**Supported Platforms:** Windows, Linux, macOS (any platform with Rust 1.56+)

## What This App Does
- Runs as a single Rust server process.
- Uses one shared session (no rooms).
- Stores all state in memory only.
- Clears all votes and participants when the process stops.

## Quick Start
1. Build and run:
   - `cargo run`
2. Share the printed `Share URL` with teammates.
3. Teammates open the URL in a browser and enter their name.

## Release Build (Recommended For Team Use)
1. Build optimized executable:
  - `cargo build --release`
2. Run optimized executable:
  - **Windows:** `target\release\storyvote.exe`
  - **Linux/macOS:** `target/release/storyvote`
3. Show CLI help for executable:
  - **Windows:** `target\release\storyvote.exe --help`
  - **Linux/macOS:** `target/release/storyvote --help`

## Runtime Modes
- Default (LAN-accessible):
  - `cargo run`
- Localhost-only (host machine only):
  - `cargo run -- --localhost-only`
- Random port:
  - `cargo run -- --random-port`
- Custom bind and port:
  - `cargo run -- --bind 0.0.0.0 --port 8787`
- Enable HTTP access logging (request/response logs):
  - `cargo run -- --http-access-log`

## Host Controls
- First participant to join becomes host.
- Host can reveal votes and reset rounds.
- Host can set the current estimation topic in the top text field.
- If host disconnects, host role moves to another connected participant.

## Voting Deck
- `0, 1, 2, 3, 5, 8, 13, 21, 34, 55, ?`

## Reveal Behavior
- Before reveal: participants show a `Voted` indicator only.
- After reveal: each participant row shows their revealed value inline.
- After reveal: summary shows average and median for numeric votes (`?` and other non-numeric values are excluded).

## Testing
- Core Rust tests (default):
  - `cargo test --bin storyvote --quiet`
- Browser smoke tests (optional, Playwright):
  - One-time setup:
    - Install Node.js and npm
    - `npm install`
    - `npx playwright install chromium`
  - Run browser smoke tests:
    - `npm run test:e2e`

Notes:
- Node/npm are not required to build or run the Rust server.
- Node/npm are only required for the optional browser smoke tests.

## Releases

Automated cross-platform builds are available via GitHub Actions:
- Trigger: Push a git tag (e.g., `git tag v0.1.0 && git push --tags`)
- Builds: Linux, macOS (Intel + ARM), Windows (all x86_64)
- Artifacts: Compressed binaries uploaded to GitHub Releases
- Action: `.github/workflows/release.yml`

## Notes
- If teammates cannot connect in LAN mode, check local firewall rules.
- Browser remembers last entered display name for convenience.
- Theme supports system auto mode and manual override (System/Light/Dark), persisted per browser.
- Cross-platform: Windows-specific build steps (icon/metadata) are conditionally compiled; Linux/macOS builds are identical.
