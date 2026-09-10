import { lazy, Suspense, useEffect, useState } from 'react';
import { m, LazyMotion, domAnimation, MotionConfig, useReducedMotion, useScroll, AnimatePresence } from 'framer-motion';
import { projects, experience, systemStages, caseStudies } from './data';
import { posts } from './blog-data.js';
import Playground from './Playground';
import { FlipWords, CardSpotlight, TracingBeam, TiltCard } from './AceternityUI';

const Sculpture = lazy(() => import('./Sculpture'));
const Arrow = () => <span aria-hidden="true">↗</span>;
const sections = [['work', 'Work'], ['about', 'About'], ['experience', 'Journey'], ['writing', 'Writing']];

export default function App() {
  const [stage, setStage] = useState(0);
  const [paused, setPaused] = useState(false);
  const [active, setActive] = useState('');
  const [showScrollTop, setShowScrollTop] = useState(false);
  const reduced = useReducedMotion();
  const { scrollYProgress } = useScroll();

  useEffect(() => {
    const observer = new IntersectionObserver(entries => {
      entries.forEach(entry => { if (entry.isIntersecting) setActive(entry.target.id); });
    }, { rootMargin: '-20% 0px -55% 0px' });
    ['home', ...sections.map(([id]) => id), 'contact'].forEach(id => {
      const el = document.getElementById(id);
      if (el) observer.observe(el);
    });
    return () => observer.disconnect();
  }, []);

  useEffect(() => {
    const handleScroll = () => {
      setShowScrollTop(window.scrollY > 350);
    };
    window.addEventListener('scroll', handleScroll, { passive: true });
    handleScroll();
    return () => window.removeEventListener('scroll', handleScroll);
  }, []);

  const handleNavClick = (e, id) => {
    e.preventDefault();
    const reducedMotion = window.matchMedia('(prefers-reduced-motion: reduce)').matches;
    const behavior = reducedMotion ? 'instant' : 'smooth';
    if (id === 'home') {
      window.scrollTo({ top: 0, behavior });
      if (window.location.hash !== '#home') {
        window.history.pushState(null, '', '#home');
      }
      setActive('home');
    } else {
      const el = document.getElementById(id);
      if (el) {
        el.scrollIntoView({ behavior, block: 'start' });
        if (window.location.hash !== `#${id}`) {
          window.history.pushState(null, '', `#${id}`);
        }
        setActive(id);
      }
    }
  };

  const smoothEase = [0.22, 1, 0.36, 1];
  const reveal = {
    initial: { opacity: reduced ? 1 : 0, y: reduced ? 0 : 26 },
    whileInView: { opacity: 1, y: 0 },
    viewport: { once: true, margin: '-40px' },
    transition: { duration: 0.6, ease: smoothEase }
  };
  const revealStagger = (delay = 0) => ({
    initial: { opacity: reduced ? 1 : 0, y: reduced ? 0 : 22 },
    whileInView: { opacity: 1, y: 0 },
    viewport: { once: true, margin: '-40px' },
    transition: { duration: 0.55, delay: reduced ? 0 : delay, ease: smoothEase }
  });

  return <LazyMotion features={domAnimation} strict><MotionConfig reducedMotion="user">
    <a className="skip-link" href="#main">Skip to content</a>
    <div className="nav-shell">
      <m.div className="scroll-progress-bar" style={{ scaleX: scrollYProgress }} aria-hidden="true" />
      <header className="header wrap">
        <a className="wordmark" href="#home" onClick={e => handleNavClick(e, 'home')} aria-label="Sabin Regmi home">
          <span className="sr-only">sabin regmi</span>
          <span aria-hidden="true">sab<span className="starred-i">ı<span className="star-tittle">*</span></span>n regm<span className="starred-i">ı<span className="star-tittle">*</span></span></span>
        </a>
        <nav aria-label="Main navigation">
          {sections.map(([id, name]) => (
            <a
              key={id}
              href={`#${id}`}
              onClick={e => handleNavClick(e, id)}
              aria-current={active === id ? 'location' : undefined}
              className="nav-link-item"
            >
              {active === id && (
                <m.span
                  layoutId="activeNavIndicator"
                  className="nav-active-pill"
                  transition={{ type: 'spring', stiffness: 380, damping: 32 }}
                  aria-hidden="true"
                />
              )}
              <span className="nav-link-text">{name}</span>
            </a>
          ))}
        </nav>
        <a className="contact-link" href="#contact" onClick={e => handleNavClick(e, 'contact')}>Let’s talk <Arrow /></a>
      </header>
    </div>
    <main id="main">
      <section className="hero wrap" id="home">
        <div className="hero-matrix-bg" aria-hidden="true" />
        <div className="hero-ambient-glow" aria-hidden="true" />
        <div className="edition"><span>THE ENGINEERING FIELD JOURNAL</span><span>RUST / ENERGY / INTERFACES</span></div>
        <m.div className="hero-copy" {...reveal}>
          <div className="hero-status-pill"><span className="pulse-dot" /><span>SYSTEMS &amp; INTERFACE ARCHITECTURE</span></div>
          <p className="eyebrow">SABIN REGMI — SOFTWARE ENGINEER</p>
          <h1>Software for<br /><span className="serif"><FlipWords words={['the real world.', 'energy systems.', 'developer tools.', 'expressive motion.']} /></span><span className="orange">*</span></h1>
          <p className="intro">Energy systems that stay reliable.<br />Developer tools that feel intuitive.<br /><strong>I build where the two meet.</strong></p>
          <div className="hero-actions"><a className="button dark shimmer" href="#work" onClick={e => handleNavClick(e, 'work')}>Explore my work <span aria-hidden="true">↓</span></a><a className="text-link" href="https://github.com/wheregmis" target="_blank" rel="noreferrer">GitHub <Arrow /></a></div>
          <p className="hero-footnote"><span className="status-dot" /> From battery controllers to expressive interfaces.</p>
        </m.div>
        <m.div className="system-panel" {...revealStagger(0.12)}>
          <div className="panel-label"><span>FIG. 01 / FROM SIGNAL TO SCREEN</span><span className="system-badge"><span className="radar-pulse" /> LIVE SIMULATION</span></div>
          <div className="system-scene"><Suspense fallback={<div className="scene-fallback">▥ → ▦ → ▤</div>}><Sculpture paused={paused} stage={stage} /></Suspense></div>
          <div className="system-stages" aria-label="Explore the system">{systemStages.map((item, index) => <button key={item.name} aria-pressed={stage === index} onClick={() => setStage(index)}><small>0{index + 1}</small>{item.name}<span aria-hidden="true">{index < 2 ? '→' : '↗'}</span></button>)}</div>
          <div className="system-description" aria-live="polite">
            <m.div
              key={stage}
              initial={{ opacity: reduced ? 1 : 0, y: reduced ? 0 : 8 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.3, ease: smoothEase }}
            >
              <p className="eyebrow">{systemStages[stage].label}</p>
              <h3>{systemStages[stage].title}</h3>
              <p>{systemStages[stage].description}</p>
              <span>{systemStages[stage].detail}</span>
            </m.div>
          </div>
          <div className="panel-footer"><span>Select a stage to explore</span><button onClick={() => setPaused(!paused)} aria-pressed={paused} disabled={!!reduced}>{reduced ? 'Motion reduced' : paused ? 'Play motion ↗' : 'Pause motion Ⅱ'}</button></div>
        </m.div>
      </section>
      <m.div className="practice-strip wrap" {...reveal}><span>01 — RELIABLE SYSTEMS</span><span>02 — EXPRESSIVE INTERFACES</span><span>03 — OPEN-SOURCE TOOLS</span></m.div>
      <section className="section wrap" id="work">
        <m.div className="section-heading" {...reveal}><div><p className="eyebrow">01 / SELECTED WORK</p><h2>Less telling.<br /><span className="serif">More showing.</span></h2></div><p className="section-note">Developer tools. Carefully considered.<br />Explore the interaction, then the thinking.</p></m.div>
        {projects.map((project, index) => <m.article className="project-study" key={project.title} {...revealStagger(index * 0.1)}>
          <div className="project-overview"><p className="eyebrow">0{index + 1} / {project.category}</p><h3>{project.title}</h3><p>{project.description}</p><div className="tags">{project.tags.map(tag => <span key={tag}>{tag}</span>)}</div><a className="text-link" href={project.href} target="_blank" rel="noreferrer">{project.type === 'motion' ? 'Explore the library' : project.type === 'threadlane' ? 'Explore Threadlane on GitHub' : 'Open the live converter'} <Arrow /></a></div>
          {project.type === 'motion' ? <CardSpotlight color="rgba(206, 73, 39, 0.1)"><Playground /></CardSpotlight> : project.type === 'threadlane' ? <TiltCard><CardSpotlight color="rgba(206, 73, 39, 0.1)"><div className="threadlane-preview"><div className="panel-label"><span>NATIVE WORKSPACE / RUST + GPUI</span><span>03</span></div><a href={project.href} target="_blank" rel="noreferrer" aria-label="Explore Threadlane workspace on GitHub"><img src="/images/threadlane-workspace.jpg" alt="Threadlane native desktop workspace with project sessions, agent conversation, and command completion" loading="lazy" /></a><div className="threadlane-features"><span>Branching conversations</span><span>Persistent terminals</span><span>Durable execution</span></div><a className="text-link" href={project.href} target="_blank" rel="noreferrer">View source & setup <Arrow /></a></div></CardSpotlight></TiltCard> : <CardSpotlight color="rgba(206, 73, 39, 0.08)"><div className="converter-preview"><div className="panel-label"><span>WORKFLOW / HTML TO RSX</span><span>02</span></div><div className="conversion-step"><span>INPUT · HTML</span><pre>{'<div class="hello">\n  Hello, world.\n</div>'}</pre></div><div className="conversion-arrow" aria-hidden="true">↓</div><div className="conversion-step"><span>OUTPUT · RSX</span><pre>{'div { class: "hello",\n  "Hello, world."\n}'}</pre></div><a className="button dark" href={project.href} target="_blank" rel="noreferrer">Try your own HTML <Arrow /></a></div></CardSpotlight>}
          <details className="case-notes"><summary><span>Inside the project <small>Problem, approach & result</small></span><span className="detail-plus" aria-hidden="true">+</span></summary><div className="case-body"><div className="case-narrative">{['problem','approach','outcome'].map(key => <section key={key}><p className="eyebrow">{key === 'outcome' ? 'THE RESULT' : key.toUpperCase()}</p><p>{caseStudies[index][key]}</p></section>)}</div><figure><img src={caseStudies[index].image} alt={caseStudies[index].caption} loading="lazy" width="1100" height="655" /><figcaption>{caseStudies[index].caption}</figcaption></figure>{project.type === 'threadlane' && <a className="text-link" href="#/blog/building-threadlane">Read the Threadlane story <Arrow /></a>}{project.type === 'motion' && <a className="text-link" href="#/blog/building-dioxus-motion">Read the development story <Arrow /></a>}</div></details>
        </m.article>)}
      </section>
      <section className="about-band" id="about">
        <m.div className="wrap about-grid" {...reveal}>
          <div>
            <p className="eyebrow">02 / THE PERSON BEHIND THE CODE</p>
            <h2>Curiosity is<br /><span className="serif">part of the process.</span></h2>
            <div className="avatar-line">
              <div className="avatar-wrapper">
                <img src="/images/avatar.png" alt="AI-generated illustration of Sabin Regmi" width="55" height="55" loading="lazy" />
                <span className="avatar-glow" aria-hidden="true" />
              </div>
              <div>Sabin Regmi<small>Software engineer · Open-source builder</small></div>
            </div>
          </div>
          <div className="about-copy">
            <p>I like working where software meets the physical world.</p>
            <p>My work spans battery management, energy monitoring, and tools that make building interfaces more enjoyable. Rust is a recurring thread: reliability at the system level, room to experiment at the interface.</p>
            <p>Outside my day job, I build with Rust and Dioxus, explore motion, and share what I learn.</p>
            <a className="text-link" href="https://linkedin.com/in/wheregmis" target="_blank" rel="noreferrer">Meet me on LinkedIn <Arrow /></a>
          </div>
        </m.div>
      </section>
      <section className="section wrap" id="experience">
        <m.div className="section-heading" {...reveal}><div><p className="eyebrow">03 / THE JOURNEY</p><h2>A few chapters.<br /><span className="serif">One curious mind.</span></h2></div><p className="section-note">From moving data<br />to making energy visible.</p></m.div>
        <TracingBeam>
          <ol className="timeline">{experience.map((job, index) => <m.li key={job.company} {...revealStagger(index * 0.08)}><div className="timeline-date"><span className="timeline-number">0{index + 1}</span><p>{job.date}</p></div><div><h3>{job.role}</h3><p className="company-name">{job.company}</p><p className="job-description">{job.description}</p><div className="tags">{job.tags.map(tag => <span key={tag}>{tag}</span>)}</div></div></m.li>)}</ol>
        </TracingBeam>
      </section>
      <section className="writing wrap" id="writing">
        <m.div className="section-heading" {...reveal}><div><p className="eyebrow">04 / NOTES FROM THE WORKBENCH</p><h2>Built something.<br /><span className="serif">Learned something.</span></h2></div><span className="journal-count">{String(posts.length).padStart(2, '0')} ENTRIES / FIELD NOTES</span></m.div>
        {posts.map((post, index) => (
          <m.div key={post.slug} {...revealStagger(index * 0.08)} className="journal-spotlight-wrapper">
            <CardSpotlight color="rgba(206, 73, 39, 0.12)">
              <a className="journal-entry" href={`#/blog/${post.slug}`}>
                <div className="journal-art" aria-hidden="true">
                  <span>FIELD NOTE / {String(posts.length - index).padStart(3, '0')}</span>
                  <strong>{post.projectName === 'Threadlane' ? '⌘' : 'ƒ'}<span>({post.projectName === 'Threadlane' ? 'lanes' : 'motion'})</span></strong>
                  <small>{post.artLabel}</small>
                </div>
                <div className="journal-copy">
                  <p className="eyebrow">{post.tags.join(' / ')} · {post.readingMinutes} MIN READ</p>
                  <h3>{post.title}</h3>
                  <p>{post.description}</p>
                  <div><time dateTime={post.isoDate}>{post.date}</time><span>Read the story <Arrow /></span></div>
                </div>
              </a>
            </CardSpotlight>
          </m.div>
        ))}
      </section>
      <footer id="contact" className="contact"><m.div className="wrap" {...reveal}><p className="eyebrow">05 / THE NEXT CONVERSATION</p><h2>Something worth<br /><span className="serif">building together?</span></h2><div className="contact-actions"><a className="button light shimmer" href="https://linkedin.com/in/wheregmis" target="_blank" rel="noreferrer">Let’s connect on LinkedIn <Arrow /></a><p>Energy systems, open-source tools,<br />or an interesting problem. Say hello.</p></div><div className="footer-bottom"><span>© {new Date().getFullYear()} Sabin Regmi</span><div><a href="https://github.com/wheregmis" target="_blank" rel="noreferrer">GitHub <Arrow /></a><a href="https://twitter.com/wheregmis" target="_blank" rel="noreferrer">X / Twitter <Arrow /></a></div><a href="#home" onClick={e => handleNavClick(e, 'home')}>Back to top ↑</a></div></m.div></footer>
    </main>
    <AnimatePresence>
      {showScrollTop && (
        <m.button
          className="floating-top-button"
          onClick={e => handleNavClick(e, 'home')}
          aria-label="Scroll to top"
          title="Scroll to top"
          initial={{ opacity: 0, scale: 0.75, y: 16 }}
          animate={{ opacity: 1, scale: 1, y: 0 }}
          exit={{ opacity: 0, scale: 0.75, y: 16 }}
          whileHover={reduced ? undefined : { scale: 1.1, y: -2 }}
          whileTap={reduced ? undefined : { scale: 0.92 }}
          transition={{ duration: 0.25, ease: smoothEase }}
        >
          <span aria-hidden="true">↑</span>
        </m.button>
      )}
    </AnimatePresence>
  </MotionConfig></LazyMotion>;
}
