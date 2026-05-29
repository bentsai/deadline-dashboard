# Wordmark — Design

## Goal

Add a "What & When" wordmark to the dashboard to give the page a sense of identity (branding/personality). The page is currently functional but anonymous.

## Decision summary

- **Text:** `What & When`
- **Treatment:** Yellow chip — solid `--accent` (#ffcc00) background, black text, Rubik 900 caps, slight letter-spacing.
- **Placement:** Inline (right-aligned) with the existing `<h1>When</h1>`, sharing the white border-bottom.
- **Linkable:** Yes — `<a href="/">` so clicking it returns to home / refreshes state. Future-friendly for adding sibling links (e.g., "about") next to it.

## Markup

Wrap the existing `<h1>` in a `<header>` row with the wordmark. Replace:

```html
<h1>When</h1>
```

With:

```html
<header class="header-row">
  <h1>When</h1>
  <a class="wordmark" href="/" aria-label="What and When — home">What &amp; When</a>
</header>
```

## Styles

- Move `border-bottom`, `padding-bottom`, and `margin-bottom` off `h1` and onto `.header-row` so the underline spans both elements.
- Reset those properties on `h1` (since the global `h1` rule still applies).
- New `.wordmark` rule using existing tokens — no new color introduced.

```css
.header-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 2rem;
  border-bottom: 6px solid var(--fg);
  padding-bottom: 1rem;
}
.header-row h1 {
  margin-bottom: 0;
  border-bottom: none;
  padding-bottom: 0;
}
.wordmark {
  display: inline-block;
  background: var(--accent);
  color: var(--card-fg);
  font-weight: 900;
  font-size: .95rem;
  letter-spacing: .02em;
  text-transform: uppercase;
  padding: .4rem .6rem;
  text-decoration: none;
  transition: background-color .15s;
}
.wordmark:hover { background: var(--fg); }
.wordmark:focus-visible { outline: 2px solid var(--accent); outline-offset: 2px; }
```

## Why these choices

- **Yellow chip over text-only:** Most "logo-y" of the treatments tested. Reads as a brand mark, not a label.
- **Inline placement:** Saves ~80px of vertical space above the fold compared to a separate top row. The hierarchy concern (wordmark adjacent to a section heading) is small for a personal dashboard.
- **Existing tokens only:** Reuses `--accent`, `--card-fg`, `--fg` — no new colors. Hover swaps to white, which keeps the chip readable while signaling interactivity.
- **Letter-spacing `.02em` (not `.1em`):** Tighter, more label-like, less competing with the heavily tracked `<h1>`.
- **Real `<a>` element:** Semantic. Wordmarks-as-links is conventional. Future-friendly for adding sibling links in the same flex row.

## Out of scope

- **No "about" link yet** — the placeholder space is implicit (the flex container will accommodate one when ready), but no markup is added.
- **No new colors or font-family changes.**
- **No mobile-specific layout** — relies on existing responsive behavior. The header row will wrap if needed.

## Accessibility notes

- `aria-label="What and When — home"` clarifies destination for screen readers.
- `:focus-visible` outline ensures keyboard users see focus state without yellow-on-yellow contrast issues.
- Black text on `#ffcc00` yellow has contrast ratio ~13:1 — well above WCAG AA.

## Testing

- Visual: confirm the underline spans the full row including under the wordmark.
- Keyboard: tab to the wordmark, verify focus ring is visible.
- Responsive: check that the header row doesn't break at narrow widths.
