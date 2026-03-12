const { chromium } = require('playwright');

(async () => {
  const browser = await chromium.launch();
  const page = await browser.newPage();

  await page.goto('http://localhost:8080/repos/admin/codeza');
  await page.waitForTimeout(2000);

  let html = await page.locator('.repo-actions').innerText();
  console.log("INITIAL:", html.replace(/\n/g, " "));

  await browser.close();
})();
