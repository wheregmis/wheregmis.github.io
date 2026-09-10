import { lazy, Suspense, useEffect, useState } from 'react';
import { m, LazyMotion, domAnimation, MotionConfig, useReducedMotion } from 'framer-motion';
import { projects, experience, systemStages, caseStudies } from './data';
import { posts } from './blog-data.js';
import Playground from './Playground';

const Sculpture = lazy(() => import('./Sculpture'));
const Arrow = () => <span aria-hidden="true">↗</span>;
const sections = [['work', 'Work'], ['about', 'About'], ['experience', 'Journey'], ['writing', 'Writing']];

export default function App() {
  const [stage, setStage] = useState(0);
  const [paused, setPaused] = useState(false);
  const [active, setActive] = useState('');
  const reduced = useReducedMotion();
  useEffect(() => {
    const observer = new IntersectionObserver(entries => {
      entries.forEach(entry => { if (entry.isIntersecting) setActive(entry.target.id); });
    }, { rootMargin: '-15% 0px -65% 0px' });
    ['home', ...sections.map(([id]) => id), 'contact'].forEach(id => observer.observe(document.getElementById(id)));
    return () => observer.disconnect();
  }, []);
  const reveal = { initial: { y: reduced ? 0 : 16 }, whileInView: { y: 0 }, viewport: { once: true }, transition: { duration: 0.4 } };
  return <LazyMotion features={domAnimation} strict><MotionConfig reducedMotion="user">
    <a className="skip-link" href="#main">Skip to content</a>
    <div className="nav-shell"><header className="header wrap">
      <a className="wordmark" href="#home" aria-label="Sabin Regmi home">sabin regmi<span className="brand-dot">●</span></a>
      <nav aria-label="Main navigation">{sections.map(([id, name]) => <a key={id} href={`#${id}`} aria-current={active === id ? 'location' : undefined}>{name}</a>)}</nav>
      <a className="contact-link" href="#contact">Let’s talk <Arrow /></a>
    </header></div>
    <main id="main">
      <section className="hero wrap" id="home">
        <div className="edition"><span>THE ENGINEERING FIELD JOURNAL</span><span>RUST / ENERGY / INTERFACES</span></div>
        <m.div className="hero-copy" {...reveal}>
          <p className="eyebrow">SABIN REGMI — SOFTWARE ENGINEER</p>
          <h1>Software for<br /><span className="serif">the real world.</span><span className="orange">*</span></h1>
          <p className="intro">Energy systems that stay reliable.<br />Developer tools that feel intuitive.<br /><strong>I build where the two meet.</strong></p>
          <div className="hero-actions"><a className="button dark" href="#work">Explore my work <span aria-hidden="true">↓</span></a><a className="text-link" href="https://github.com/wheregmis" target="_blank" rel="noreferrer">GitHub <Arrow /></a></div>
          <p className="hero-footnote"><span className="status-dot" /> From battery controllers to expressive interfaces.</p>
        </m.div>
        <div className="system-panel">
          <div className="panel-label"><span>FIG. 01 / FROM SIGNAL TO SCREEN</span><span className="system-badge">CONCEPT MODEL</span></div>
          <div className="system-scene"><Suspense fallback={<div className="scene-fallback">▥ → ▦ → ▤</div>}><Sculpture paused={paused} stage={stage} /></Suspense></div>
          <div className="system-stages" aria-label="Explore the system">{systemStages.map((item, index) => <button key={item.name} aria-pressed={stage === index} onClick={() => setStage(index)}><small>0{index + 1}</small>{item.name}<span aria-hidden="true">{index < 2 ? '→' : '↗'}</span></button>)}</div>
          <div className="system-description" aria-live="polite"><p className="eyebrow">{systemStages[stage].label}</p><h3>{systemStages[stage].title}</h3><p>{systemStages[stage].description}</p><span>{systemStages[stage].detail}</span></div>
          <div className="panel-footer"><span>Select a stage to explore</span><button onClick={() => setPaused(!paused)} aria-pressed={paused} disabled={!!reduced}>{reduced ? 'Motion reduced' : paused ? 'Play motion ↗' : 'Pause motion Ⅱ'}</button></div>
        </div>
      </section>
      <div className="practice-strip wrap"><span>01 — RELIABLE SYSTEMS</span><span>02 — EXPRESSIVE INTERFACES</span><span>03 — OPEN-SOURCE TOOLS</span></div>
      <section className="section wrap" id="work">
        <div className="section-heading"><div><p className="eyebrow">01 / SELECTED WORK</p><h2>Less telling.<br /><span className="serif">More showing.</span></h2></div><p className="section-note">Developer tools. Carefully considered.<br />Explore the interaction, then the thinking.</p></div>
        {projects.map((project, index) => <m.article className="project-study" key={project.title} {...reveal}>
          <div className="project-overview"><p className="eyebrow">0{index + 1} / {project.category}</p><h3>{project.title}</h3><p>{project.description}</p><div className="tags">{project.tags.map(tag => <span key={tag}>{tag}</span>)}</div><a className="text-link" href={project.href} target="_blank" rel="noreferrer">{project.type === 'motion' ? 'Explore the library' : project.type === 'threadlane' ? 'Explore Threadlane on GitHub' : 'Open the live converter'} <Arrow /></a></div>
          {project.type === 'motion' ? <Playground /> : project.type === 'threadlane' ? <div className="threadlane-preview"><div className="panel-label"><span>NATIVE WORKSPACE / RUST + GPUI</span><span>03</span></div><a href={project.href} target="_blank" rel="noreferrer" aria-label="Explore Threadlane workspace on GitHub"><img src="/images/threadlane-workspace.jpg" alt="Threadlane native desktop workspace with project sessions, agent conversation, and command completion" loading="lazy" /></a><div className="threadlane-features"><span>Branching conversations</span><span>Persistent terminals</span><span>Durable execution</span></div><a className="text-link" href={project.href} target="_blank" rel="noreferrer">View source & setup <Arrow /></a></div> : <div className="converter-preview"><div className="panel-label"><span>WORKFLOW / HTML TO RSX</span><span>02</span></div><div className="conversion-step"><span>INPUT · HTML</span><pre>{'<div class="hello">\n  Hello, world.\n</div>'}</pre></div><div className="conversion-arrow" aria-hidden="true">↓</div><div className="conversion-step"><span>OUTPUT · RSX</span><pre>{'div { class: "hello",\n  "Hello, world."\n}'}</pre></div><a className="button dark" href={project.href} target="_blank" rel="noreferrer">Try your own HTML <Arrow /></a></div>}
          <details className="case-notes"><summary><span>Inside the project <small>Problem, approach & result</small></span><span className="detail-plus" aria-hidden="true">+</span></summary><div className="case-body"><div className="case-narrative">{['problem','approach','outcome'].map(key => <section key={key}><p className="eyebrow">{key === 'outcome' ? 'THE RESULT' : key.toUpperCase()}</p><p>{caseStudies[index][key]}</p></section>)}</div><figure><img src={caseStudies[index].image} alt={caseStudies[index].caption} loading="lazy" width="1100" height="655" /><figcaption>{caseStudies[index].caption}</figcaption></figure>{project.type === 'threadlane' && <a className="text-link" href="#/blog/building-threadlane">Read the Threadlane story <Arrow /></a>}{project.type === 'motion' && <a className="text-link" href="#/blog/building-dioxus-motion">Read the development story <Arrow /></a>}</div></details>
        </m.article>)}
      </section>
      <section className="about-band" id="about"><div className="wrap about-grid"><div><p className="eyebrow">02 / THE PERSON BEHIND THE CODE</p><h2>Curiosity is<br /><span className="serif">part of the process.</span></h2><div className="avatar-line"><img src="/images/avatar.png" alt="AI-generated illustration of Sabin Regmi" width="55" height="55" loading="lazy" /><div>Sabin Regmi<small>Software engineer · Open-source builder</small></div></div></div><div className="about-copy"><p>I like working where software meets the physical world.</p><p>My work spans battery management, energy monitoring, and tools that make building interfaces more enjoyable. Rust is a recurring thread: reliability at the system level, room to experiment at the interface.</p><p>Outside my day job, I build with Rust and Dioxus, explore motion, and share what I learn.</p><a className="text-link" href="https://linkedin.com/in/wheregmis" target="_blank" rel="noreferrer">Meet me on LinkedIn <Arrow /></a></div></div></section>
      <section className="section wrap" id="experience"><div className="section-heading"><div><p className="eyebrow">03 / THE JOURNEY</p><h2>A few chapters.<br /><span className="serif">One curious mind.</span></h2></div><p className="section-note">From moving data<br />to making energy visible.</p></div><ol className="timeline">{experience.map((job, index) => <li key={job.company}><div className="timeline-date"><span className="timeline-number">0{index + 1}</span><p>{job.date}</p></div><div><h3>{job.role}</h3><p className="company-name">{job.company}</p><p className="job-description">{job.description}</p><div className="tags">{job.tags.map(tag => <span key={tag}>{tag}</span>)}</div></div></li>)}</ol></section>
      <section className="writing wrap" id="writing"><div className="section-heading"><div><p className="eyebrow">04 / NOTES FROM THE WORKBENCH</p><h2>Built something.<br /><span className="serif">Learned something.</span></h2></div><span className="journal-count">{String(posts.length).padStart(2, '0')} ENTRIES / FIELD NOTES</span></div>{posts.map((post, index) => <a className="journal-entry" key={post.slug} href={`#/blog/${post.slug}`}><div className="journal-art" aria-hidden="true"><span>FIELD NOTE / {String(posts.length - index).padStart(3, '0')}</span><strong>{post.projectName === 'Threadlane' ? '⌘' : 'ƒ'}<span>({post.projectName === 'Threadlane' ? 'lanes' : 'motion'})</span></strong><small>{post.artLabel}</small></div><div className="journal-copy"><p className="eyebrow">{post.tags.join(' / ')} · {post.readingMinutes} MIN READ</p><h3>{post.title}</h3><p>{post.description}</p><div><time dateTime={post.isoDate}>{post.date}</time><span>Read the story <Arrow /></span></div></div></a>)}</section>
      <footer id="contact" className="contact"><div className="wrap"><p className="eyebrow">05 / THE NEXT CONVERSATION</p><h2>Something worth<br /><span className="serif">building together?</span></h2><div className="contact-actions"><a className="button light" href="https://linkedin.com/in/wheregmis" target="_blank" rel="noreferrer">Let’s connect on LinkedIn <Arrow /></a><p>Energy systems, open-source tools,<br />or an interesting problem. Say hello.</p></div><div className="footer-bottom"><span>© {new Date().getFullYear()} Sabin Regmi</span><div><a href="https://github.com/wheregmis" target="_blank" rel="noreferrer">GitHub <Arrow /></a><a href="https://twitter.com/wheregmis" target="_blank" rel="noreferrer">X / Twitter <Arrow /></a></div><a href="#home">Back to top ↑</a></div></div></footer>
    </main>
  </MotionConfig></LazyMotion>;
}
