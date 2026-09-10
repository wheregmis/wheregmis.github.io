export const posts = [{
  slug: 'building-threadlane',
  file: 'building_threadlane.md',
  title: 'Building Threadlane',
  subtitle: 'A native workspace for durable agent work',
  description: 'Bringing a native Rust interface, branching conversations, and durable execution into one coding workspace.',
  date: 'September 10, 2026',
  isoDate: '2026-09-10',
  readingMinutes: 4,
  tags: ['Rust', 'GPUI', 'Agent systems'],
  projectUrl: 'https://github.com/wheregmis/threadlane',
  projectName: 'Threadlane',
  artLabel: 'ON AGENTS & DURABILITY',
}, {
  slug: 'building-dioxus-motion',
  file: 'building_dioxus_motion.md',
  isoDate: '2024-03-15',
  projectUrl: 'https://github.com/wheregmis/dioxus-motion',
  projectName: 'dioxus-motion',
  artLabel: 'ON PHYSICS & INTERFACES',
  title: 'Building dioxus-motion',
  subtitle: 'A physics-based animation library for Rust',
  description: 'The thinking behind natural motion, spring physics, and a cross-platform animation library.',
  date: 'March 15, 2024',
  readingMinutes: 4,
  tags: ['Rust', 'Dioxus', 'Animation'],
}];

export function markdownBody(source) {
  return source.replace(/^\+\+\+\r?\n[\s\S]*?\r?\n\+\+\+(?:\r?\n|$)/, '').trim();
}
