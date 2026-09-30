import { useEffect, useRef, useState } from 'react';
import { useReducedMotion } from 'framer-motion';

/** A lightweight, progressively enhanced signal diagram. No WebGL required. */
export default function SignalSculpture({ paused, stage }) {
  const ref = useRef(null);
  const reduced = useReducedMotion();
  const [visible, setVisible] = useState(true);
  const [foreground, setForeground] = useState(true);
  useEffect(() => {
    const observer = new IntersectionObserver(([entry]) => setVisible(entry.isIntersecting));
    if (ref.current) observer.observe(ref.current);
    const onVisibility = () => setForeground(!document.hidden);
    document.addEventListener('visibilitychange', onVisibility);
    onVisibility();
    return () => { observer.disconnect(); document.removeEventListener('visibilitychange', onVisibility); };
  }, []);
  return <div ref={ref} className={`signal-sculpture signal-stage-${stage}`} data-paused={paused || reduced || !visible || !foreground} aria-hidden="true">
    <div className="signal-coordinate coordinate-top">INPUT / 0{stage + 1}</div>
    <div className="signal-coordinate coordinate-bottom">SIGNAL → SYSTEM → EXPERIENCE</div>
    <div className="signal-orbit orbit-one"><span /></div>
    <div className="signal-orbit orbit-two"><span /></div>
    <div className="signal-orbit orbit-three"><span /></div>
    <div className="signal-core"><div className="core-face"><svg width="40" height="40" viewBox="0 0 40 40" fill="none" stroke="currentColor" strokeWidth="1.5">
      {stage === 0 ? <><rect x="11" y="11" width="18" height="18" rx="3"/><rect x="16" y="16" width="8" height="8" rx="1"/><path d="M15 6v5m10-5v5M15 29v5m10-5v5M6 15h5m-5 10h5m18-10h5m-5 10h5"/></> : stage === 1 ? <><path d="M4 20h5l4-10 7 20 7-20 4 10h5"/><path d="M4 7h32M4 33h32" opacity=".4"/></> : <><rect x="7" y="8" width="26" height="24" rx="3"/><path d="M7 15h26M12 21h9m-9 5h16"/></>}
    </svg></div></div>
    <div className="signal-cross cross-one">+</div><div className="signal-cross cross-two">+</div>
  </div>;
}
