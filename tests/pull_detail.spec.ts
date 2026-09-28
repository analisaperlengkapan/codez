import { test, expect } from '@playwright/test';

/**
 * Exercises the single-pull endpoint (`GET /repos/:owner/:repo/pulls/:index`)
 * that the detail page now relies on instead of filtering the list client-side.
 */
test.describe('Pull request detail', () => {
  test('list links to a detail page that loads the right pull', async ({ page }) => {
    await page.goto('/repos/admin/codeza/pulls');
    await expect(page.getByRole('heading', { name: 'Pull Requests for admin/codeza' })).toBeVisible();

    // The seeded pull shows up and links to its detail route.
    await page.getByRole('link', { name: /#1 First PR/ }).click();

    await expect(page).toHaveURL(/\/repos\/admin\/codeza\/pulls\/1$/);
    // The detail page fetches the pull directly; assert on its title.
    await expect(page.getByText('First PR').first()).toBeVisible({ timeout: 10000 });
  });

  test('unknown pull renders a not-found message rather than crashing', async ({ page }) => {
    const errors: string[] = [];
    page.on('pageerror', (err) => errors.push(err.message));

    await page.goto('/repos/admin/codeza/pulls/9999');
    await expect(page.locator('main.page')).toBeVisible();
    await expect(page.getByText('Pull Request not found')).toBeVisible({ timeout: 10000 });
    expect(errors).toEqual([]);
  });

  test('the detail endpoint returns the requested pull', async ({ request }) => {
    const ok = await request.get('/api/v1/repos/admin/codeza/pulls/1');
    expect(ok.ok()).toBeTruthy();
    expect((await ok.json()).title).toBe('First PR');

    const missing = await request.get('/api/v1/repos/admin/codeza/pulls/9999');
    expect(missing.ok()).toBeTruthy();
    expect(await missing.json()).toBeNull();
  });
});