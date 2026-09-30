const navigationEvent = 'portfolio:navigate';

const decodeSegment = value => {
  try {
    return decodeURIComponent(value);
  } catch {
    // A malformed shared URL should never prevent the page from rendering.
    return value;
  }
};

export function parseRoute(hash = '') {
  if (hash.startsWith('#/blog/')) {
    const [slug = '', anchor = ''] = hash.slice(7).split('/');
    return { kind: 'article', slug: decodeSegment(slug), anchor: decodeSegment(anchor) };
  }
  return { kind: 'portfolio', section: decodeSegment(hash.replace(/^#/, '')) || 'home' };
}

export function subscribeToNavigation(callback, browser = window) {
  // pushState does not emit hashchange; browser Back/Forward also emits popstate.
  const events = ['hashchange', 'popstate', navigationEvent];
  events.forEach(event => browser.addEventListener(event, callback));
  return () => events.forEach(event => browser.removeEventListener(event, callback));
}

export function scrollToSection(id, { instant = false, browser = window, document: page = document } = {}) {
  const reduced = browser.matchMedia?.('(prefers-reduced-motion: reduce)').matches;
  const behavior = instant || reduced ? 'instant' : 'smooth';
  if (!id || id === 'home') {
    browser.scrollTo({ top: 0, behavior });
    return true;
  }
  const target = page.getElementById(id);
  if (!target) return false;
  target.scrollIntoView({ behavior, block: 'start' });
  return true;
}

export function navigateToSection(id, { browser = window, document: page = document } = {}) {
  if (id !== 'home' && !page.getElementById(id)) return false;
  const hash = `#${encodeURIComponent(id)}`;
  if (browser.location.hash === hash) {
    // Re-selecting the active section should still bring it back into view.
    return scrollToSection(id, { browser, document: page });
  }
  browser.history.pushState(null, '', hash);
  browser.dispatchEvent(new Event(navigationEvent));
  return true;
}
