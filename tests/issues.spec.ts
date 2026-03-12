import { test, expect } from '@playwright/test';

test.describe('Issues Feature', () => {
  test('should list, create issues, and add a comment', async ({ page }) => {
    // Navigate to issues page
    await page.goto('http://127.0.0.1:8080/repos/admin/codeza/issues');

    // Make sure we are on the issues page
    await expect(page.getByRole('heading', { name: 'Issues for admin/codeza' })).toBeVisible();

    // Click 'New Issue' button
    await page.getByRole('button', { name: 'New Issue' }).click();

    // Fill in the issue details
    const issueTitle = 'Test Issue from Playwright ' + Date.now();
    await page.getByPlaceholder('Title').fill(issueTitle);
    await page.getByPlaceholder('Body').fill('This is an issue created via an automated Playwright test.');

    // Submit the issue
    await page.getByRole('button', { name: 'Create Issue' }).click();

    // Wait for the new issue to appear in the list
    const newIssueLink = page.getByRole('link', { name: issueTitle }).first();
    await expect(newIssueLink).toBeVisible();

    // Click the new issue to view its details
    await newIssueLink.click();

    // Check if the issue detail page rendered correctly by looking for the title
    await expect(page.getByRole('heading', { name: issueTitle })).toBeVisible();

    // Verify 'Comments' section is visible
    await expect(page.getByRole('heading', { name: 'Comments' })).toBeVisible();

    // Add a comment
    const commentText = 'This is a test comment from Playwright.';
    await page.getByPlaceholder('Leave a comment').fill(commentText);
    await page.getByRole('button', { name: 'Comment' }).click();

    // Verify comment appears in the UI
    await expect(page.getByText(commentText)).toBeVisible();
  });
});
