import { lazy, Suspense, useEffect, useRef, useSyncExternalStore } from 'react';
import { createRoot } from 'react-dom/client';
import App from './App';
import { parseRoute, scrollToSection, subscribeToNavigation } from './navigation.js';
import './style.css';

if (typeof window !== 'undefined' && 'scrollRestoration' in window.history) {
  window.history.scrollRestoration = 'manual';
}

const Blog = lazy(() => import('./Blog.jsx'));
const subscribe = callback => subscribeToNavigation(callback);
const getHash = () => window.location.hash;

function Root() {
  const hash = useSyncExternalStore(subscribe, getHash);
  const route = parseRoute(hash);
  const previousHash = useRef(null);

  useEffect(() => {
    const initial = previousHash.current === null;
    previousHash.current = hash;
    const destination = parseRoute(hash);
    if (destination.kind === 'article') return;

    // Wait until the portfolio has mounted, including when returning from an article.
    // Initial links and reloads land immediately without rewriting the shared URL.
    const frame = window.requestAnimationFrame(() => {
      scrollToSection(destination.section, { instant: initial });
    });
    return () => window.cancelAnimationFrame(frame);
  }, [hash]);
  return route.kind === 'article' ? <Suspense fallback={<main className="article wrap" aria-busy="true"><p role="status">Loading article…</p><a href="#writing">Back to writing</a></main>}><Blog slug={route.slug} anchor={route.anchor} /></Suspense> : <App />;
}

createRoot(document.getElementById('root')).render(<Root />);
