import { lazy, Suspense, useEffect, useSyncExternalStore } from 'react';
import { createRoot } from 'react-dom/client';
import App from './App';
import './style.css';

const Blog = lazy(() => import('./Blog.jsx'));
const subscribe = callback => {
  window.addEventListener('hashchange', callback);
  return () => window.removeEventListener('hashchange', callback);
};

function Root() {
  const hash = useSyncExternalStore(subscribe, () => window.location.hash);
  const isBlog = hash.startsWith('#/blog/');
  useEffect(() => {
    if (!isBlog) document.getElementById(hash.slice(1))?.scrollIntoView({ behavior: 'instant' });
  }, [hash, isBlog]);
  return isBlog ? <Suspense fallback={<main className="article wrap" aria-busy="true"><p role="status">Loading article…</p><a href="#writing">Back to writing</a></main>}><Blog slug={hash.slice(7).split('/')[0]} anchor={hash.slice(7).split('/')[1]} /></Suspense> : <App />;
}

createRoot(document.getElementById('root')).render(<Root />);
