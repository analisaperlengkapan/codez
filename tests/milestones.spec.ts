import { test, expect } from '@playwright/test';

test.describe('Milestones Feature', () => {
  test('should list and create milestones', async ({ page }) => {
    await page.goto('http://127.0.0.1:8080/repos/admin/codeza/milestones');

    await expect(page.getByRole('heading', { name: 'Milestones' })).toBeVisible();

    await page.getByPlaceholder('Title').fill('v2.0 Beta');
    await page.getByRole('button', { name: 'Create' }).click();

    // The component doesn't automatically refresh on create, so we can verify the API or reload
    await page.reload();

    await expect(page.getByRole('link', { name: 'v2.0 Beta' }).first()).toBeVisible();
  });
});
