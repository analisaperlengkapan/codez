import { test, expect } from '@playwright/test';

test.describe('Milestones Feature', () => {
  test('should list and create milestones', async ({ page }) => {
    await page.goto('/repos/admin/codeza/milestones');

    await expect(page.getByRole('heading', { name: 'Milestones' })).toBeVisible();

    // A unique title, so a milestone left over from an earlier run on a reused
    // server cannot satisfy the assertion on its own.
    const title = 'Milestone ' + Date.now();
    await page.getByPlaceholder('Title').fill(title);
    await page.getByRole('button', { name: 'Create' }).click();

    // The list refetches on success, so the new milestone shows without a reload.
    await expect(page.getByRole('link', { name: title })).toBeVisible();
    await expect(page.getByPlaceholder('Title')).toHaveValue('');
  });
});
