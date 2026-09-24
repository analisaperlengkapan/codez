import { test, expect } from '@playwright/test';

/**
 * The backend falls back to serving the SPA shell for client routes, but must
 * not use that fallback to leak local files (path traversal) or to answer a
 * missing build asset with HTML.
 *
 * These HTTP-level checks are defense-in-depth: in CI the suite runs behind
 * `trunk serve`, which fronts the same routes, so the authoritative
 * enforcement tests for the fallback are the `static_asset_tests` unit tests in
 * `crates/backend/src/main.rs`. Keeping an end-to-end assertion here also
 * covers the `cargo run -p backend` topology where the backend serves assets
 * directly.
 */
test.describe('Static fallback', () => {
  test('never discloses files outside the static root', async ({ request }) => {
    // `--path-as-is` equivalents: raw and percent-encoded traversal attempts.
    for (const path of [
      '/../Cargo.toml',
      '/..%2fCargo.toml',
      '/%2e%2e/Cargo.toml',
      '/sub/../../etc/passwd',
    ]) {
      const res = await request.get(path);
      expect([200, 400, 404]).toContain(res.status());

      const body = await res.text();
      expect(body, `${path} must not leak Cargo.toml`).not.toContain('[package]');
      expect(body, `${path} must not leak /etc/passwd`).not.toContain('root:x:');
      if (res.status() === 200) {
        // Whatever is served must be the SPA shell, not a file.
        expect(res.headers()['content-type']).toContain('text/html');
        expect(body).toContain('<!DOCTYPE html>');
      }
    }
  });

  test('unknown API paths stay 404 JSON-free', async ({ request }) => {
    const res = await request.get('/api/v1/does-not-exist');
    expect(res.status()).toBe(404);
    expect(await res.text()).not.toContain('<!DOCTYPE html>');
  });

  // The `404` for missing build assets and the top-level asset classification
  // are covered by `static_asset_tests` in `crates/backend/src/main.rs`. They
  // are not asserted here because the e2e suite runs behind `trunk serve`, whose
  // own SPA fallback answers missing assets before the backend sees them.

  test('dotted repository deep links still serve the SPA shell', async ({ page }) => {
    // `library.js` ends in `.js` but is a route segment, not a missing asset.
    const res = await page.goto('/repos/admin/library.js');
    expect(res?.status()).toBeLessThan(400);
    await expect(page.locator('header.app-header')).toBeVisible();
    await expect(page.locator('main.page')).toBeVisible();
  });
});

test.describe('Global search', () => {
  test('filtering narrows results and clearing the box clears them', async ({ page }) => {
    await page.goto('/');
    const input = page.locator('header.app-header form input[name="q"]');

    // A term that matches a seeded repo.
    await input.fill('codeza');
    await input.press('Enter');
    await expect(page).toHaveURL(/\/search\?q=codeza/);

    const results = page.locator('.search-results .item-list li');
    await expect(results.first()).toBeVisible();
    expect(await results.count()).toBeGreaterThan(0);
    await expect(page.locator('.search-input-flex')).toHaveValue('codeza');

    // Clearing the header box navigates to `/search?q=` and must reset the page
    // rather than leaving the previous query and results on screen.
    await input.fill('');
    await input.press('Enter');
    await expect(page).toHaveURL(/\/search\?q=$/);
    await expect(page.locator('.search-input-flex')).toHaveValue('');
  });
});
