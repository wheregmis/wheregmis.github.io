import test from 'node:test';
import assert from 'node:assert/strict';
import { navigateToSection, parseRoute, scrollToSection, subscribeToNavigation } from '../src/navigation.js';

function environment({ hash = '', reduced = false } = {}) {
  const browser = new EventTarget();
  const calls = { scroll: [], push: [], sections: [] };
  browser.location = { hash };
  browser.matchMedia = () => ({ matches: reduced });
  browser.scrollTo = value => calls.scroll.push(value);
  browser.history = { pushState: (...args) => { calls.push.push(args); browser.location.hash = args[2]; } };
  const document = { getElementById: id => ['work', 'about', 'writing'].includes(id)
    ? { scrollIntoView: value => calls.sections.push({ id, ...value }) }
    : null };
  return { browser, document, calls };
}

test('direct portfolio and article hashes retain their destination', () => {
  assert.deepEqual(parseRoute(''), { kind: 'portfolio', section: 'home' });
  assert.deepEqual(parseRoute('#home'), { kind: 'portfolio', section: 'home' });
  assert.deepEqual(parseRoute('#work'), { kind: 'portfolio', section: 'work' });
  assert.deepEqual(parseRoute('#writing'), { kind: 'portfolio', section: 'writing' });
  assert.deepEqual(parseRoute('#/blog/building-dioxus-motion/section-1'), {
    kind: 'article', slug: 'building-dioxus-motion', anchor: 'section-1'
  });
  assert.deepEqual(parseRoute('#/blog/building-threadlane'), { kind: 'article', slug: 'building-threadlane', anchor: '' });
});

test('encoded and malformed anchors cannot break routing', () => {
  assert.equal(parseRoute('#%77ork').section, 'work');
  assert.equal(parseRoute('#/blog/building%2Dthreadlane/section%2D1').anchor, 'section-1');
  assert.doesNotThrow(() => parseRoute('#/blog/%broken/%broken'));
  assert.equal(parseRoute('#%broken').section, '%broken');
});

test('navigation subscribers observe hash, history, and application changes and unsubscribe cleanly', () => {
  const { browser } = environment();
  let calls = 0;
  const unsubscribe = subscribeToNavigation(() => calls++, browser);
  for (const type of ['hashchange', 'popstate', 'portfolio:navigate']) browser.dispatchEvent(new Event(type));
  assert.equal(calls, 3);
  unsubscribe();
  for (const type of ['hashchange', 'popstate', 'portfolio:navigate']) browser.dispatchEvent(new Event(type));
  assert.equal(calls, 3);
});

test('section links create one history entry, notify React, and repeat-click without duplicates', () => {
  const env = environment();
  let updates = 0;
  subscribeToNavigation(() => updates++, env.browser);
  assert.equal(navigateToSection('work', env), true);
  assert.equal(env.browser.location.hash, '#work');
  assert.equal(env.calls.push.length, 1);
  assert.equal(updates, 1);
  assert.equal(navigateToSection('work', env), true);
  assert.equal(env.calls.push.length, 1);
  assert.deepEqual(env.calls.sections, [{ id: 'work', behavior: 'smooth', block: 'start' }]);
  assert.equal(navigateToSection('missing', env), false);
  assert.equal(env.calls.push.length, 1);
});

test('initial/reduced-motion navigation is immediate and home returns to the top', () => {
  const env = environment();
  scrollToSection('work', { ...env, instant: true });
  assert.equal(env.calls.sections[0].behavior, 'instant');
  scrollToSection('home', env);
  assert.deepEqual(env.calls.scroll[0], { top: 0, behavior: 'smooth' });
  const reduced = environment({ reduced: true, hash: '#writing' });
  navigateToSection('writing', reduced);
  assert.equal(reduced.calls.sections[0].behavior, 'instant');
  assert.equal(scrollToSection('missing', env), false);
});
