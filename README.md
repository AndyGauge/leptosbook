# leptoskit

**SvelteKit-inspired UI primitives for Leptos** — bringing joy and ergonomics to Leptos development.

Gesture-driven book navigation, context-driven state, and a PWA install prompt. All type-safe, all Rust. Built for Leptos `0.9`.

## Features

- **`<Folio>`** — Page through a list with swipe, trackpad, mouse drag, and keyboard
- **`<FolioNav>`** — Prev/next buttons with a page counter and auto-disable at the ends
- **`<FolioTabs>`** — A tab bar that jumps to any page
- **`use_folio_context()`** — Drive or read navigation from any descendant component
- **`<InstallPrompt>`** — "Add to Home Screen" prompt for iOS and Chrome/Android
- **Gesture toolkit** — `resolve()`, `SwipeDir`, `SwipeConfig` for rolling your own
- **Animated transitions** — Slide-in animations keyed to turn direction

## Quick Start

```toml
[dependencies]
leptos = { version = "0.9.0-alpha", features = ["csr"] }
leptoskit = "0.1"
```

```rust
use leptos::prelude::*;
use leptoskit::prelude::*;

#[derive(Clone)]
struct Card { title: &'static str, body: &'static str }

#[component]
fn App() -> impl IntoView {
    // Folio takes a `Signal<Vec<T>>`. For a static list, derive one.
    let cards = Signal::derive(|| vec![
        Card { title: "One", body: "Swipe or press → to advance." },
        Card { title: "Two", body: "Press ← to go back." },
    ]);

    view! {
        <Folio
            items=cards
            render=|c: Card| view! {
                <section><h1>{c.title}</h1><p>{c.body}</p></section>
            }
        >
            <FolioNav/>
        </Folio>
    }
}
```

Swipe left/right, press arrow keys, scroll horizontally on a trackpad, or click the buttons. It all just works.

## Components

### `<Folio>`

The paginated container. Renders one item at a time and owns all navigation state.

| Prop | Type | Default | Notes |
|------|------|---------|-------|
| `items` | `Signal<Vec<T>>` | — | Items to page through |
| `render` | `impl Fn(T) -> impl IntoView + Clone + Send + Sync` | — | Renders the visible item |
| `threshold` | `f64` | `60.0` | Min swipe distance in px |
| `inject_css` | `bool` | `true` | Inject the built-in stylesheet |
| `on_page_change` | `Option<Arc<dyn Fn(usize) + Send + Sync>>` | `None` | Called after each turn with the new index |
| `empty_fallback` | `Option<ChildrenFn>` | `None` | Shown when `items` is empty |
| `children` | `Option<Children>` | `None` | Rendered below the page (e.g. `<FolioNav/>`) |

`T` must be `Clone + Send + Sync + 'static`.

### `<FolioNav>`

Prev/next buttons plus a `current / total` counter. Must be a descendant of `<Folio>`.

| Prop | Type | Default |
|------|------|---------|
| `prev_label` | `String` | `"←"` |
| `next_label` | `String` | `"→"` |

```rust
<FolioNav prev_label="‹ before".to_string() next_label="after ›".to_string()/>
```

### `<FolioTabs>`

A tab bar. Unlike `FolioNav`, it does **not** read context implicitly — you pass it the active index and a select callback, so you can wire it to a `Folio` (via `use_folio_context()`) or to your own state.

| Prop | Type |
|------|------|
| `tabs` | `Vec<&'static str>` |
| `active` | `ReadSignal<usize>` |
| `on_select` | `Arc<dyn Fn(usize) + Send + Sync>` |

```rust
let ctx = use_folio_context();
let go_to = ctx.go_to.clone();
view! {
    <FolioTabs
        tabs=vec!["Morning", "Evening"]
        active=ctx.current_page
        on_select=Arc::new(move |i| go_to(i))
    />
}
```

### `<InstallPrompt>`

A PWA "install" / "Add to Home Screen" prompt. Shows iOS instructions on iOS, listens for `beforeinstallprompt` on Chrome/Android, and stays silent once installed or dismissed (remembered in `localStorage`).

| Prop | Type | Default |
|------|------|---------|
| `title` | `String` | `"Install this app"` |
| `description` | `String` | `"Add it to your home screen for the best experience."` |
| `inject_css` | `bool` | `true` |

`title`/`description` apply to the Chrome/Android prompt; iOS shows the standard "Add to Home Screen" instructions.

## Driving navigation: `use_folio_context()`

Any descendant of `<Folio>` can call `use_folio_context()` to get a `FolioContext`:

```rust
pub struct FolioContext {
    pub current_page: ReadSignal<usize>,        // reactive current index
    pub total_pages:  Signal<usize>,            // reactive length
    pub go_next: Arc<dyn Fn() + Send + Sync>,   // forward one page (clamped)
    pub go_prev: Arc<dyn Fn() + Send + Sync>,   // back one page (clamped)
    pub go_to:   Arc<dyn Fn(usize) + Send + Sync>, // jump to an index (clamped)
    pub anim_epoch: ReadSignal<u64>,            // increments every turn
    pub last_dir:   ReadSignal<Option<TurnDir>>,// Forward / Backward
}
```

You navigate by **calling the closures**, and read state from the signals:

```rust
let ctx = use_folio_context();
let go_next = ctx.go_next.clone();

view! {
    <button on:click=move |_| go_next() disabled=move || {
        ctx.current_page.get() + 1 >= ctx.total_pages.get()
    }>
        "Next"
    </button>
    <span>{move || format!("{} / {}", ctx.current_page.get() + 1, ctx.total_pages.get())}</span>
}
```

`TurnDir` has exactly two variants — `Forward` and `Backward` — and is reported via `last_dir` so you can react to (or animate) the direction of the most recent turn.

## Gestures

`<Folio>` recognizes these out of the box:

| Input | Result |
|-------|--------|
| Swipe / drag right, scroll right | Next page |
| Swipe / drag left, scroll left | Previous page |
| `ArrowRight` / `ArrowDown` / `PageDown` / `Space` | Next page |
| `ArrowLeft` / `ArrowUp` / `PageUp` | Previous page |

To build your own recognizer, use the gesture module directly:

```rust
use leptoskit::{resolve, SwipeDir};

match resolve(dx, dy, 60.0) {
    Some(SwipeDir::Right) => { /* … */ }
    Some(SwipeDir::Left)  => { /* … */ }
    Some(SwipeDir::Up) | Some(SwipeDir::Down) => { /* … */ }
    None => { /* below threshold */ }
}
```

`SwipeConfig` (`threshold`, `keyboard`, `mouse`) exists as a builder for your own handlers. Note it is **not yet** wired into `<Folio>` — Folio currently takes only `threshold`. See [Roadmap](#roadmap).

## Styling

Default styles ship in the `FOLIO_CSS` constant and are injected automatically. Disable that and bring your own:

```rust
<Folio inject_css=false items=cards render=render>
    <style>{leptoskit::FOLIO_CSS}</style>  // or your own
    <FolioNav/>
</Folio>
```

Key classes: `.folio`, `.folio-page-slot`, `.folio-page`, `.folio-enter-left`, `.folio-enter-right`, `.folio-nav`, `.folio-nav-btn`, `.folio-nav-counter`, `.folio-tabs`, `.folio-tab`, `.folio-tab.active`. The install prompt ships its own `INSTALL_CSS`.

## Examples

Each is a standalone Trunk app under [`examples/`](examples/):

| Example | Shows |
|---------|-------|
| `basic-folio` | The minimal Folio + FolioNav |
| `gesture-demo` | `resolve()` live, plus a custom nav bar from `FolioContext` |
| `onboarding-tour` | `FolioTabs` wired to context + footer nav — the real-world shape |

```bash
cd examples/basic-folio
trunk serve --open
```

## Documentation

- [Full Guide](GUIDE.md) — every component, prop, and pattern
- [Cookbook](https://andygauge.github.io/leptoskit) — task-focused recipes
- [API docs](https://docs.rs/leptoskit) — rustdoc

## Roadmap

- Wire `SwipeConfig` (keyboard/mouse toggles) into `<Folio>`
- Carousel/wrap-around mode
- Virtualization for very large lists

See [CHANGELOG.md](CHANGELOG.md) for release history.

## License

Dual-licensed under [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE). Take your pick.

---

Built for developers who think UI should be a joy, not a chore.
