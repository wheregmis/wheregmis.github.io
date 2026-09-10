import { useEffect, useRef, useState } from 'react';
import { useReducedMotion } from 'framer-motion';
import * as THREE from 'three';

export default function Sculpture({ paused, stage }) {
  const host = useRef(null);
  const reduced = useReducedMotion();
  const state = useRef({ paused, reduced, stage });
  const update = useRef(() => {});
  const [unavailable, setUnavailable] = useState(false);
  useEffect(() => { state.current = { paused, reduced, stage }; update.current(); }, [paused, reduced, stage]);
  useEffect(() => {
    const element = host.current;
    let renderer;
    try { renderer = new THREE.WebGLRenderer({ antialias: true, alpha: true }); }
    catch { setUnavailable(true); return; }
    renderer.setPixelRatio(Math.min(window.devicePixelRatio, 1.5));
    element.appendChild(renderer.domElement);
    const scene = new THREE.Scene();
    const camera = new THREE.PerspectiveCamera(38, 1, 0.1, 100);
    camera.position.set(0, 4.5, 8.5);
    camera.lookAt(0, 0, 0);
    scene.add(new THREE.HemisphereLight('#fff9ed', '#465746', 3));
    const light = new THREE.DirectionalLight('#ffffff', 4);
    light.position.set(-3, 6, 5);
    scene.add(light);
    const geometry = new THREE.BoxGeometry(1, 1, 1);
    const materials = [0, 1, 2].map(() => new THREE.MeshStandardMaterial({ color: '#9ca98d', roughness: 0.45, metalness: 0.2 }));
    const dark = new THREE.MeshStandardMaterial({ color: '#293b31', roughness: 0.6 });
    const orange = new THREE.MeshStandardMaterial({ color: '#f47b46', emissive: '#b43c12', emissiveIntensity: 0.3 });
    const box = (x, y, z, w, h, d, material) => {
      const mesh = new THREE.Mesh(geometry, material);
      mesh.position.set(x, y, z); mesh.scale.set(w, h, d); scene.add(mesh);
      return mesh;
    };
    // A conceptual system diagram, not a live battery or production topology.
    [-2.3, 0, 2.3].forEach(x => box(x, -0.5, 0, 1.8, 0.12, 1.55, dark));
    for (let i = 0; i < 6; i++) {
      const x = -2.8 + (i % 3) * 0.5, z = Math.floor(i / 3) * 0.65 - 0.3;
      box(x, 0.08, z, 0.37, 1.05, 0.48, materials[0]);
      box(x, 0.64, z, 0.16, 0.1, 0.22, dark);
    }
    box(0, -0.1, 0, 1.35, 0.62, 1.05, materials[1]);
    for (let i = 0; i < 4; i++) box(-0.42 + i * 0.28, 0.24, 0, 0.12, 0.07, 0.72, dark);
    box(2.3, 0.38, 0, 1.5, 1.05, 0.17, dark);
    box(2.3, 0.4, 0.1, 1.32, 0.85, 0.03, materials[2]);
    for (let i = 0; i < 5; i++) box(1.83 + i * 0.23, 0.21 + i * 0.045, 0.14, 0.1, 0.2 + i * 0.09, 0.04, dark);
    box(2.3, -0.28, 0, 0.18, 0.38, 0.2, dark);
    const cableGeometry = new THREE.BufferGeometry().setFromPoints([new THREE.Vector3(-2.3,-0.4,0.85),new THREE.Vector3(-2.3,-0.4,1.15),new THREE.Vector3(2.3,-0.4,1.15),new THREE.Vector3(2.3,-0.4,0.85)]);
    const cableMaterial = new THREE.LineBasicMaterial({ color: '#899681' });
    scene.add(new THREE.Line(cableGeometry, cableMaterial));
    const signals = [0,1,2].map(() => box(0,-0.39,1.15,0.1,0.1,0.1,orange));
    let phase = 0, previousTime, visible = true;
    const baseColor = new THREE.Color('#9ca98d');
    const activeColor = new THREE.Color('#f07943');
    const render = (time) => {
      if (time !== undefined && previousTime !== undefined) phase += Math.min(time - previousTime, 50) * 0.00016;
      previousTime = time;
      signals.forEach((signal, i) => { signal.position.x = -2.3 + ((phase + i / 3) % 1) * 4.6; });
      materials.forEach((material, i) => {
        const target = i === state.current.stage ? activeColor : baseColor;
        if (state.current.reduced || state.current.paused) {
          material.color.copy(target);
        } else {
          material.color.lerp(target, 0.09);
        }
      });
      if (!state.current.reduced && !state.current.paused && time !== undefined) {
        const targetCamX = (state.current.stage - 1) * 0.3 + Math.sin(time * 0.0005) * 0.18;
        const targetCamY = 4.5 + Math.cos(time * 0.0007) * 0.1;
        camera.position.x += (targetCamX - camera.position.x) * 0.04;
        camera.position.y += (targetCamY - camera.position.y) * 0.04;
        camera.lookAt(0, 0, 0);
        orange.emissiveIntensity = 0.3 + Math.sin(time * 0.004) * 0.12;
      }
      renderer.render(scene, camera);
    };
    const resize = () => {
      const {width, height} = element.getBoundingClientRect();
      camera.aspect = width / Math.max(height, 1); camera.updateProjectionMatrix();
      renderer.setSize(width, height); render();
    };
    const sync = () => {
      renderer.setAnimationLoop(null); previousTime = undefined; render();
      if (!state.current.paused && !state.current.reduced && visible && !document.hidden) renderer.setAnimationLoop(render);
    };
    update.current = sync;
    const observer = new ResizeObserver(resize); observer.observe(element);
    const visibility = new IntersectionObserver(([entry]) => { visible = entry.isIntersecting; sync(); }); visibility.observe(element);
    document.addEventListener('visibilitychange', sync);
    resize(); sync();
    return () => {
      observer.disconnect(); visibility.disconnect(); document.removeEventListener('visibilitychange', sync);
      renderer.setAnimationLoop(null); geometry.dispose(); cableGeometry.dispose(); cableMaterial.dispose();
      [...materials, dark, orange].forEach(material => material.dispose());
      renderer.dispose(); renderer.domElement.remove(); update.current = () => {};
    };
  }, []);
  return <div className="sculpture" ref={host} aria-hidden="true">{unavailable && <div className="scene-fallback">▥ → ▦ → ▤</div>}</div>;
}
