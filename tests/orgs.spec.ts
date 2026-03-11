import { test, expect } from '@playwright/test';

test.describe('Organizations Feature', () => {
  test('should create and list org details', async ({ page }) => {
    await page.goto('http://127.0.0.1:8080/org/create');

    await expect(page.getByRole('heading', { name: 'New Organization' })).toBeVisible();

    await page.getByPlaceholder('Organization Name').fill('test-org-playwright');
    await page.getByPlaceholder('Description').fill('Created by test');
    await page.getByRole('button', { name: 'Create Organization' }).click();

    // After creation, navigate to org profile manually
    await page.goto('http://127.0.0.1:8080/orgs/test-org-playwright');
    await expect(page.getByRole('heading', { name: 'test-org-playwright' })).toBeVisible();
  });
});
