# leptosbook Guide

A complete tour of building gesture-driven, paginated interfaces with Leptos `0.9`.

## Contents

1. [Mental model](#mental-model)
2. [Folio](#folio)
3. [Built-in navigation](#built-in-navigation)
4. [Driving navigation yourself](#driving-navigation-yourself)
5. [Gestures](#gestures)
6. [The install prompt](#the-install-prompt)
7. [Styling](#styling)
8. [Recipes](#recipes)
9. [Gotchas](#gotchas)

---

## Mental model

leptosbook is built around one component: **`Folio`**. Think of it as a book.

- You hand it a `Signal<Vec<T>>` and a `render` function.
- It shows exactly **one item at a time**.
- It owns all navigation state and shares it through a **context** (`FolioContext`).
- Any descendant — a nav bar, a tab strip, a progress dot — can read that context (`use_folio_context()`) to display state or drive a turn.

```
swipe / key / click
        │
        ▼
  Folio updates current_page  ──►  FolioContext signals fire
        │                                   │
        ▼                                   ▼
  page re-renders with             your descendants
  a direction-aware                re-render (counters,
  slide animation                  tabs, dots, …)
```

That's the whole architecture. Everything below is detail.

---

## Folio

```rust
use leptos::prelude::*;
use leptosbook::prelude::*;

#[derive(Clone)]
struct Item { title: String, body: String }

#[component]
fn Reader(items: Signal<Vec<Item>>) -> impl IntoView {
    view! {
        <Folio
            items=items
            render=|it: Item| view! {
                <article>
                    <h1>{it.title}</h1>
                    <p>{it.body}</p>
                </article>
            }
        >
            <FolioNav/>
        </Folio>
    }
}
```

### Props

| Prop | Type | Default | Notes |
|------|------|---------|-------|
| `items` | `Signal<Vec<T>>` | — | The pages. Changing it reactively re-renders. |
| `render` | `impl Fn(T) -> IV + Clone + Send + Sync + 'static` | — | Called **only for the visible page**. |
| `threshold` | `f64` | `60.0` | Min pointer travel (px) to count as a swipe. |
| `inject_css` | `bool` | `true` | Inject `FOLIO_CSS`. Set `false` to fully self-style. |
| `on_page_change` | `Option<Arc<dyn Fn(usize) + Send + Sync>>` | `None` | Fires after each turn with the new 0-based index. |
| `empty_fallback` | `Option<ChildrenFn>` | `None` | Rendered when `items` is empty. |
| `children` | `Option<Children>` | `None` | Rendered *below* the page slot. |

`T: Clone + Send + Sync + 'static`.

### Notes that bite people

- **`items` is a `Signal`, not a `Vec`.** For a static list, wrap it: `Signal::derive(|| vec![…])`.
- **`render` runs once per visible page**, not once per item. Keep it cheap; it re-runs on every turn.
- **`children` render below the page**, inside the same focusable `.folio` container — that's why `<FolioNav/>` as a child can read the context.
- Only **horizontal** intent pages by trackpad wheel; vertical scroll is left alone (`touch-action: pan-y`).

---

## Built-in navigation

### FolioNav

Prev button, `current / total` counter, next button. Auto-disables at the ends. Must be inside a `<Folio>`.

```rust
<FolioNav/>                                   // ← / →
<FolioNav prev_label="Back".to_string()
          next_label="Next".to_string()/>
```

| Prop | Type | Default |
|------|------|---------|
| `prev_label` | `String` | `"←"` |
| `next_label` | `String` | `"→"` |

### FolioTabs

A tab strip. It is **decoupled from context on purpose** — you supply `active` and `on_select`, so it works with a Folio *or* with any other indexed state.

| Prop | Type |
|------|------|
| `tabs` | `Vec<&'static str>` |
| `active` | `ReadSignal<usize>` |
| `on_select` | `Arc<dyn Fn(usize) + Send + Sync>` |

To bind it to a Folio, pull `current_page` and `go_to` out of the context:

```rust
#[component]
fn BookTabs() -> impl IntoView {
    let ctx = use_folio_context();
    let go_to = ctx.go_to.clone();
    view! {
        <FolioTabs
            tabs=vec!["Overview", "Details", "Pricing"]
            active=ctx.current_page
            on_select=std::sync::Arc::new(move |i| go_to(i))
        />
    }
}
```

---

## Driving navigation yourself

`use_folio_context()` returns a `FolioContext`. Call it from any descendant of `<Folio>` (it panics otherwise — that panic is your signal that the component isn't nested correctly).

```rust
#[derive(Clone)]
pub struct FolioContext {
    pub current_page: ReadSignal<usize>,
    pub total_pages:  Signal<usize>,
    pub go_next: Arc<dyn Fn() + Send + Sync + 'static>,
    pub go_prev: Arc<dyn Fn() + Send + Sync + 'static>,
    pub go_to:   Arc<dyn Fn(usize) + Send + Sync + 'static>,
    pub anim_epoch: ReadSignal<u64>,
    pub last_dir:   ReadSignal<Option<TurnDir>>,
}

pub enum TurnDir { Forward, Backward }
```

**You navigate by calling closures**, and **read state from signals**:

```rust
let ctx = use_folio_context();

// clone the closures you need into handlers
let go_next = ctx.go_next.clone();
let go_prev = ctx.go_prev.clone();

let at_start = move || ctx.current_page.get() == 0;
let at_end   = move || ctx.current_page.get() + 1 >= ctx.total_pages.get();

view! {
    <button on:click=move |_| go_prev() disabled=at_start>"Prev"</button>
    <span>{move || format!("{} / {}", ctx.current_page.get() + 1, ctx.total_pages.get())}</span>
    <button on:click=move |_| go_next() disabled=at_end>"Next"</button>
}
```

### Reacting to turns

`anim_epoch` ticks on every turn; `last_dir` tells you which way. Use either to trigger side effects or custom animations:

```rust
let ctx = use_folio_context();
Effect::new(move |_| {
    let page = ctx.current_page.get();          // re-runs on every turn
    leptos::logging::log!("now on page {page}");
});
```

### Why closures instead of a `turn(dir)` method?

The Folio clamps and computes direction internally (e.g. `go_to` figures out whether it's going forward or backward for the animation). Exposing pre-bound closures keeps that logic in one place and keeps call sites trivial: `go_next()`, `go_to(3)`.

---

## Gestures

### What Folio handles automatically

| Input | Result |
|-------|--------|
| Swipe / drag / horizontal-scroll **right** | `Forward` (next) |
| Swipe / drag / horizontal-scroll **left** | `Backward` (prev) |
| `ArrowRight` / `ArrowDown` / `PageDown` / `Space` | next |
| `ArrowLeft` / `ArrowUp` / `PageUp` | prev |

A turn only fires once travel exceeds `threshold` (default 60px). The trackpad path debounces so one flick = one page.

### Rolling your own with `resolve`

```rust
use leptosbook::{resolve, SwipeDir};

// dx, dy are end-minus-start displacements.
match resolve(dx, dy, 60.0) {
    Some(SwipeDir::Left)  => { /* … */ }
    Some(SwipeDir::Right) => { /* … */ }
    Some(SwipeDir::Up)    => { /* … */ }
    Some(SwipeDir::Down)  => { /* … */ }
    None                  => { /* under threshold */ }
}
```

`resolve` picks the dominant axis, then checks it against the threshold. Horizontal wins ties.

### `SwipeConfig`

```rust
use leptosbook::SwipeConfig;

let cfg = SwipeConfig::default()  // threshold 60, keyboard + mouse on
    .threshold(100.0)
    .no_keyboard()
    .no_mouse();
```

It's a builder for **your own** handlers. It is **not yet** consumed by `<Folio>` — Folio currently exposes only the `threshold` prop and always listens for touch/mouse/wheel/keyboard. Wiring `SwipeConfig` into Folio is on the roadmap.

---

## The install prompt

```rust
<InstallPrompt/>
```

- On **iOS Safari**: shows "Add to Home Screen" instructions (since iOS has no programmatic install).
- On **Chrome/Edge/Android**: captures `beforeinstallprompt` and shows an "Install" button that triggers the native prompt.
- Stays silent if already running standalone, or if previously dismissed (remembered via `localStorage["pwa_dismissed"]`).

| Prop | Type | Default |
|------|------|---------|
| `title` | `String` | `"Install this app"` |
| `description` | `String` | `"Add it to your home screen for the best experience."` |
| `inject_css` | `bool` | `true` |

```rust
<InstallPrompt
    title="Install Acme".to_string()
    description="Get the full-screen, offline-ready experience.".to_string()
/>
```

`title` and `description` apply to the Chrome/Android prompt. On iOS the prompt
shows the standard "Add to Home Screen" share-sheet instructions instead.

---

## Styling

`FOLIO_CSS` is injected by default. To take over completely:

```rust
<Folio inject_css=false items=items render=render>
    <style>{leptosbook::FOLIO_CSS}</style>   // start from the defaults…
    // …then your overrides, or omit FOLIO_CSS entirely for a clean slate
    <FolioNav/>
</Folio>
```

### Class reference

| Class | Element |
|-------|---------|
| `.folio` | Outer focusable container |
| `.folio-page-slot` | Clipping viewport for the page |
| `.folio-page` | The current page (absolutely positioned) |
| `.folio-enter-left` / `.folio-enter-right` | Slide-in animation by direction |
| `.folio-empty` | Centering wrapper for `empty_fallback` |
| `.folio-nav` | `FolioNav` container |
| `.folio-nav-btn` / `:disabled` | Nav buttons |
| `.folio-nav-counter` | The `n / total` text |
| `.folio-tabs` | `FolioTabs` container |
| `.folio-tab` / `.folio-tab.active` | Individual tab |
| `.install-prompt`, `.install-prompt-*` | From `INSTALL_CSS` |

The animation direction is chosen from `last_dir`: forward turns slide `folio-enter-left`, backward turns slide `folio-enter-right`.

---

## Recipes

### Static list

```rust
let items = Signal::derive(|| vec![/* … */]);
```

### Data from a resource

Feed any `Signal<Vec<T>>` — including one derived from a Leptos resource — into `items`, and provide an `empty_fallback` for the loading/empty state:

```rust
<Folio
    items=Signal::derive(move || data.get().unwrap_or_default())
    render=render
    empty_fallback=|| view! { <p>"Loading…"</p> }.into()
/>
```

### Progress dots from context

```rust
#[component]
fn Dots() -> impl IntoView {
    let ctx = use_folio_context();
    view! {
        <div class="dots">
            {move || (0..ctx.total_pages.get()).map(|i| {
                let cur = ctx.current_page.get();
                view! { <span class:on=move || i == cur>"•"</span> }
            }).collect_view()}
        </div>
    }
}
```

---

## Gotchas

- **`use_folio_context()` outside a `<Folio>` panics.** Keep nav components as descendants.
- **`items` must be a `Signal`.** A bare `Vec` won't type-check.
- **`render` must be `Clone + Send + Sync`.** Avoid capturing non-`Send` state; clone what you need in first.
- **`FolioTabs` won't follow the page on its own** — wire `active`/`on_select` to the context as shown above.
- **`SwipeConfig` doesn't affect `<Folio>` yet.** Use the `threshold` prop for now.

---

For runnable code, see [`examples/`](examples/). For task-focused walkthroughs, see the [cookbook](https://andygauge.github.io/leptosbook).
