const { test, expect } = require('@playwright/test');

test('page renders share URL and join transitions into app panel', async ({ page }) => {
  await page.goto('/');

  const shareUrl = page.locator('#shareUrl');
  await expect(shareUrl).not.toBeEmpty();
  await expect(shareUrl).toContainText('127.0.0.1:8787');

  await page.fill('#nameInput', 'SmokeUser');
  await page.click('#joinBtn');

  await expect(page.locator('#joinPanel')).toBeHidden();
  await expect(page.locator('#appPanel')).toBeVisible();
  await expect(page.locator('#meLabel')).toHaveText('SmokeUser');
});

test('refresh disconnects and allows same-name rejoin', async ({ page }) => {
  await page.goto('/');

  await page.fill('#nameInput', 'ReconnectUser');
  await page.click('#joinBtn');

  await expect(page.locator('#appPanel')).toBeVisible();
  await expect(page.locator('#joinPanel')).toBeHidden();

  await page.reload();

  await expect(page.locator('#joinPanel')).toBeVisible();
  await expect(page.locator('#appPanel')).toBeHidden();

  await page.fill('#nameInput', 'ReconnectUser');
  await page.click('#joinBtn');

  await expect(page.locator('#joinPanel')).toBeHidden();
  await expect(page.locator('#appPanel')).toBeVisible();
  await expect(page.locator('#meLabel')).toHaveText('ReconnectUser');
});

test('theme override persists across reload', async ({ page }) => {
  await page.goto('/');

  const themeSelect = page.locator('#themeSelect');
  await themeSelect.selectOption('dark');

  await expect(themeSelect).toHaveValue('dark');
  await expect(page.locator('html')).toHaveAttribute('data-theme', 'dark');

  await page.reload();

  await expect(page.locator('#themeSelect')).toHaveValue('dark');
  await expect(page.locator('html')).toHaveAttribute('data-theme', 'dark');
});

test('copy share URL button writes URL and shows copied status', async ({ page }) => {
  await page.addInitScript(() => {
    window.__copiedValue = '';
    Object.defineProperty(navigator, 'clipboard', {
      configurable: true,
      value: {
        writeText: async (value) => {
          window.__copiedValue = value;
        },
      },
    });
  });

  await page.goto('/');

  await page.click('#copyShareBtn');

  await expect(page.locator('#copyShareStatus')).toHaveText('Copied.');
  const copiedValue = await page.evaluate(() => window.__copiedValue);
  expect(copiedValue).toContain('127.0.0.1:8787');
});

test('reveal summary shows average and median for numeric votes', async ({ page }) => {
  await page.goto('/');

  await page.fill('#nameInput', 'SummaryHost');
  await page.click('#joinBtn');

  await expect(page.locator('#summaryStatus')).toHaveText('Summary appears after reveal.');

  await page.locator('.card', { hasText: '8' }).click();
  await page.click('#revealBtn');

  await expect(page.locator('#summaryStatus')).toContainText('Summary: Avg 8 | Median 8 | Numeric 1/1');
});
