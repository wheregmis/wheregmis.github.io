const { chromium } = require(process.env.PLAYWRIGHT_MODULE || 'playwright');
const assert = require('node:assert/strict');

const baseURL = process.env.PORTFOLIO_URL || 'http://127.0.0.1:5173';

async function atSection(page, id) {
  await page.waitForFunction(section => {
    const target = document.getElementById(section);
    if (!target) return false;
    const actual = target.getBoundingClientRect().top;
    const margin = parseFloat(getComputedStyle(target).scrollMarginTop) || 0;
    const bottom = Math.abs(window.scrollY + innerHeight - document.documentElement.scrollHeight) < 3;
    return Math.abs(actual - margin) < 4 || (bottom && actual >= 0 && actual < innerHeight);
  }, id);
}

(async () => {
  const browser = await chromium.launch({ headless: true });
  try {
    const page = await browser.newPage({ viewport: { width: 1440, height: 1000 }, reducedMotion: 'reduce' });
    const errors = [];
    page.on('pageerror', error => errors.push(error.message));

    for (const id of ['work', 'about', 'writing']) {
      await page.goto(`${baseURL}/#${id}`, { waitUntil: 'networkidle' });
      assert.equal(new URL(page.url()).hash, `#${id}`);
      await atSection(page, id);
      await page.reload({ waitUntil: 'networkidle' });
      assert.equal(new URL(page.url()).hash, `#${id}`);
      await atSection(page, id);
    }

    await page.goto(baseURL, { waitUntil: 'networkidle' });
    const nav = page.getByRole('navigation', { name: 'Main navigation' });
    await nav.getByRole('link', { name: 'Work', exact: true }).click();
    await atSection(page, 'work');
    const historyLength = await page.evaluate(() => history.length);
    await page.evaluate(() => window.scrollTo({ top: 0, behavior: 'instant' }));
    await nav.getByRole('link', { name: 'Work', exact: true }).click();
    await atSection(page, 'work');
    assert.equal(await page.evaluate(() => history.length), historyLength, 'repeat link added a history entry');
    await nav.getByRole('link', { name: 'About', exact: true }).click();
    await atSection(page, 'about');
    await page.goBack();
    assert.equal(new URL(page.url()).hash, '#work');
    await atSection(page, 'work');
    await page.goForward();
    assert.equal(new URL(page.url()).hash, '#about');
    await atSection(page, 'about');

    // Start at an article directly, rather than only reaching it through the portfolio.
    await page.goto(`${baseURL}/#/blog/building-dioxus-motion`, { waitUntil: 'networkidle' });
    await page.getByRole('heading', { name: 'Building dioxus-motion', exact: true }).waitFor();
    await page.locator('.article-contents summary').click();
    await page.getByRole('link', { name: 'Introduction', exact: true }).click();
    await atSection(page, 'section-0');
    await page.goBack();
    assert.equal(new URL(page.url()).hash, '#/blog/building-dioxus-motion');
    await page.waitForFunction(() => window.scrollY < 3);
    await page.goForward();
    await atSection(page, 'section-0');
    await page.reload({ waitUntil: 'networkidle' });
    await atSection(page, 'section-0');
    await page.getByRole('link', { name: '← Back to all writing', exact: true }).first().click();
    assert.equal(new URL(page.url()).hash, '#writing');
    await atSection(page, 'writing');
    await page.goBack();
    await page.getByRole('heading', { name: 'Building dioxus-motion', exact: true }).waitFor();
    await atSection(page, 'section-0');
    await page.goForward();
    await atSection(page, 'writing');
    assert.equal(await page.title(), 'Sabin Regmi — Software Engineer');

    await page.goto(`${baseURL}/#/blog/not-a-post`, { waitUntil: 'networkidle' });
    await page.getByRole('heading', { name: 'Article not found.' }).waitFor();
    await page.getByRole('link', { name: 'Return to the writing section.' }).click();
    await atSection(page, 'writing');
    assert.deepEqual(errors, []);
    console.log('Passed section deep links/reloads, repeated links, Back/Forward, direct articles, contents history, article reloads, return to writing, and missing articles.');
  } finally {
    await browser.close();
  }
})().catch(error => { console.error(error); process.exitCode = 1; });
