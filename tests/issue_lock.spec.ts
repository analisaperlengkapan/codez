import { test, expect } from '@playwright/test';

test.describe('Issue Lock Conversation Feature', () => {
  test('should lock an issue and prevent commenting, then unlock and allow commenting', async ({ page }) => {
    // Navigate to issues page
    await page.goto('http://127.0.0.1:8080/repos/admin/codeza/issues');

    await page.waitForTimeout(1000);

    // Make sure we are on the issues page
    await expect(page.getByRole('heading', { name: 'Issues for admin/codeza' })).toBeVisible({ timeout: 10000 });

    // Click 'New Issue' button
    await page.getByRole('button', { name: 'New Issue' }).click();

    // Fill in the issue details
    const issueTitle = 'Locking Test Issue ' + Date.now();
    await page.getByPlaceholder('Title').fill(issueTitle);
    await page.getByPlaceholder('Body').fill('This is a test issue for locking conversation.');

    // Submit the issue
    await page.getByRole('button', { name: 'Create Issue' }).click();

    // Wait for the new issue to appear in the list
    const newIssueLink = page.getByRole('link', { name: issueTitle }).first();
    await expect(newIssueLink).toBeVisible({ timeout: 10000 });

    // Click the new issue to view its details
    await newIssueLink.click();

    // Check if the issue detail page rendered correctly by looking for the title
    await expect(page.getByRole('heading', { name: issueTitle })).toBeVisible();

    // Verify 'Comments' section is visible
    await expect(page.getByRole('heading', { name: 'Comments' })).toBeVisible();

    // Add a comment to ensure commenting works initially
    const firstCommentText = 'First comment before locking.';
    await page.getByPlaceholder('Leave a comment').fill(firstCommentText);
    await page.getByRole('button', { name: 'Comment' }).click();
    await expect(page.getByText(firstCommentText)).toBeVisible();

    // Lock the conversation
    const lockBtn = page.getByRole('button', { name: 'Lock Conversation' });
    await expect(lockBtn).toBeVisible();
    await lockBtn.click();

    // Wait for the UI to update: The "Unlock Conversation" button should appear
    const unlockBtn = page.getByRole('button', { name: 'Unlock Conversation' });
    await expect(unlockBtn).toBeVisible({ timeout: 10000 });

    // Verify that the comment form is gone and the "This conversation is locked" message is visible
    await expect(page.getByText('This conversation is locked')).toBeVisible();
    await expect(page.getByRole('button', { name: 'Comment' })).toBeHidden();
    await expect(page.getByPlaceholder('Leave a comment')).toBeHidden();

    // Unlock the conversation
    await unlockBtn.click();

    // Wait for the UI to update: The "Lock Conversation" button should appear
    await expect(lockBtn).toBeVisible({ timeout: 10000 });

    // Verify the comment form is back
    await expect(page.getByText('This conversation is locked')).toBeHidden();
    await expect(page.getByRole('button', { name: 'Comment' })).toBeVisible();

    // Add a comment to ensure commenting works again
    const secondCommentText = 'Second comment after unlocking.';
    await page.getByPlaceholder('Leave a comment').fill(secondCommentText);
    await page.getByRole('button', { name: 'Comment' }).click();
    await expect(page.getByText(secondCommentText)).toBeVisible();
  });
});
