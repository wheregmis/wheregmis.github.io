import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { getResolvedTheme, applyTheme, toggleTheme } from '../src/theme.js';

test('theme initialization and resolution respects preference order', () => {
  // Mock global DOM environment
  const mockStorage = {};
  const mockAttrs = {};
  const metaElements = {
    'theme-color': { content: '#f5f3ed', setAttribute(k, v) { this.content = v; } },
    'color-scheme': { content: 'light dark', setAttribute(k, v) { this.content = v; } }
  };

  let mediaMatches = false;
  let dispatchedEvents = [];

  globalThis.window = {
    matchMedia: (query) => ({
      matches: mediaMatches,
      addEventListener: () => {},
      removeEventListener: () => {}
    }),
    dispatchEvent: (event) => {
      dispatchedEvents.push(event);
    }
  };

  globalThis.CustomEvent = class CustomEvent {
    constructor(type, init) {
      this.type = type;
      this.detail = init?.detail;
    }
  };

  globalThis.document = {
    documentElement: {
      getAttribute: (k) => mockAttrs[k] || null,
      setAttribute: (k, v) => { mockAttrs[k] = v; }
    },
    querySelector: (selector) => {
      if (selector.includes('theme-color')) return metaElements['theme-color'];
      if (selector.includes('color-scheme')) return metaElements['color-scheme'];
      return null;
    }
  };

  globalThis.localStorage = {
    getItem: (k) => mockStorage[k] || null,
    setItem: (k, v) => { mockStorage[k] = v; }
  };

  // 1. Initial state without stored theme or media query match -> 'light'
  assert.equal(getResolvedTheme(), 'light');

  // 2. OS preference dark when nothing is stored or set
  mediaMatches = true;
  assert.equal(getResolvedTheme(), 'dark');

  // 3. Explicit stored preference overrides media query
  mediaMatches = false;
  mockStorage['theme'] = 'dark';
  assert.equal(getResolvedTheme(), 'dark');

  mockStorage['theme'] = 'light';
  assert.equal(getResolvedTheme(), 'light');

  // 4. Test applyTheme('dark')
  applyTheme('dark');
  assert.equal(mockAttrs['data-theme'], 'dark');
  assert.equal(mockStorage['theme'], 'dark');
  assert.equal(metaElements['theme-color'].content, '#141815');
  assert.equal(metaElements['color-scheme'].content, 'dark');
  assert.equal(dispatchedEvents.at(-1)?.detail?.theme, 'dark');

  // 5. Test applyTheme('light')
  applyTheme('light');
  assert.equal(mockAttrs['data-theme'], 'light');
  assert.equal(mockStorage['theme'], 'light');
  assert.equal(metaElements['theme-color'].content, '#f5f3ed');
  assert.equal(metaElements['color-scheme'].content, 'light');
  assert.equal(dispatchedEvents.at(-1)?.detail?.theme, 'light');

  // 6. Test toggleTheme()
  const nextTheme = toggleTheme();
  assert.equal(nextTheme, 'dark');
  assert.equal(getResolvedTheme(), 'dark');

  const secondToggle = toggleTheme();
  assert.equal(secondToggle, 'light');
  assert.equal(getResolvedTheme(), 'light');
});

test('index.html includes color-scheme meta and synchronous bootstrap script', () => {
  const html = readFileSync(new URL('../index.html', import.meta.url), 'utf-8');
  assert.ok(html.includes('<meta name="color-scheme" content="light dark"'), 'Includes color-scheme meta');
  assert.ok(html.includes('data-theme'), 'Bootstrap script sets data-theme attribute');
  assert.ok(html.includes('prefers-color-scheme: dark'), 'Bootstrap script checks prefers-color-scheme');
});

test('style.css defines robust light and dark theme design tokens', () => {
  const css = readFileSync(new URL('../src/style.css', import.meta.url), 'utf-8');
  assert.ok(css.includes('color-scheme: light dark'), 'style.css declares color-scheme: light dark on root');
  assert.ok(css.includes('@media (prefers-color-scheme: dark)'), 'style.css has prefers-color-scheme media query');
  assert.ok(css.includes(':root[data-theme="dark"]'), 'style.css has :root[data-theme="dark"]');
  assert.ok(css.includes(':root[data-theme="light"]'), 'style.css has :root[data-theme="light"]');
  assert.ok(css.includes('.theme-toggle-btn'), 'style.css has .theme-toggle-btn styling');
  assert.ok(css.includes('.header-actions'), 'style.css has .header-actions styling');
});

test('ThemeToggle is integrated into both portfolio and blog headers', () => {
  const appContent = readFileSync(new URL('../src/App.jsx', import.meta.url), 'utf-8');
  const blogContent = readFileSync(new URL('../src/Blog.jsx', import.meta.url), 'utf-8');

  assert.ok(appContent.includes('<ThemeToggle'), 'App.jsx includes ThemeToggle component');
  assert.ok(blogContent.includes('<ThemeToggle'), 'Blog.jsx includes ThemeToggle component');
});
