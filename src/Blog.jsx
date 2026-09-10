import { useEffect, useRef, useState } from 'react';
import Markdown from 'react-markdown';
import remarkGfm from 'remark-gfm';
const sources = import.meta.glob('./content/blog/*.md', { query: '?raw', import: 'default', eager: true });
import { posts, markdownBody } from './blog-data.js';

function CodeBlock({ children }) {
  const code = useRef(null);
  const [status, setStatus] = useState('Copy code');
  async function copy() {
    try {
      await navigator.clipboard.writeText(code.current.textContent);
      setStatus('Copied');
    } catch {
      setStatus('Select code to copy');
    }
  }
  return <div className="article-code"><div className="code-toolbar"><span>CODE</span><button onClick={copy} aria-live="polite">{status}</button></div><pre ref={code} tabIndex={0}>{children}</pre></div>;
}

const markdownComponents = { pre: CodeBlock, h1: ({ children }) => <h2>{children}</h2>, table: ({ children }) => <div className="article-table" tabIndex={0} role="region" aria-label="Scrollable table"><table>{children}</table></div> };

export default function Blog({ slug, anchor }) {
  const [contents, setContents] = useState([]);
  const post = posts.find(post => post.slug === slug);
  useEffect(() => {
    document.title = `${post ? post.title : 'Article not found'} — Sabin Regmi`;
    window.scrollTo({ top: 0, behavior: 'instant' });
    document.getElementById('article-title')?.focus({ preventScroll: true });
    return () => { document.title = 'Sabin Regmi — Software Engineer'; };
  }, [post]);
  useEffect(() => {
    const headings = [...document.querySelectorAll('.article-body h2')];
    setContents(headings.map((heading, index) => {
      heading.id = `section-${index}`;
      return { id: heading.id, title: heading.textContent };
    }));
  }, [slug]);
  useEffect(() => {
    if (anchor) document.getElementById(anchor)?.scrollIntoView({ behavior: 'instant' });
  }, [anchor, contents]);
  return <>
    <a className="skip-link" href="#article-body" onClick={event => { event.preventDefault(); document.getElementById('article-body')?.focus(); }}>Skip to article</a>
    <header className="header wrap"><a className="wordmark" href="#home">sabin regmi<span>®</span></a><a className="text-link" href="#writing">← Back to all writing</a></header>
    <main className="article wrap" id="main">
      {post ? <article>
        <header className="article-heading"><p className="eyebrow">NOTES FROM THE WORKBENCH / <time dateTime={post.isoDate}>{post.date}</time></p><h1 id="article-title" tabIndex={-1}>{post.title}</h1><p className="article-subtitle serif">{post.subtitle}</p><p className="article-byline">Written by Sabin Regmi · {post.readingMinutes} min read</p><div className="tags">{post.tags.map(tag => <span key={tag}>{tag}</span>)}</div></header>
        <details className="article-contents"><summary>In this field note <span aria-hidden="true">↓</span></summary><nav aria-label="Table of contents">{contents.map(item => <a key={item.id} href={`#/blog/${slug}/${item.id}`} aria-current={anchor === item.id ? 'location' : undefined}>{item.title}</a>)}</nav></details>
        <div className="article-body" id="article-body" tabIndex={-1}><Markdown remarkPlugins={[remarkGfm]} skipHtml components={markdownComponents}>{markdownBody(sources[`./content/blog/${post.file}`])}</Markdown></div>
        <footer className="article-footer"><a className="text-link" href="#writing">← Back to all writing</a><a className="text-link" href={post.projectUrl}>Explore {post.projectName} ↗</a></footer>
      </article> : <><h1 id="article-title" tabIndex={-1}>Article not found.</h1><p>This article isn’t available. <a href="#writing">Return to the writing section.</a></p></>}
    </main>
  </>;
}
