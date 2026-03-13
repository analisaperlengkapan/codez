import { test, expect } from '@playwright/test';

test.describe('CI/CD Actions End-to-End', () => {
    test.beforeEach(async ({ page }) => {
        // Navigate to the repository's actions workflow runs list
        // id 1 is the default CI workflow
        await page.goto('http://127.0.0.1:8080/repos/admin/codeza/actions/workflows/1');
    });

    test('should trigger, cancel, and delete a workflow run', async ({ page }) => {
        // We set up promises to wait for the responses before taking the action that triggers them
        const postResponsePromise = page.waitForResponse(resp => resp.url().includes('/api/v1/repos/') && resp.request().method() === 'POST');
        // Trigger a run
        await page.click('.run-workflow-btn');
        await postResponsePromise;

        // Wait for the new run to appear (status 'queued')
        // Wait for the new run to appear (status 'queued')
        // Locate the run item with the 'queued' status directly
        const runItem = page.locator('.run-item').filter({ has: page.locator('.run-status', { hasText: 'queued' }) }).first();
        await expect(runItem).toBeVisible();
        await expect(runItem.locator('.run-status')).toHaveText('queued');

        const patchResponsePromise = page.waitForResponse(resp => resp.url().includes('/api/v1/repos/') && resp.request().method() === 'PATCH');
        // Cancel the run
        await runItem.locator('.cancel-run-btn').click();
        await patchResponsePromise;

        // Wait for the UI to update to 'cancelled' state
        // This is safe because we just got the PATCH response back successfully
        const cancelledRunItem = page.locator('.run-item').filter({ has: page.locator('.run-status', { hasText: 'cancelled' }) }).first();

        // Wait for status to change to 'cancelled'
        await expect(cancelledRunItem.locator('.run-status')).toHaveText('cancelled');

        // Delete button should now be visible instead of cancel
        await expect(cancelledRunItem.locator('.delete-run-btn')).toBeVisible();

        const deleteResponsePromise = page.waitForResponse(resp => resp.url().includes('/api/v1/repos/') && resp.request().method() === 'DELETE');
        const getRunsAfterDeletePromise = page.waitForResponse(resp => resp.url().includes('/api/v1/repos/') && resp.url().includes('/runs') && resp.request().method() === 'GET');

        // Delete the run
        await cancelledRunItem.locator('.delete-run-btn').click();
        await deleteResponsePromise;

        // Wait for list to fetch the updated state and ensure the list is empty (or has fewer items)
        await getRunsAfterDeletePromise;
        // Wait an extra moment for Reactivity to re-render the list after the fetch
        await page.waitForTimeout(2000);

        // Force a reload to avoid leptos reactivity sync issues if it's lagging
        await page.reload();
        await page.waitForLoadState('networkidle');

        await expect(cancelledRunItem).not.toBeVisible({ timeout: 10000 });
    });
});
