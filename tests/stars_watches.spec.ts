import { test, expect } from '@playwright/test';

test.describe('Repository Stars and Watches Flow', () => {
  test('should allow user to toggle star and watch status', async ({ page }) => {
    // 1. Navigate to create repo page
    await page.goto('http://127.0.0.1:8080/repo/create');
    await expect(page.locator('h3')).toContainText('Create New Repository', { timeout: 10000 });

    // Create new repo
    const testRepoName = `star-watch-test-repo-${Date.now()}`;
    await page.fill('input[placeholder="Repository Name"]', testRepoName);
    await page.fill('input[placeholder="Description"]', 'Testing stars and watches');

    // Listen for response from creation
    const responsePromise = page.waitForResponse(response =>
      response.url().includes('/api/v1/user/repos') &&
      [201, 200, 409].includes(response.status())
    );
    await page.click('button:has-text("Create")');
    await responsePromise;

    // 2. Navigate to the repo manually
    await page.goto(`http://127.0.0.1:8080/repos/admin/${testRepoName}`);
    await expect(page.locator('h3')).toContainText(`Repository: admin / ${testRepoName}`, { timeout: 10000 });

    // Wait for data load
    await page.waitForTimeout(2000);

    const actionsDiv = page.locator('.repo-actions');

    // Fallback if already starred from a previous run or initial states
    const unstarButton = actionsDiv.getByRole('button', { name: /^Unstar \(/ }).first();
    if (await unstarButton.isVisible()) {
        await unstarButton.click();
        await page.waitForTimeout(1000);
    }

    const unwatchButton = actionsDiv.getByRole('button', { name: /^Unwatch \(/ }).first();
    if (await unwatchButton.isVisible()) {
        await unwatchButton.click();
        await page.waitForTimeout(1000);
    }

    await expect(actionsDiv.getByRole('button', { name: /^Star \(/ }).first()).toBeVisible({ timeout: 5000 });
    await expect(actionsDiv.getByRole('button', { name: /^Watch \(/ }).first()).toBeVisible({ timeout: 5000 });

    // 4. Star the repository
    await actionsDiv.getByRole('button', { name: /^Star \(/ }).first().click();
    await page.waitForTimeout(1000);
    await expect(actionsDiv.getByRole('button', { name: /^Unstar \(/ }).first()).toBeVisible({ timeout: 5000 });

    // 5. Watch the repository
    await actionsDiv.getByRole('button', { name: /^Watch \(/ }).first().click();
    await page.waitForTimeout(1000);
    await expect(actionsDiv.getByRole('button', { name: /^Unwatch \(/ }).first()).toBeVisible({ timeout: 5000 });

    // 6. Unstar the repository
    await actionsDiv.getByRole('button', { name: /^Unstar \(/ }).first().click();
    await page.waitForTimeout(1000);
    await expect(actionsDiv.getByRole('button', { name: /^Star \(/ }).first()).toBeVisible({ timeout: 5000 });

    // 7. Unwatch the repository
    await actionsDiv.getByRole('button', { name: /^Unwatch \(/ }).first().click();
    await page.waitForTimeout(1000);
    await expect(actionsDiv.getByRole('button', { name: /^Watch \(/ }).first()).toBeVisible({ timeout: 5000 });
  });
});
