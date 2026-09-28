import { test, expect } from '@playwright/test';

test.describe('Milestones Feature', () => {
  test('should list and create milestones', async ({ page }) => {
    await page.goto('/repos/admin/codeza/milestones');

    await expect(page.getByRole('heading', { name: 'Milestones' })).toBeVisible();

    await page.getByPlaceholder('Title').fill('v2.0 Beta');
    await page.getByRole('button', { name: 'Create' }).click();

    // The list refetches on success, so the new milestone shows without a reload.
    await expect(page.getByRole('link', { name: 'v2.0 Beta' }).first()).toBeVisible();
  });
});
