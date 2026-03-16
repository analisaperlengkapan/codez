import { test, expect } from '@playwright/test';

test.describe('Branch Protection Feature', () => {
  test('should create a branch protection rule and display it in the list', async ({ page }) => {
    // Navigate to the repository branch settings page
    // Using admin/codeza since it should be created by mock data or earlier tests
    await page.goto('http://127.0.0.1:8080/repos/admin/codeza/settings/branches');

    // Wait for leptos router to settle
    await page.waitForTimeout(2000);

    // Make sure we are on the branch protection page
    await expect(page.getByRole('heading', { name: 'Add Branch Protection Rule' })).toBeVisible({ timeout: 10000 });

    // Use a unique name for the branch to avoid conflicts
    const branchName = 'protected-branch-' + Date.now();

    // Fill the branch protection form
    await page.getByPlaceholder('e.g. main, release-*').fill(branchName);

    // Check 'Enable Push'
    await page.getByLabel('Enable Push', { exact: true }).check();

    // Fill 'Required Status Checks'
    await page.getByPlaceholder('e.g. ci/test, security-scan').fill('ci/test, lint');

    // Submit the form
    const postResponsePromise = page.waitForResponse(resp =>
      resp.url().includes('/api/v1/repos/admin/codeza/branch_protections') &&
      resp.request().method() === 'POST'
    );
    await page.getByRole('button', { name: 'Create Rule' }).click();

    // Wait for the API call to complete
    const response = await postResponsePromise;
    expect(response.status()).toBe(201); // Created

    // Wait for leptos router to settle and resource to refetch
    await page.waitForTimeout(1000);

    // Verify the newly created rule appears in the "Protected Branches" list
    // It should include the branch name and the checks we specified
    await expect(page.locator('.protected-branches')).toContainText(branchName);
    await expect(page.locator('.protected-branches')).toContainText('Push: true');
    await expect(page.locator('.protected-branches')).toContainText('Checks: ci/test, lint');
  });
});
