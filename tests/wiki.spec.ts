import { test, expect } from '@playwright/test';

test.describe('Wiki Feature', () => {
  test('should list, create, and edit a wiki page', async ({ page }) => {
    // Navigate to the repository wiki page
    await page.goto('http://127.0.0.1:8080/repos/admin/codeza/wiki');

    // Make sure we are on the wiki page
    await expect(page.getByRole('heading', { name: 'Pages' })).toBeVisible();

    // Verify default pages are listed
    await expect(page.getByRole('list').getByRole('link', { name: 'Home' })).toBeVisible();
    await expect(page.getByRole('link', { name: 'Installation' })).toBeVisible();

    // Wait for content area to load 'Home' by default
    await expect(page.locator('.wiki-header h3')).toHaveText('Home');

    // Navigate to create a new page
    await page.goto('http://127.0.0.1:8080/repos/admin/codeza/wiki/pages/NewPage/edit');
    await expect(page.getByRole('heading', { name: 'Editing NewPage' })).toBeVisible();

    // Fill in the new wiki page details
    const newPageContent = 'This is a new wiki page created by Playwright.';
    await page.locator('textarea').fill(newPageContent);
    await page.getByPlaceholder('Commit Message').fill('Added NewPage');

    // Submit the new wiki page
    await page.getByRole('button', { name: 'Save Page' }).click();

    // The frontend does not redirect automatically in the mock, so we wait briefly and navigate manually
    await page.waitForTimeout(1000);
    await page.goto('http://127.0.0.1:8080/repos/admin/codeza/wiki/pages/NewPage');

    // Wait for network idle to ensure data is fetched
    await page.waitForLoadState('networkidle');

    // Verify the new page was created and content is visible
    await expect(page.locator('.wiki-header h3')).toHaveText('NewPage');
    await expect(page.locator('.wiki-content pre')).toHaveText(newPageContent);

    // Verify it appears in the sidebar list
    await expect(page.getByRole('link', { name: 'NewPage' })).toBeVisible();

    // Now edit the page
    await page.getByRole('link', { name: 'Edit' }).click();
    await expect(page.getByRole('heading', { name: 'Editing NewPage' })).toBeVisible();

    const updatedPageContent = 'This content has been updated by Playwright.';
    await page.locator('textarea').fill(updatedPageContent);
    await page.getByPlaceholder('Commit Message').fill('Updated NewPage');

    // Submit the update
    await page.getByRole('button', { name: 'Save Page' }).click();

    // The frontend doesn't redirect so we wait and navigate
    await page.waitForTimeout(2000);
    await page.goto('http://127.0.0.1:8080/repos/admin/codeza/wiki/pages/NewPage');
    await page.waitForLoadState('networkidle');

    // Wait and retry verify the page was updated
    await expect(page.locator('.wiki-content pre')).toHaveText(updatedPageContent, { timeout: 10000 });
  });
});
