import { useEffect, useRef, useState, useCallback } from 'react';
import { m, AnimatePresence, useReducedMotion, useScroll, useSpring, useTransform } from 'framer-motion';

/**
 * Aceternity FlipWords: Smoothly rotates words with vertical slide, fade, and blur transitions.
 */
export function FlipWords({ words, duration = 3000, className = '' }) {
  const [currentWord, setCurrentWord] = useState(words[0]);
  const reduced = useReducedMotion();

  const startAnimation = useCallback(() => {
    const nextIndex = (words.indexOf(currentWord) + 1) % words.length;
    setCurrentWord(words[nextIndex]);
  }, [currentWord, words]);

  useEffect(() => {
    if (reduced) return;
    const interval = setInterval(startAnimation, duration);
    return () => clearInterval(interval);
  }, [reduced, startAnimation, duration]);

  if (reduced) {
    return <span className={`flip-word-static ${className}`}>{words[0]}</span>;
  }

  return (
    <span className={`flip-words-container ${className}`}>
      <AnimatePresence mode="wait">
        <m.span
          key={currentWord}
          initial={{ opacity: 0, y: 14, filter: 'blur(6px)' }}
          animate={{ opacity: 1, y: 0, filter: 'blur(0px)' }}
          exit={{ opacity: 0, y: -14, filter: 'blur(6px)' }}
          transition={{ duration: 0.45, ease: [0.22, 1, 0.36, 1] }}
          className="flip-word-item"
        >
          {currentWord}
        </m.span>
      </AnimatePresence>
    </span>
  );
}

/**
 * Aceternity CardSpotlight: A subtle mouse-following radial gradient glow effect.
 */
export function CardSpotlight({ children, className = '', radius = 350, color = 'rgba(206, 73, 39, 0.12)', ...props }) {
  const divRef = useRef(null);
  const [isFocused, setIsFocused] = useState(false);
  const [position, setPosition] = useState({ x: 0, y: 0 });
  const [opacity, setOpacity] = useState(0);
  const reduced = useReducedMotion();

  const handleMouseMove = (e) => {
    if (!divRef.current || reduced) return;
    const rect = divRef.current.getBoundingClientRect();
    setPosition({ x: e.clientX - rect.left, y: e.clientY - rect.top });
  };

  const handleFocus = () => {
    setIsFocused(true);
    setOpacity(1);
  };

  const handleBlur = () => {
    setIsFocused(false);
    setOpacity(0);
  };

  const handleMouseEnter = () => {
    if (!reduced) setOpacity(1);
  };

  const handleMouseLeave = () => {
    setOpacity(0);
  };

  return (
    <div
      ref={divRef}
      onMouseMove={handleMouseMove}
      onFocus={handleFocus}
      onBlur={handleBlur}
      onMouseEnter={handleMouseEnter}
      onMouseLeave={handleMouseLeave}
      className={`card-spotlight ${className}`}
      {...props}
    >
      {!reduced && (
        <div
          className="spotlight-glow"
          style={{
            opacity,
            background: `radial-gradient(${radius}px circle at ${position.x}px ${position.y}px, ${color}, transparent 80%)`,
          }}
          aria-hidden="true"
        />
      )}
      {children}
    </div>
  );
}

/**
 * Aceternity TracingBeam: A luminous SVG path that traces scroll progress through a container.
 */
export function TracingBeam({ children, className = '' }) {
  const containerRef = useRef(null);
  const contentRef = useRef(null);
  const [svgHeight, setSvgHeight] = useState(0);
  const reduced = useReducedMotion();

  const { scrollYProgress } = useScroll({
    target: containerRef,
    offset: ['start center', 'end center'],
  });

  const pathLength = useSpring(scrollYProgress, {
    stiffness: 400,
    damping: 60,
  });

  const y1 = useTransform(pathLength, [0, 1], [30, Math.max(svgHeight - 30, 30)]);
  const y2 = useTransform(pathLength, [0, 1], [0, Math.max(svgHeight - 60, 0)]);

  useEffect(() => {
    if (!contentRef.current) return;
    const updateHeight = () => {
      if (contentRef.current) {
        setSvgHeight(contentRef.current.offsetHeight);
      }
    };
    updateHeight();
    const observer = new ResizeObserver(updateHeight);
    observer.observe(contentRef.current);
    return () => observer.disconnect();
  }, []);

  if (reduced) {
    return <div className={`tracing-beam-container ${className}`}>{children}</div>;
  }

  return (
    <div ref={containerRef} className={`tracing-beam-container ${className}`}>
      <div className="tracing-beam-track" aria-hidden="true">
        <m.div
          style={{
            boxShadow: '0 0 16px rgba(206, 73, 39, 0.6), 0 0 4px rgba(206, 73, 39, 0.8)',
            top: y1,
          }}
          className="tracing-beam-dot"
        />
        <svg
          viewBox={`0 0 20 ${svgHeight}`}
          width="20"
          height={svgHeight}
          className="tracing-beam-svg"
          aria-hidden="true"
        >
          <path
            d={`M 10 0 V ${svgHeight}`}
            fill="none"
            stroke="#d4d8cc"
            strokeWidth="1.5"
            strokeDasharray="4 4"
          />
          <m.path
            d={`M 10 0 V ${svgHeight}`}
            fill="none"
            stroke="url(#tracing-gradient)"
            strokeWidth="2.5"
            transition={{ duration: 0.5 }}
            style={{
              pathLength: pathLength,
            }}
          />
          <defs>
            <linearGradient id="tracing-gradient" gradientUnits="userSpaceOnUse" x1="0" x2="0" y1={y2} y2={y1}>
              <stop stopColor="#ce4927" stopOpacity="0" />
              <stop stopColor="#ce4927" />
              <stop offset="0.8" stopColor="#f4864b" />
              <stop offset="1" stopColor="#263c30" stopOpacity="0" />
            </linearGradient>
          </defs>
        </svg>
      </div>
      <div ref={contentRef} className="tracing-beam-content">
        {children}
      </div>
    </div>
  );
}

/**
 * Aceternity 3D Tilt Card: Subtle interactive perspective tilt responding to mouse position.
 */
export function TiltCard({ children, className = '', maxTilt = 8, ...props }) {
  const cardRef = useRef(null);
  const [rotateX, setRotateX] = useState(0);
  const [rotateY, setRotateY] = useState(0);
  const [isHovered, setIsHovered] = useState(false);
  const reduced = useReducedMotion();

  const handleMouseMove = (e) => {
    if (!cardRef.current || reduced) return;
    const rect = cardRef.current.getBoundingClientRect();
    const x = e.clientX - rect.left;
    const y = e.clientY - rect.top;
    const centerX = rect.width / 2;
    const centerY = rect.height / 2;
    const rotX = ((y - centerY) / centerY) * -maxTilt;
    const rotY = ((x - centerX) / centerX) * maxTilt;
    setRotateX(rotX);
    setRotateY(rotY);
  };

  const handleMouseEnter = () => {
    if (!reduced) setIsHovered(true);
  };

  const handleMouseLeave = () => {
    setIsHovered(false);
    setRotateX(0);
    setRotateY(0);
  };

  if (reduced) {
    return <div className={className} {...props}>{children}</div>;
  }

  return (
    <m.div
      ref={cardRef}
      onMouseMove={handleMouseMove}
      onMouseEnter={handleMouseEnter}
      onMouseLeave={handleMouseLeave}
      animate={{
        rotateX: isHovered ? rotateX : 0,
        rotateY: isHovered ? rotateY : 0,
      }}
      transition={{ type: 'spring', stiffness: 350, damping: 25 }}
      style={{ transformStyle: 'preserve-3d', perspective: 1000 }}
      className={`tilt-card ${className}`}
      {...props}
    >
      {children}
    </m.div>
  );
}
