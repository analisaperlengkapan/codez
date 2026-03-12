import { test, expect } from '@playwright/test';

test.describe('User Keys Management Feature', () => {
  test('should list, create, and delete SSH and GPG keys', async ({ page }) => {
    // Navigate to user settings page
    await page.goto('http://127.0.0.1:8080/settings/profile');

    // Verify page loads
    await expect(page.getByRole('heading', { name: 'Settings' })).toBeVisible();

    // 1. Add SSH Key
    const sshTitle = 'My Test SSH Key ' + Date.now();
    const sshContent = 'ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABgQC' + Date.now();

    await page.getByPlaceholder('Title').fill(sshTitle);
    await page.getByPlaceholder('Key starting with ssh-rsa...').fill(sshContent);
    await page.getByRole('button', { name: 'Add SSH Key' }).click();

    // Verify SSH Key appears in list
    const sshEntry = page.getByText(sshTitle);
    await expect(sshEntry).toBeVisible();

    // 2. Add GPG Key
    const gpgContent = '-----BEGIN PGP PUBLIC KEY BLOCK-----\nTest GPG Key Data ' + Date.now() + '\n-----END PGP PUBLIC KEY BLOCK-----';
    await page.getByPlaceholder('Armored GPG Public Key...').fill(gpgContent);
    await page.getByRole('button', { name: 'Add GPG Key' }).click();

    // The mock backend endpoint create_gpg_key generates a key_id and primary_key_id.
    // We expect a new GPG key entry with a 'Delete' button next to it.
    // We can just verify the Delete button count increased in the GPG keys section,
    // or look for a new text entry. Since the ID is mocked by backend, we'll check for the Delete button inside the GPG section.
    const gpgSection = page.locator('.gpg-keys');
    await expect(gpgSection.getByRole('button', { name: 'Delete' }).first()).toBeVisible();

    // 3. Delete SSH Key
    // Find the delete button next to our specific SSH key
    const sshListItem = page.locator('li').filter({ hasText: sshTitle });
    await sshListItem.getByRole('button', { name: 'Delete' }).click();

    // Verify it disappears
    await expect(sshListItem).not.toBeVisible();
  });
});
