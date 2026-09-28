import { test, expect } from '@playwright/test';

test.describe('Labels Feature', () => {
  test('should list and create labels', async ({ page }) => {
    await page.goto('/repos/admin/codeza/labels');

    await expect(page.getByRole('heading', { name: 'Labels' })).toBeVisible();

    const name = 'needs-triage-' + Date.now();
    await page.getByPlaceholder('Name').fill(name);
    await page.getByRole('button', { name: 'Create' }).click();

    // The list refetches on success, so the new label shows without a reload.
    await expect(page.getByText(name, { exact: true })).toBeVisible();
  });
});
