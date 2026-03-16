import { test, expect } from '@playwright/test';

test.describe('Discussions Feature', () => {
  test('should list discussions and create a new discussion', async ({ page }) => {
    // Go to the repository discussions page (using mock owner 'admin' and repo 'codeza')
    await page.goto('http://127.0.0.1:8080/repos/admin/codeza/discussions');

    await page.waitForTimeout(1000);

    // Check if the discussions header is visible
    await expect(page.getByRole('heading', { name: 'Discussions' })).toBeVisible({ timeout: 10000 });

    // Click 'New Discussion' button
    const newButton = page.getByRole('button', { name: 'New Discussion' });
    await newButton.click();

    // Fill the form
    await page.getByPlaceholder('Title').fill('New Playwright Discussion');
    await page.getByPlaceholder('Body').fill('This discussion was created by Playwright tests.');
    await page.getByRole('combobox').selectOption('Ideas');

    // Submit
    await page.getByRole('button', { name: 'Start Discussion' }).click();

    // Verify it appeared in the list
    await expect(page.getByRole('link', { name: 'New Playwright Discussion' }).first()).toBeVisible();

    // Navigate to discussion detail
    await page.getByRole('link', { name: 'New Playwright Discussion' }).first().click();

    // Check details
    await expect(page.getByRole('heading', { name: 'New Playwright Discussion' })).toBeVisible();
    await expect(page.getByText('This discussion was created by Playwright tests.')).toBeVisible();

    // Add a comment
    await page.getByPlaceholder('Write your comment here...').fill('Playwright test comment.');
    await page.getByRole('button', { name: 'Post Comment' }).click();

    // Check if comment appeared
    await expect(page.getByText('Playwright test comment.').first()).toBeVisible();
  });
});
