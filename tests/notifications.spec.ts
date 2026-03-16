import { test, expect } from '@playwright/test';

test.describe('Notifications Feature', () => {
  test('should trigger and display a notification when a new issue is created, and allow marking it as read', async ({ page }) => {
    // Navigate to the repository issues page
    await page.goto('http://127.0.0.1:8080/repos/admin/codeza/issues');

    // Create a new issue
    const newIssueTitle = `Notification Test Issue ${Date.now()}`;
    await page.getByRole('button', { name: 'New Issue' }).click();
    await page.getByPlaceholder('Title').fill(newIssueTitle);
    await page.getByPlaceholder('Body').fill('Testing notifications.');
    await page.getByRole('button', { name: 'Create Issue' }).click();

    // Wait for the new issue to appear in the list
    const newIssueLink = page.getByRole('link', { name: newIssueTitle }).first();
    await expect(newIssueLink).toBeVisible({ timeout: 10000 });

    // Navigate to Dashboard
    await page.goto('http://127.0.0.1:8080/');

    // Ensure the notifications list is loaded
    await expect(page.getByRole('heading', { name: 'Notifications' })).toBeVisible({ timeout: 10000 });

    // Look for the notification
    // The subject format is "New issue in {repo_name}: {title}"
    const expectedSubject = `New issue in codeza: ${newIssueTitle}`;
    const notificationItem = page.locator('li', { hasText: expectedSubject });
    await expect(notificationItem).toBeVisible();

    // It should say (Unread) initially
    await expect(notificationItem).toContainText('(Unread)');

    // Click 'Mark Read'
    const markReadBtn = notificationItem.getByRole('button', { name: 'Mark Read' });
    await markReadBtn.click();

    // Since our mock frontend simply calls the API but doesn't immediately refresh the list on click without a reload/signal change,
    // we should wait and reload the dashboard, or maybe the frontend handles it dynamically.
    // Let's reload the page to ensure the state is refetched.
    await page.goto('http://127.0.0.1:8080/');
    await expect(page.getByRole('heading', { name: 'Notifications' })).toBeVisible({ timeout: 10000 });

    const updatedNotificationItem = page.locator('li', { hasText: expectedSubject });
    await expect(updatedNotificationItem).toBeVisible();
    await expect(updatedNotificationItem).toContainText('(Read)');

    // The 'Mark Read' button should no longer be present
    await expect(updatedNotificationItem.getByRole('button', { name: 'Mark Read' })).not.toBeVisible();
  });
});
