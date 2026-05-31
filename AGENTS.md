# AGENTS.md

Guidance for AI agents (and humans) working in this repository.

## What this is

"What & When" (whatandwhen.fyi) — a personal dashboard for due-date countdowns and top-of-mind notes. Deployed as a static site on GitHub Pages (see `CNAME`).

## Running & testing

There is **no build step, no bundler, no test suite, no dependencies**. The entire app is one file: `index.html`.

- Develop: open `index.html` directly, or `python3 -m http.server 8080` and visit it.
- "Test" = manual verification in a browser. There is no lint config or CI.

## Architecture

Everything — markup, CSS, and JS — lives inline in `index.html`. Treat that single file as the whole codebase.

- **Three sections, three independent localStorage keys**, each with its own load/save/render pair. They stack full-width in source/visual order — **What**, then **When**, then **Words** — under a pinned top cluster (wordmark · export · theme toggle):
  - **What** (`…-what-v1`) — top-of-mind cards, drag-reorderable. `loadWhat`/`saveWhat`/`renderWhat`.
  - **When** (`…-when-v1`) — countdown cards with due dates. `load`/`save`/`render`.
  - **Words** (`…-words-v1`) — a debounced scratch textarea. `loadWords`/`saveWords`.
- **Domains namespace the keys.** The three key consts derive from `keyFor(part)`, which prepends an optional domain from the URL hash (`domainFromHash()` → `DOMAIN`). No hash = the unprefixed default keys (`whatandwhen-when-v1`, etc.) — the original, never-migrated store. A hash like `#work` yields `whatandwhen-work-when-v1` and friends. The domain is lowercased and sanitized to `[a-z0-9-]`; `#data=…` is recognized as an import hash, not a domain. Each domain is fully independent (no global/aggregate view by design). Switching domains is URL-driven: a `hashchange` listener calls `location.reload()` so each domain boots cleanly. Theme is **not** namespaced — it stays global.
- **localStorage is the only data store** — no server, no accounts, no sync. Corrupt JSON is caught and reseeded (`makeSeed` / `WHAT_SEED`). Seed data is generated relative to `new Date()` so the demo always looks current. **Only the default domain is seeded** — named domains (`DOMAIN` truthy) start empty (`load`/`loadWhat` return `[]`).
- **Render model is full-redraw.** `render()` / `renderWhat()` wipe the grid (`innerHTML = ""`) and rebuild every card from state on each change. Inline editing swaps a card's `innerHTML` for a `<textarea>` + action buttons. A 60s `setInterval` re-renders to keep countdowns live, but **skips while a `.editing` or `.add-card` card is open** — preserve this guard when touching the interval, edit, or add flow.
- **Adding a card** is driven by a `+` button in each section header (When/What), not a placeholder card in the grid. `showWhenAdd()` / `showWhatAdd()` append a one-off `.add-card` with an inline `<textarea>` at the end of the grid; Enter saves, Esc/empty-blur dismisses, non-empty blur saves. The grids themselves only ever hold real cards.
- **Layout is pure stacked flow.** Each section is a full-width `.section`; the When and What card grids are CSS grid with `repeat(auto-fill, minmax(min(var(--grid-min), 100%), 1fr))` — the `min(…, 100%)` clamp lets cards shrink to a single fitting column on narrow viewports (phones) instead of overflowing. There is no separate mobile breakpoint.
- **The LCP element is JS-rendered** (the big `.card-days` number). The static HTML ships empty `#grid`/`#what-grid`. Keep the app logic inline and synchronous in `<head>`/end-of-`<body>`; moving it to an external/deferred script or adding a fetch would turn cheap render-delay into a request chain and regress LCP. The app is non-functional without JS by design (client-only localStorage app).

### Date parsing (`parseDate` / `splitInput` / `extractTime`)

Natural-language date entry is the most intricate logic. `splitInput` separates a card name from a trailing date phrase using an ordered list of regexes; `parseDate` interprets the phrase; `extractTime` peels off a trailing `at 3pm`. Supported forms and defaults are documented in `README.md` — keep that table in sync when changing parsing.

- Default time is **9:00 AM**, except a bare card with no detected date defaults to **today 5:00 PM** (`parseWhenInput`).
- **Strict by design:** month and day names must be full words where the code requires it; do not loosen input validation to save code. Day names spelled in full (`friday`, not `fri`). `urgency()` classifies cards (done / past / urgent ≤12h / soon ≤5d / ok) and drives card color.

### Other behaviors

- **Theme:** dark default; `prefers-color-scheme` + a manual toggle persisted to `whatandwhen-theme-v1` (global — not per-domain). A tiny pre-paint inline script in `<head>` applies the saved theme before first paint to avoid a flash — keep it inline and first, and do not make it domain-aware.
- **Export/import:** "export" base64-encodes the current domain's three stores **plus a `domain` field** into a `#data=` URL fragment (`encodeData`/`decodeData`, using `encodeURIComponent`+`unescape` to survive Unicode). On load, a `#data=` hash imports into the payload's domain's keys; if that domain matches the current one it clears the hash via `history.replaceState`, otherwise it `location.replace`s to `#<domain>` for a clean load. Payloads with no `domain` field import into the default domain (backward-compatible with pre-domains export links).
- **Domain label:** when `DOMAIN` is set, init appends a `·`-separated tag (reusing `.wordmark`/`.cluster-sep`, set via `textContent`) after the wordmark and updates `document.title`. Default domain shows no extra label.
- All user text goes through `escHtml()` before insertion. Card content is built with `innerHTML` string concatenation — any new dynamic text must be escaped the same way.
- Fonts load non-render-blocking via the `media="print" onload` swap with a `<noscript>` fallback. Preserve that pattern.

## Conventions

- **Prioritize simplicity and human readability above all.** This is a small single-file app meant to stay legible end to end — prefer the plainest code that works over clever or abstracted solutions, and don't add structure (frameworks, build steps, dependencies, indirection) the app doesn't need.
- **Lighthouse must stay at 100 across all categories.** Verify before considering a change done; treat a drop as a regression to fix, not accept. (See the JS-rendered LCP note above — the inline, dependency-free structure is what keeps the scores perfect.)
- Match the existing inline style: vanilla JS, no framework, no external deps, CSS custom properties in `:root` (and `:root.theme-light`) for all theming.
- Update `README.md`'s date-parsing table when parsing behavior changes.

## Working principles

- **Think before coding.** State assumptions explicitly; if a request has multiple interpretations or a simpler approach exists, surface it rather than silently picking. If something is unclear, stop and ask — don't paper over confusion.
- **Surgical changes.** Touch only what the request requires. Don't refactor, reformat, or "improve" adjacent code that isn't broken, and match the surrounding style even if you'd do it differently. Remove only the orphans your own change created; if you spot unrelated dead code, mention it instead of deleting it. Every changed line should trace to the request.
- **Goal-driven execution.** Turn the task into a concrete success check and loop until it's verified — for this app that means manually exercising the change in a browser and confirming Lighthouse is still 100. State a brief plan for multi-step work.
