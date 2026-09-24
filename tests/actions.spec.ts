import { test, expect } from '@playwright/test';

test.describe('CI/CD Actions End-to-End', () => {
    test.beforeEach(async ({ page }) => {
        // Navigate to the repository's actions workflow runs list
        // id 1 is the default CI workflow
        await page.goto('/repos/admin/codeza/actions/workflows/1');
    });

    test('should trigger a run and then delete it', async ({ page }) => {
        // We set up promises to wait for the responses before taking the action that triggers them
        const postResponsePromise = page.waitForResponse(resp => resp.url().includes('/api/v1/repos/') && resp.request().method() === 'POST');
        // Trigger a run
        await page.click('.run-workflow-btn');
        const postResponse = await postResponsePromise;
        const created = await postResponse.json();
        const runId = created.id as number;

        // A triggered run completes synchronously with step logs (there is no
        // background executor in the demo), so it renders as "success" — not a
        // run stuck in "queued" with no logs.
        const runItem = page.locator('.run-item').filter({ hasText: `Run #${runId} ` });
        await expect(runItem).toBeVisible();
        await expect(runItem.locator('.run-status')).toHaveText('success');
        // Regression guard: a triggered run must not stay "queued" with an empty
        // log viewer (previously only rerun/update ever set a status).
        await expect(runItem.locator('.log-viewer .log-step').first()).toBeVisible();

        const deleteResponsePromise = page.waitForResponse(resp => resp.url().includes('/api/v1/repos/') && resp.request().method() === 'DELETE');
        const getRunsAfterDeletePromise = page.waitForResponse(resp => resp.url().includes('/api/v1/repos/') && resp.url().includes('/runs') && resp.request().method() === 'GET');

        // A completed run offers Delete rather than Cancel.
        await runItem.locator('.delete-run-btn').click();
        await deleteResponsePromise;

        // Wait for list to fetch the updated state
        await getRunsAfterDeletePromise;
        await page.waitForTimeout(1000);

        // Force a reload to avoid leptos reactivity sync issues if it's lagging
        await page.reload();
        await page.waitForLoadState('networkidle');

        await expect(page.locator('.run-item').filter({ hasText: `Run #${runId} ` })).toHaveCount(0);
    });
});

