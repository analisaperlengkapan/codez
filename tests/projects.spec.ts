import { test, expect } from '@playwright/test';

test.describe('Projects Feature', () => {
  test('should list and create projects', async ({ page }) => {
    await page.goto('/repos/admin/codeza/projects');

    await expect(page.getByRole('heading', { name: 'Projects' })).toBeVisible();

    await page.getByRole('button', { name: 'New Project' }).click();

    await page.getByPlaceholder('Project Title').fill('Roadmap');
    await page.getByPlaceholder('Description').fill('Feature roadmap');
    await page.getByRole('button', { name: 'Create Project' }).click();

    await expect(page.getByRole('link', { name: 'Roadmap' }).first()).toBeVisible();

    await page.getByRole('link', { name: 'Roadmap' }).first().click();
    await expect(page.getByRole('heading', { name: 'Roadmap' })).toBeVisible();

    // Add a column
    await page.getByPlaceholder('New Column').fill('To Do');
    await page.getByRole('button', { name: 'Add Column' }).click();

    await expect(page.getByRole('heading', { name: 'To Do' }).first()).toBeVisible();
  });

  test('closing a project flips the status and button without a reload', async ({ page }) => {
    await page.goto('/repos/admin/codeza/projects');

    await page.getByRole('button', { name: 'New Project' }).click();
    const title = 'Board ' + Date.now();
    await page.getByPlaceholder('Project Title').fill(title);
    await page.getByRole('button', { name: 'Create Project' }).click();
    await page.getByRole('link', { name: title }).first().click();

    await expect(page.getByRole('heading', { name: title })).toBeVisible();
    await page.getByRole('button', { name: 'Close Project' }).click();

    // The resource must re-fetch, otherwise the old title/button stay on screen.
    await expect(page.getByRole('button', { name: 'Reopen Project' })).toBeVisible();
    await expect(page.getByRole('heading', { name: `${title} (Closed)` })).toBeVisible();

    await page.getByRole('button', { name: 'Reopen Project' }).click();
    await expect(page.getByRole('button', { name: 'Close Project' })).toBeVisible();
  });
});
