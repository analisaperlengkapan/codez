import { test, expect } from '@playwright/test';

test.describe('Pull Requests Feature', () => {
  test('should list pull requests, view PR, and add a review', async ({ page, request }) => {
    // We will use the request context to create a new PR via API first.
    // The verify_reviews.py script creates PR on admin/codeza using head="feature", base="main".
    // Let's do the same here.
    const prTitle = 'Automated PR ' + Date.now();

    // Create PR
    const prResponse = await request.post(`http://127.0.0.1:3000/api/v1/repos/admin/codeza/pulls`, {
      data: {
        title: prTitle,
        head: 'feature',
        base: 'main',
        body: 'Testing pull request feature from Playwright',
      }
    });

    // Log response if it fails
    if (!prResponse.ok()) {
      console.log(await prResponse.text());
    }
    expect(prResponse.ok()).toBeTruthy();

    // Now navigate to the pull requests page
    // Wait for navigation instead of relying on it directly right after API call in case of delays
    await page.goto(`http://127.0.0.1:8080/repos/admin/codeza/pulls`, { waitUntil: 'networkidle' });

    // Make sure we are on the pull requests page
    await expect(page.getByRole('heading', { name: `Pull Requests for admin/codeza` })).toBeVisible({ timeout: 10000 });

    // Verify the newly created pull request is listed
    const prLink = page.getByRole('link', { name: prTitle }).first();
    await expect(prLink).toBeVisible();

    // Click the pull request link to view details
    await prLink.click();

    // Check if the detail page rendered correctly
    await expect(page.getByRole('heading', { name: prTitle })).toBeVisible();

    // Verify the 'Files Changed' section
    await expect(page.getByRole('heading', { name: 'Files Changed' })).toBeVisible();

    // Add a review comment
    const reviewText = 'This is a review comment from Playwright.';
    await page.getByPlaceholder('Leave a comment').fill(reviewText);

    // Submit review (Comment)
    await page.getByRole('button', { name: 'Comment' }).click();

    // Verify review comment appears in the UI
    await expect(page.getByText(reviewText)).toBeVisible();

    // Also test adding an 'Approve' review
    await page.getByPlaceholder('Leave a comment').fill('Looks good, approved.');
    await page.getByRole('button', { name: 'Approve' }).click();

    // Check for the exact match of APPROVED state
    await expect(page.getByText('APPROVED', { exact: true }).first()).toBeVisible();
  });
});
