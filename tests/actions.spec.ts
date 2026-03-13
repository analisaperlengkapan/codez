import { test, expect } from '@playwright/test';

test.describe('CI/CD Actions End-to-End', () => {
    test.beforeEach(async ({ page }) => {
        // Navigate to the repository's actions workflow runs list
        // id 1 is the default CI workflow
        await page.goto('http://127.0.0.1:8080/repos/admin/codeza/actions/workflows/1');
    });

    test('should trigger, cancel, and delete a workflow run', async ({ page }) => {
        // Trigger a run
        await page.click('.run-workflow-btn');

        // Wait for the new run to appear (status 'queued')
        const runItem = page.locator('.run-item').first();
        await expect(runItem).toBeVisible();
        await expect(runItem.locator('.run-status')).toHaveText('queued');

        // Cancel the run
        await runItem.locator('.cancel-run-btn').click();

        // Wait for status to change to 'cancelled'
        await expect(runItem.locator('.run-status')).toHaveText('cancelled');
        // Delete button should now be visible instead of cancel
        await expect(runItem.locator('.delete-run-btn')).toBeVisible();

        // Delete the run
        await runItem.locator('.delete-run-btn').click();

        // Ensure the run is deleted
        await expect(runItem).not.toBeVisible();
    });
});
