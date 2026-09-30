# Sabin Regmi — Portfolio

React and Framer Motion, built with Vite. Requires Node.js 22.12+.

```sh
npm ci
npm run dev
npm test
npm run build
```

Edit project and experience content in `src/data.js`, page sections in `src/App.jsx`, and styles in `src/style.css`. The CSS signal sculpture runs without WebGL, respects reduced motion, and pauses when offscreen, in a hidden browser tab, or using its pause control. The original Three.js source is retained for reference but is no longer shipped in the production bundle.

The previous portfolio is preserved in `backup/portfolio-before-react-2026-09-10/`, including source, assets, content, and its original deployment workflow. Its SHA-256 manifest is checked by `npm test`. This folder is not included in production output.

GitHub Actions builds on pushes to `master` and publishes `dist` to `gh-pages/docs`, preserving the original deployment target. The repository's Pages source must be `gh-pages` and `/docs`. No deployment occurs until pushed. Work history and social links were carried over from the previous portfolio; review the current-role dates before publishing.

Animation implementation reference: [Motion reduced motion](https://motion.dev/docs/react-use-reduced-motion).

For optional browser verification, start the dev server, then run `node tests/browser-smoke.cjs` in an environment with Playwright and Chromium installed. Set `PORTFOLIO_URL` if the server uses another port and `PLAYWRIGHT_MODULE` to use an existing Playwright installation. Screenshots are saved to the OS temporary directory.

Blog posts render as styled React pages at `#/blog/building-dioxus-motion`, so direct links and refresh work on GitHub Pages without server routing. Metadata lives in `src/blog-data.js`; the article imports its Markdown from `src/content/blog/`, strips the existing TOML frontmatter, and renders through react-markdown with GFM support. Raw HTML is disabled. The renderer is loaded only when opening an article.

The Signal Studio redesign includes a conceptual animated controller/telemetry/interface model, a Framer Motion spring playground, expandable project case notes using archived screenshots, a fully visible work timeline, sticky section navigation, and article contents links. The converter preview is illustrative; its CTA opens the original live tool. The diagram is illustrative, not live telemetry.

Threadlane's description, case notes, and workspace screenshot are sourced from its [public README](https://github.com/wheregmis/threadlane). The portfolio uses a resized local JPEG of that screenshot.

## Signal Studio design

The redesign uses a quiet slate-and-cobalt editorial palette, a dark kinetic signal study, larger stable hero typography, rounded project studies, and a visible two-row mobile navigation. Both light and dark preferences remain supported. Motion is progressive enhancement; the headline stays readable without waiting for a text animation.

Section URLs now survive refresh and Back/Forward navigation; article contents history restores the article heading. `tests/navigation.test.mjs` covers routing behavior. `node tests/navigation-browser.cjs` is an additional optional browser regression suite (same Playwright environment setup as the smoke test).

Checks for this revision: `npm test` and `npm run build`. Browser suites require a browser-enabled local or CI environment.
