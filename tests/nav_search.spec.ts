import { test, expect } from '@playwright/test';

/**
 * The header search box is hidden at ≤768px, so a plain Search link must remain
 * reachable in the primary nav at those widths (and stay hidden on desktop,
 * where the box is present).
 */
test.describe('Global navigation search affordance', () => {
  test('desktop keeps the search box and hides the redundant link', async ({ page }) => {
    await page.setViewportSize({ width: 1280, height: 800 });
    await page.goto('/');

    await expect(page.locator('.global-search')).toBeVisible();
    await expect(page.locator('nav.nav-links a.search-link')).toBeHidden();
  });

  test('mobile exposes a usable Search link and no search box', async ({ page }) => {
    await page.setViewportSize({ width: 390, height: 844 });
    await page.goto('/');

    await expect(page.locator('.global-search')).toBeHidden();

    const searchLink = page.locator('nav.nav-links a.search-link');
    await expect(searchLink).toBeVisible();
    await expect(searchLink).toHaveText('Search');

    await searchLink.click();
    await expect(page).toHaveURL(/\/search$/);
    await expect(page.locator('.search-page')).toBeVisible();
  });
});
