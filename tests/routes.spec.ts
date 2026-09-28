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
    // `slug` is a bare section; `path` covers sub-views that map onto a tab
    // (edit/search belong to Code, compare belongs to Pull requests).
    const sections: Array<{ path: string; label: string }> = [
      { path: 'issues', label: 'Issues' },
      { path: 'pulls', label: 'Pull requests' },
      { path: 'commits', label: 'Commits' },
      { path: 'security', label: 'Security' },
      { path: 'settings', label: 'Settings' },
      { path: 'src/src/main.rs', label: 'Code' },
      { path: 'edit/README.md', label: 'Code' },
      { path: 'search', label: 'Code' },
      { path: 'compare', label: 'Pull requests' },
    ];
    for (const { path, label } of sections) {
      await page.goto(`/repos/admin/codeza/${path}`);
      const active = page.locator('nav.repo-nav a.active');
      await expect(active, `active tab on /${path}`).toHaveText(label);
      await expect(active, `aria-current on /${path}`).toHaveAttribute('aria-current', 'page');
    }
  });

  test('the shared repository header renders on every sub-page', async ({ page }) => {
    const paths = [
      '/repos/admin/codeza',
      '/repos/admin/codeza/issues',
      '/repos/admin/codeza/pulls',
      '/repos/admin/codeza/actions',
      '/repos/admin/codeza/wiki',
      '/repos/admin/codeza/settings',
    ];
    for (const path of paths) {
      await page.goto(path);
      const header = page.locator('.repo-header');
      await expect(header).toBeVisible();
      await expect(header.locator('h3')).toContainText('Repository: admin / codeza');
      await expect(header.getByRole('button', { name: 'Fork' })).toBeVisible();
      await expect(page.locator('nav.repo-nav')).toBeVisible();
    }
  });
});
