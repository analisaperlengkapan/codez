import { test, expect } from '@playwright/test';

test.describe('Projects Feature', () => {
  test('should list and create projects', async ({ page }) => {
    await page.goto('http://127.0.0.1:8080/repos/admin/codeza/projects');

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
});
