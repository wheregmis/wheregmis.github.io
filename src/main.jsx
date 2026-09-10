import { lazy, Suspense, useEffect, useRef, useSyncExternalStore } from 'react';
import { createRoot } from 'react-dom/client';
import App from './App';
import './style.css';

if (typeof window !== 'undefined' && 'scrollRestoration' in window.history) {
  window.history.scrollRestoration = 'manual';
}

const Blog = lazy(() => import('./Blog.jsx'));
const subscribe = callback => {
  window.addEventListener('hashchange', callback);
  return () => window.removeEventListener('hashchange', callback);
};

function Root() {
  const hash = useSyncExternalStore(subscribe, () => window.location.hash);
  const isBlog = hash.startsWith('#/blog/');
  const isInitialMount = useRef(true);

  useEffect(() => {
    if (isBlog) return;

    const isReload =
      typeof performance !== 'undefined' &&
      performance.getEntriesByType?.('navigation')?.[0]?.type === 'reload';

    if (isInitialMount.current) {
      isInitialMount.current = false;
      if (isReload || !hash || hash === '#home' || hash === '#work') {
        if (window.location.hash && !window.location.hash.startsWith('#/blog/')) {
          window.history.replaceState(null, '', window.location.pathname + window.location.search);
        }
        window.scrollTo({ top: 0, behavior: 'instant' });
        return;
      }
    }

    const targetId = hash.startsWith('#') ? hash.slice(1) : hash;
    const reduced = window.matchMedia('(prefers-reduced-motion: reduce)').matches;
    const behavior = reduced ? 'instant' : 'smooth';

    if (!targetId || targetId === 'home') {
      window.scrollTo({ top: 0, behavior });
      return;
    }

    const element = document.getElementById(targetId);
    if (element) {
      element.scrollIntoView({ behavior, block: 'start' });
    }
  }, [hash, isBlog]);
  return isBlog ? <Suspense fallback={<main className="article wrap" aria-busy="true"><p role="status">Loading article…</p><a href="#writing">Back to writing</a></main>}><Blog slug={hash.slice(7).split('/')[0]} anchor={hash.slice(7).split('/')[1]} /></Suspense> : <App />;
}

createRoot(document.getElementById('root')).render(<Root />);
