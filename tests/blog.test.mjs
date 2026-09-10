import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { createElement } from 'react';
import { renderToStaticMarkup } from 'react-dom/server';
import Markdown from 'react-markdown';
import remarkGfm from 'remark-gfm';
import { posts, markdownBody } from '../src/blog-data.js';

test('article renders real headings, lists, and escaped Rust code without frontmatter', () => {
 const source = readFileSync(new URL('../src/content/blog/building_dioxus_motion.md', import.meta.url), 'utf8');
 const html = renderToStaticMarkup(createElement(Markdown, { remarkPlugins: [remarkGfm], skipHtml: true }, markdownBody(source)));
 assert.match(html, /<h1>Introduction<\/h1>/);
 assert.match(html, /<pre><code class="language-rust">/);
 assert.match(html, /use_motion&lt;T: Animatable&gt;/);
 assert.match(html, /<ol>/);
 assert.doesNotMatch(html, /title =|\+\+\+/);
 assert.equal(markdownBody('+++\r\ntitle = "Test"\r\n+++\r\n# Body'), '# Body');
 assert.equal(markdownBody('# Body'), '# Body');
});

test('GFM formatting works and unsafe HTML and links cannot execute', () => {
 const html = renderToStaticMarkup(createElement(Markdown, { remarkPlugins: [remarkGfm], skipHtml: true }, '| A | B |\n|---|---|\n| 1 | 2 |\n\n~~old~~\n\n- [x] Done\n\n<script>alert(1)</script>\n\n[bad](javascript:alert)'));
 assert.match(html, /<table>/);
 assert.match(html, /<del>old<\/del>/);
 assert.match(html, /type="checkbox"/);
 assert.doesNotMatch(html, /<script|href="javascript:/);
});

 test('each published post has its own source, date, and destination', () => {
  assert.equal(new Set(posts.map(post => post.slug)).size, posts.length);
  for (const post of posts) {
   const source = readFileSync(new URL(`../src/content/blog/${post.file}`, import.meta.url), 'utf8');
   assert.ok(source.length > 100);
   assert.ok(Number.isFinite(Date.parse(post.isoDate)));
   assert.ok(post.readingMinutes > 0);
   assert.equal(new URL(post.projectUrl).protocol, 'https:');
   const html = renderToStaticMarkup(createElement(Markdown, {remarkPlugins:[remarkGfm], skipHtml:true}, markdownBody(source)));
   assert.match(html, /<h2>/);
   if (post.slug === 'building-threadlane') {
    assert.match(html, /Record intent before acting/);
    assert.match(html, /threadlane-workspace.jpg/);
    assert.equal(post.projectUrl, 'https://github.com/wheregmis/threadlane');
   }
  }
 });
