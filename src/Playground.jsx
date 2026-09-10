import { useState } from 'react';
import { m, useReducedMotion } from 'framer-motion';

export default function Playground() {
  const [stiffness, setStiffness] = useState(100);
  const [damping, setDamping] = useState(12);
  const [destination, setDestination] = useState(false);
  const reduced = useReducedMotion();
  return <div className="playground">
    <div className="panel-label"><span>INTERACTIVE / SPRING LAB</span><span>01</span></div>
    <div className="spring-track" aria-hidden="true"><div className="track-line" /><span className="track-origin">A</span><span className="track-end">B</span><m.div className="spring-puck" initial={false} animate={{ left: destination ? 'calc(100% - 60px)' : '16px', rotate: destination ? 180 : 0 }} whileHover={reduced ? undefined : { scale: 1.08 }} whileTap={reduced ? undefined : { scale: 0.92 }} transition={reduced ? {duration:0} : {type:'spring', stiffness, damping, mass:1}}>✳</m.div></div>
    <div className="spring-controls">
      <label>Stiffness <output>{stiffness}</output><input aria-label="Spring stiffness" type="range" min="40" max="250" step="10" value={stiffness} onChange={event => setStiffness(Number(event.target.value))} /></label>
      <label>Damping <output>{damping}</output><input aria-label="Spring damping" type="range" min="5" max="35" value={damping} onChange={event => setDamping(Number(event.target.value))} /></label>
    </div>
    <div className="demo-bottom"><m.button whileTap={reduced ? undefined : { scale: 0.97 }} className="button dark" onClick={() => setDestination(!destination)}>Run spring {destination ? '←' : '→'}</m.button><span aria-live="polite">Target {destination ? 'B' : 'A'} · {reduced ? 'Reduced motion' : 'Adjust. Run. Feel the difference.'}</span></div>
    <p className="demo-note">A Framer Motion illustration of the spring concepts behind Dioxus Motion.</p>
  </div>;
}
