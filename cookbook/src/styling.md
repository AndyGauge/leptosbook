# Theming and styling

leptoskit ships sensible defaults in the `FOLIO_CSS` constant and injects them
automatically. You can layer on top, or take over completely.

## Layer on top (default)

Leave `inject_css=true` (the default) and just write CSS that targets the
leptoskit classes — later rules win:

```rust
<Folio items=items render=render>   // inject_css defaults to true
    <FolioNav/>
</Folio>
```

```css
/* your stylesheet, loaded after the component */
.folio-nav-btn {
  background: #2563eb;
  color: white;
  border: none;
  padding: .5rem 1.25rem;
  border-radius: 6px;
}
.folio-nav-btn:hover:not(:disabled) { background: #1d4ed8; }
.folio-nav-counter { font-variant-numeric: tabular-nums; }
```

## Take over completely

Set `inject_css=false` and supply your own — optionally starting from the
defaults via the `FOLIO_CSS` constant:

```rust
<Folio inject_css=false items=items render=render>
    <style>{leptoskit::FOLIO_CSS}</style>   // start from defaults…
    <style>{ "/* …then your overrides */" }</style>
    <FolioNav/>
</Folio>
```

Or omit `FOLIO_CSS` entirely for a clean slate.

## Class reference

| Class | Element |
|-------|---------|
| `.folio` | Outer focusable container |
| `.folio-page-slot` | Clipping viewport for the page |
| `.folio-page` | The current page (absolutely positioned) |
| `.folio-enter-left` / `.folio-enter-right` | Direction-aware slide-in |
| `.folio-empty` | Wrapper for `empty_fallback` |
| `.folio-nav` | `FolioNav` container |
| `.folio-nav-btn` / `:disabled` | Nav buttons |
| `.folio-nav-counter` | The `n / total` text |
| `.folio-tabs` | `FolioTabs` container |
| `.folio-tab` / `.folio-tab.active` | Individual tab |

## Customizing the transition

The slide-in direction is chosen from `last_dir`: forward turns get
`.folio-enter-left`, backward turns get `.folio-enter-right`. Override the
keyframes to change the feel — here, a cross-fade instead of a slide:

```css
.folio-enter-left, .folio-enter-right {
  animation: fade .2s ease both;
}
@keyframes fade { from { opacity: 0 } to { opacity: 1 } }
```

## Respecting reduced motion

Be kind to users who ask for less animation:

```css
@media (prefers-reduced-motion: reduce) {
  .folio-enter-left, .folio-enter-right { animation: none; }
}
```
