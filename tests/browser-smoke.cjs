const { chromium } = require(process.env.PLAYWRIGHT_MODULE || 'playwright');
const assert = require('node:assert/strict');
(async () => {
 const browser = await chromium.launch({headless:true});
 const page = await browser.newPage({viewport:{width:1440,height:1000}});
 const errors=[];
 page.on('pageerror', error => errors.push(error.message));
 await page.goto(process.env.PORTFOLIO_URL || 'http://127.0.0.1:5173', {waitUntil:'networkidle'});
 await page.locator('canvas').waitFor();
 for (const id of ['work','about','experience','contact']) { await page.locator('#'+id).scrollIntoViewIfNeeded(); await page.waitForTimeout(700); }
 await page.evaluate(() => window.scrollTo({top:0,behavior:'instant'}));
 await page.screenshot({path:require('node:path').join(require('node:os').tmpdir(), 'portfolio-desktop.png'),fullPage:true});
 await page.getByRole('button',{name:'Pause motion'}).click();
 assert.equal(await page.getByRole('button',{name:'Play motion'}).getAttribute('aria-pressed'),'true');
 await page.getByRole('button',{name:'02 Telemetry'}).click();
 await page.getByRole('heading',{name:'Make system behavior visible.'}).waitFor();
 await page.getByRole('heading',{name:'Research Assistant'}).waitFor();
 await page.getByRole('heading',{name:'Contract Software Engineer'}).waitFor();
 await page.getByRole('slider',{name:'Spring stiffness'}).press('ArrowRight');
 assert.equal(await page.getByRole('slider',{name:'Spring stiffness'}).inputValue(), '110');
 await page.getByRole('button',{name:'Run spring →'}).click();
 await page.getByRole('button',{name:'Run spring ←'}).waitFor();
 await page.locator('.case-notes summary').first().click();
 assert.equal(await page.locator('.case-notes').first().getAttribute('open'), '');
 await page.getByRole('img',{name:/Original Dioxus Motion showcase/}).waitFor();
 await page.getByRole('link',{name:'Explore my work'}).click();
 assert.equal(new URL(page.url()).hash,'#work');
 for (const width of [390, 320]) {
  await page.setViewportSize({width,height:844});
  await page.evaluate(() => window.scrollTo({top:0,behavior:'instant'}));
  await page.waitForTimeout(500);
  assert.ok(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth), `overflow at ${width}`);
  await page.screenshot({path:require('node:path').join(require('node:os').tmpdir(), `portfolio-mobile-${width}.png`),fullPage:true});
 }
 await page.emulateMedia({reducedMotion:'reduce'});
 await page.reload({waitUntil:'networkidle'});
 await page.getByRole('button',{name:'Motion reduced'}).waitFor();
 assert.equal(await page.getByRole('button',{name:'Motion reduced'}).isDisabled(),true);

 await page.locator('.journal-entry').filter({hasText:'Building dioxus-motion'}).click();
 await page.getByRole('heading', {name:'Building dioxus-motion', exact:true}).waitFor();
 assert.equal(await page.locator('.article-code pre').count(), 5);
 await page.reload({waitUntil:'networkidle'});
 await page.getByRole('heading', {name:'Introduction', exact:true}).waitFor();
 await page.locator('.article-contents summary').click();
 await page.getByRole('link',{name:'Introduction',exact:true}).click();
 assert.ok(new URL(page.url()).hash.endsWith('/section-0'));
 assert.equal(await page.locator('#section-0').innerText(),'Introduction');
 assert.ok(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth), 'article mobile overflow');
 await page.getByRole('link', {name:'← Back to all writing',exact:true}).first().click();
 assert.equal(new URL(page.url()).hash, '#writing');
 assert.deepEqual(errors,[]);
 await browser.close();
 console.log('Passed desktop, mobile 390/320px, system stages, spring controls, case notes, navigation, pause, reduced motion, and console checks.');
})().catch(error => { console.error(error); process.exit(1); });
