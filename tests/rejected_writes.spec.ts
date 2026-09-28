import { test, expect } from '@playwright/test';

/**
 * A rejected write must leave the user's input in place and surface a visible
 * error. These tests force the server to answer with a non-2xx status via
 * `page.route`, so they do not depend on any particular validation rule.
 */
test.describe('Rejected writes', () => {
  test('a rejected issue creation keeps the title and shows an error', async ({ page }) => {
    await page.goto('/repos/admin/codeza/issues');
    await expect(page.getByRole('heading', { name: 'Issues for admin/codeza' })).toBeVisible();

    await page.getByRole('button', { name: 'New Issue' }).click();

    const title = 'Rejected issue ' + Date.now();
    await page.getByPlaceholder('Title').fill(title);
    await page.getByPlaceholder('Body').fill('This write will be rejected.');

    // Fail only the create call, leaving the list request alone.
    await page.route('**/api/v1/repos/admin/codeza/issues', async (route) => {
      if (route.request().method() === 'POST') {
        await route.fulfill({ status: 422, contentType: 'application/json', body: '{}' });
      } else {
        await route.continue();
      }
    });

    await page.getByRole('button', { name: 'Create Issue' }).click();

    // The error is reported and the typed title survives.
    await expect(page.locator('.form-error')).toBeVisible();
    await expect(page.getByPlaceholder('Title')).toHaveValue(title);

    await page.unroute('**/api/v1/repos/admin/codeza/issues');
  });

  test('a rejected comment keeps the comment body and shows an error', async ({ page }) => {
    // Create an issue to comment on.
    const title = 'Comment target ' + Date.now();
    const created = await page.request.post('/api/v1/repos/admin/codeza/issues', {
      data: { title, body: 'Body' },
    });
    expect(created.ok()).toBeTruthy();
    const issue = await created.json();

    await page.goto(`/repos/admin/codeza/issues/${issue.number}`);
    await expect(page.getByRole('heading', { name: title })).toBeVisible();

    const comment = 'This comment will be rejected.';
    await page.getByPlaceholder('Leave a comment').fill(comment);

    await page.route('**/comments', async (route) => {
      if (route.request().method() === 'POST') {
        await route.fulfill({ status: 500, contentType: 'application/json', body: '{}' });
      } else {
        await route.continue();
      }
    });

    await page.getByRole('button', { name: 'Comment' }).click();

    await expect(page.locator('.form-error')).toBeVisible();
    await expect(page.getByPlaceholder('Leave a comment')).toHaveValue(comment);

    await page.unroute('**/comments');
  });

  test('a rejected pull request keeps the typed title and shows an error', async ({ page }) => {
    await page.goto('/repos/admin/codeza/compare');
    await expect(page.getByRole('heading', { name: 'Compare changes' })).toBeVisible();

    const title = 'Rejected PR ' + Date.now();
    await page.getByPlaceholder('Title').fill(title);
    await page.getByPlaceholder('Leave a comment').fill('Body that must survive.');

    await page.route('**/api/v1/repos/admin/codeza/pulls', async (route) => {
      if (route.request().method() === 'POST') {
        await route.fulfill({ status: 422, contentType: 'application/json', body: '{}' });
      } else {
        await route.continue();
      }
    });

    await page.getByRole('button', { name: 'Create Pull Request' }).click();

    const error = page.locator('.form-error[role="alert"]');
    await expect(error).toBeVisible();
    await expect(error).toHaveText('Could not save your changes. Please try again.');
    // The write is rejected, so the form must keep what the user typed.
    await expect(page.getByPlaceholder('Title')).toHaveValue(title);
    await expect(page.getByPlaceholder('Leave a comment')).toHaveValue(
      'Body that must survive.',
    );

    await page.unroute('**/api/v1/repos/admin/codeza/pulls');
  });

  test('a rejected wiki save keeps the page body and shows an error', async ({ page }) => {
    await page.goto('/repos/admin/codeza/wiki/pages/RejectedSave/edit');

    const body = 'Wiki body that must survive a rejected save ' + Date.now();
    await page.locator('textarea').fill(body);

    await page.route('**/api/v1/repos/admin/codeza/wiki/pages', async (route) => {
      if (route.request().method() === 'POST') {
        await route.fulfill({ status: 503, contentType: 'application/json', body: '{}' });
      } else {
        await route.continue();
      }
    });

    await page.getByRole('button', { name: 'Save Page' }).click();

    await expect(page.locator('.form-error')).toBeVisible();
    await expect(page.locator('textarea')).toHaveValue(body);

    await page.unroute('**/api/v1/repos/admin/codeza/wiki/pages');
  });
});
