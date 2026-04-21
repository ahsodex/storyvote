const { defineConfig } = require('@playwright/test');

module.exports = defineConfig({
  testDir: './tests/e2e',
  timeout: 30_000,
  expect: {
    timeout: 10_000,
  },
  use: {
    baseURL: 'http://127.0.0.1:8787',
    headless: true,
  },
  webServer: {
    command: 'cargo run -- --localhost-only --port 8787',
    url: 'http://127.0.0.1:8787/health',
    reuseExistingServer: true,
    timeout: 120_000,
  },
});
