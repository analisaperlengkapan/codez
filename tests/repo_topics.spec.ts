import { test, expect } from '@playwright/test';

test.describe('Repository Topics Feature', () => {
  test('should create a repository and add topics', async ({ page }) => {
    // Navigate to create repo page
    await page.goto('http://127.0.0.1:8080/repo/create');

    // Make sure we are on the new repository page
    await expect(page.getByRole('heading', { name: 'Create New Repository' })).toBeVisible();

    // Fill in the repository details
    const repoName = 'test-topics-repo';
    await page.getByPlaceholder('Repository Name').fill(repoName);
    await page.getByPlaceholder('Description').fill('A repository to test topics feature.');

    // Select Public visibility (checkbox)
    // By default it is public, we leave the checkbox unchecked

    // Submit
    await page.getByRole('button', { name: 'Create' }).click();

    // Give it a moment to create the repository
    // We should wait for the redirect, but since the frontend might not redirect automatically in this mock,
    // we manually navigate after a short wait, but use a more robust check if possible.
    await page.waitForTimeout(500);
    await page.goto(`http://127.0.0.1:8080/repos/admin/${repoName}`);

    // Verify repository page loaded
    await expect(page.getByRole('heading', { name: `Repository: admin / ${repoName}` })).toBeVisible();

    // Verify "Edit Topics" button exists and click it
    const editTopicsBtn = page.getByRole('button', { name: 'Edit Topics' });
    await expect(editTopicsBtn).toBeVisible();
    await editTopicsBtn.click();

    // Fill the topics input and save
    const input = page.getByPlaceholder('rust, webassembly, leptos');
    await expect(input).toBeVisible();
    await input.fill('rust, webassembly, leptos');

    await page.getByRole('button', { name: 'Save' }).click();

    // Wait for the new topics to appear as links
    await expect(page.getByRole('link', { name: 'rust' })).toBeVisible();
    await expect(page.getByRole('link', { name: 'webassembly' })).toBeVisible();
    await expect(page.getByRole('link', { name: 'leptos' })).toBeVisible();
  });
});
