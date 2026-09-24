import { test, expect } from '@playwright/test';

/**
 * Every client route must render without an uncaught exception and show the
 * expected page shell. This is a broad smoke net so a broken resource or a
 * renamed route is caught before the narrower feature specs run.
 */
const ROUTES: Array<{ path: string; heading?: RegExp }> = [
  { path: '/' },
  { path: '/explore' },
  { path: '/admin' },
  { path: '/admin/users' },
  { path: '/search' },
  { path: '/notifications' },
  { path: '/login' },
  { path: '/register' },
  { path: '/settings/profile' },
  { path: '/repo/create' },
  { path: '/repo/migrate' },
  { path: '/org/create' },
  { path: '/users/admin' },
  { path: '/users/admin/followers' },
  { path: '/users/admin/following' },
  { path: '/orgs/codeza-org' },
  { path: '/packages/admin' },
  { path: '/packages/admin/cargo/my-lib/1.0.0' },
  { path: '/repos/admin/codeza' },
  { path: '/repos/admin/codeza/src/src/main.rs' },
  { path: '/repos/admin/codeza/commits' },
  { path: '/repos/admin/codeza/commits/1' },
  { path: '/repos/admin/codeza/branches' },
  { path: '/repos/admin/codeza/tags' },
  { path: '/repos/admin/codeza/search' },
  { path: '/repos/admin/codeza/compare' },
  { path: '/repos/admin/codeza/issues' },
  { path: '/repos/admin/codeza/issues/1' },
  { path: '/repos/admin/codeza/pulls' },
  { path: '/repos/admin/codeza/pulls/1' },
  { path: '/repos/admin/codeza/labels' },
  { path: '/repos/admin/codeza/milestones' },
  { path: '/repos/admin/codeza/milestones/1' },
  { path: '/repos/admin/codeza/releases' },
  { path: '/repos/admin/codeza/releases/new' },
  { path: '/repos/admin/codeza/releases/1' },
  { path: '/repos/admin/codeza/actions' },
  { path: '/repos/admin/codeza/actions/workflows/1' },
  { path: '/repos/admin/codeza/security' },
  { path: '/repos/admin/codeza/pulse' },
  { path: '/repos/admin/codeza/projects' },
  { path: '/repos/admin/codeza/discussions' },
  { path: '/repos/admin/codeza/wiki' },
  { path: '/repos/admin/codeza/wiki/pages/Home' },
  { path: '/repos/admin/codeza/wiki/pages/Home/edit' },
  { path: '/repos/admin/codeza/edit/README.md' },
  { path: '/repos/admin/codeza/collaborators' },
  { path: '/repos/admin/codeza/settings' },
  { path: '/repos/admin/codeza/settings/branches' },
  { path: '/repos/admin/codeza/settings/webhooks' },
  { path: '/repos/admin/codeza/settings/secrets' },
  { path: '/repos/admin/codeza/settings/keys' },
  { path: '/repos/admin/codeza/settings/lfs' },
];

test.describe('Route smoke coverage', () => {
  for (const { path } of ROUTES) {
    test(`renders ${path}`, async ({ page }) => {
      const errors: string[] = [];
      page.on('pageerror', (err) => errors.push(err.message));

      const response = await page.goto(path);
      expect(response?.status(), `${path} should return < 400`).toBeLessThan(400);

      // Wait for the SPA shell + any Suspense fallbacks to settle.
      await expect(page.locator('header.app-header')).toBeVisible();
      await expect(page.locator('main.page')).toBeVisible();
      // Give async resources a moment so a failed fetch surfaces as an error.
      await page.waitForTimeout(400);

      expect(errors, `uncaught errors on ${path}`).toEqual([]);
    });
  }
});

test.describe('Repository chrome', () => {
  test('RepoNav marks the active section on every sub-page', async ({ page }) => {
    const sections: Array<[string, string]> = [
      ['issues', 'Issues'],
      ['pulls', 'Pull requests'],
      ['commits', 'Commits'],
      ['settings', 'Settings'],
    ];
    for (const [slug, label] of sections) {
      await page.goto(`/repos/admin/codeza/${slug}`);
      const active = page.locator('nav.repo-nav a.active');
      await expect(active).toHaveText(label);
      await expect(active).toHaveAttribute('aria-current', 'page');
    }
  });
});
