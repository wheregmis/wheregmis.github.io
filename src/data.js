export const projects = [
  { title: 'Dioxus Motion', category: 'OPEN SOURCE · ANIMATION', description: 'Making motion feel natural in Rust. A cross-platform animation library for expressive web, desktop, and mobile interfaces.', tags: ['Rust', 'Dioxus', 'Spring physics'], href: 'https://crates.io/crates/dioxus-motion', type: 'motion' },
  { title: 'HTML → RSX', category: 'DEVELOPER TOOL · CONVERTER', description: 'Less translating. More building. A focused tool that turns HTML into Dioxus RSX with a single click.', tags: ['Rust', 'Dioxus', 'Developer experience'], href: 'https://wheregmis.github.io/dioxus_html_rsx/', type: 'code' },
  { title: 'Threadlane', category: 'NATIVE APP · AI CODING WORKSPACE', description: 'A native home for agent-assisted development. Built in Rust with GPUI, Threadlane brings project workspaces, branching conversations, and persistent terminals into one focused desktop app.', tags: ['Rust', 'GPUI', 'MCP', 'WASI'], href: 'https://github.com/wheregmis/threadlane', type: 'threadlane' },
];
export const experience = [
  { company: 'TROES Corp', role: 'Software Engineer', date: 'May 2023 — Present', description: 'Building battery controllers in Rust and remote monitoring systems for renewable energy. Connecting real-time data with useful, reliable interfaces.', tags: ['Rust', 'Python', 'AWS', 'Grafana', 'InfluxDB'] },
  { company: 'Lambton College', role: 'Research Assistant', date: 'Aug 2022 — Apr 2023', description: 'Developed web and mobile monitoring applications for battery energy storage, bringing real-time system performance data to the people who need it.', tags: ['Flutter', 'Vue.js', 'Django', 'Docker'] },
  { company: 'Seva Development', role: 'Contract Software Engineer', date: 'Oct 2021 — Jan 2022', description: 'Built a data migration engine connecting MySQL, Oracle, PostgreSQL, and Salesforce, with a focus on reliable, accurate transfers.', tags: ['Python', 'AWS Lambda', 'PostgreSQL'] },
];

export const systemStages = [
  { name: 'Controller', label: '01 / HARDWARE', title: 'Start with the physical world.', description: 'Rust-powered battery controllers connect software to energy storage. Reliability starts here.', detail: 'Battery systems · Rust' },
  { name: 'Telemetry', label: '02 / OBSERVABILITY', title: 'Make system behavior visible.', description: 'Collect and organize performance data so operators can understand what the system is doing.', detail: 'InfluxDB · Grafana · Python' },
  { name: 'Interface', label: '03 / HUMAN EXPERIENCE', title: 'Turn data into understanding.', description: 'Web and mobile interfaces bring system information to the people monitoring it.', detail: 'Vue.js · Django · Flutter' },
];

export const caseStudies = [
  { problem: 'Rust interfaces needed motion that felt natural across web and native platforms.', approach: 'Build around spring physics and tweening, with typed animation values and a hook-based API that fits Dioxus.', outcome: 'An open-source animation library with examples ranging from simple values to coordinated transforms.', image: '/images/dioxus-motion.jpg', caption: 'Original Dioxus Motion showcase: cube, flower, shape, and progress experiments.' },
  { problem: 'Moving HTML into a Dioxus interface means translating markup into RSX syntax.', approach: 'Keep the workflow focused: an HTML input, a conversion action, and a separate output pane for the generated RSX.', outcome: 'A browser-based converter that makes a repetitive part of building Dioxus interfaces easier.', image: '/images/html-rsx.jpg', caption: 'Original converter interface, with HTML input and generated RSX side by side.' },
  { problem: 'Bring project context, agent conversations, and tool execution into a single native workspace.', approach: 'Pair a GPU-accelerated GPUI interface with a durable, multi-lane runtime. Connect providers and external agents through MCP, ACP, and sandboxed WASI extensions.', outcome: 'A Rust desktop application with persistent sessions, integrated terminals, Git diffs, and an execution inspector.', image: '/images/threadlane-workspace.jpg', caption: 'Threadlane desktop workspace — screenshot from the project README.' },
];
