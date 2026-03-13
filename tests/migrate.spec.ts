import { test, expect } from '@playwright/test';

test.describe('Repository Migration Feature', () => {
  test('should migrate a repository and display imported data', async ({ page }) => {
    // Navigate to migrate repo page
    await page.goto('http://127.0.0.1:8080/repo/migrate');

    // Make sure we are on the migrate repository page
    await expect(page.getByRole('heading', { name: 'Migrate Repository' })).toBeVisible();

    // Use a unique name for the repository to avoid conflicts
    const repoName = 'migrated-repo-' + Date.now();

    // Fill the migration form
    await page.locator('select').selectOption('github');
    await page.getByPlaceholder('Clone URL').fill('https://github.com/example/repo.git');
    await page.getByPlaceholder('Repository Name').fill(repoName);

    // Submit the migration
    const postResponsePromise = page.waitForResponse(resp => resp.url().includes('/api/v1/repos/migrate') && resp.request().method() === 'POST');
    await page.getByRole('button', { name: 'Migrate' }).click();

    // Wait for the migration API call to complete
    const response = await postResponsePromise;
    expect(response.status()).toBe(201);

    // After migration, it should redirect to the repository page
    await page.waitForURL(`http://127.0.0.1:8080/repos/admin/${repoName}`);
    await expect(page.getByRole('heading', { name: `admin / ${repoName}` })).toBeVisible();

    // Wait for leptos router to settle
    await page.waitForTimeout(2000);

    // Verify mock issues were imported correctly
    await page.goto(`http://127.0.0.1:8080/repos/admin/${repoName}/issues`);
    await expect(page.getByRole('heading', { name: `Issues for admin/${repoName}` })).toBeVisible();
    await expect(page.locator('body')).toContainText('Imported Issue 1');

    // Wait for leptos router to settle
    await page.waitForTimeout(2000);

    // Verify mock pull requests were imported correctly
    await page.goto(`http://127.0.0.1:8080/repos/admin/${repoName}/pulls`);
    await expect(page.getByRole('heading', { name: `Pull Requests for admin/${repoName}` })).toBeVisible({ timeout: 10000 });
    await expect(page.locator('body')).toContainText('Imported PR 1');
  });
});
