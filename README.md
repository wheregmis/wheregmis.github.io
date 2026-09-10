# Sabin Regmi — Portfolio

React, Three.js, and Framer Motion, built with Vite. Requires Node.js 22.12+.

```sh
npm ci
npm run dev
npm test
npm run build
```

Edit project and experience content in `src/data.js`, page sections in `src/App.jsx`, and styles in `src/style.css`. The Three.js hero is loaded separately, respects reduced motion, pauses offscreen, and has a static fallback when WebGL is unavailable.

The previous portfolio is preserved in `backup/portfolio-before-react-2026-09-10/`, including source, assets, content, and its original deployment workflow. Its SHA-256 manifest is checked by `npm test`. This folder is not included in production output.

GitHub Actions builds on pushes to `master` and publishes `dist` to `gh-pages/docs`, preserving the original deployment target. The repository's Pages source must be `gh-pages` and `/docs`. No deployment occurs until pushed. Work history and social links were carried over from the previous portfolio; review the current-role dates before publishing.

Animation implementation references: [Three.js renderer](https://threejs.org/docs/pages/WebGLRenderer.html) and [Motion reduced motion](https://motion.dev/docs/react-use-reduced-motion).

For optional browser verification, start the dev server, then run `node tests/browser-smoke.cjs` in an environment with Playwright and Chromium installed. Set `PORTFOLIO_URL` if the server uses another port and `PLAYWRIGHT_MODULE` to use an existing Playwright installation. Screenshots are saved to the OS temporary directory.

Blog posts render as styled React pages at `#/blog/building-dioxus-motion`, so direct links and refresh work on GitHub Pages without server routing. Metadata lives in `src/blog-data.js`; the article imports its Markdown from `src/content/blog/`, strips the existing TOML frontmatter, and renders through react-markdown with GFM support. Raw HTML is disabled. The renderer is loaded only when opening an article.

The field-journal redesign includes a conceptual Three.js controller/telemetry/interface model, a Framer Motion spring playground, expandable project case notes using archived screenshots, a fully visible work timeline, sticky section navigation, and article contents links. The converter preview is illustrative; its CTA opens the original live tool. The 3D diagram is not live telemetry.

Threadlane's description, case notes, and workspace screenshot are sourced from its [public README](https://github.com/wheregmis/threadlane). The portfolio uses a resized local JPEG of that screenshot.
