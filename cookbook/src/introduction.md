# The leptoskit Cookbook

leptoskit gives Leptos a small set of primitives for **paginated, gesture-driven
interfaces** — image carousels, onboarding flows, slide decks, swipeable
dashboards, wizards, anything where the user moves through a sequence one screen
at a time.

This cookbook is task-focused. Each chapter solves one concrete problem with a
complete, copy-pasteable snippet. If you want the full prop-by-prop reference,
read the [Guide](https://github.com/andygauge/leptoskit/blob/main/GUIDE.md)
instead; if you want runnable apps, see the
[`examples/`](https://github.com/andygauge/leptoskit/tree/main/examples) directory.

## The one component you need to know

Everything starts with `Folio`: hand it a `Signal<Vec<T>>` and a `render`
function, and it shows one item at a time with swipe, trackpad, mouse, and
keyboard navigation built in. Descendant components read or drive that
navigation through `use_folio_context()`.

```rust
use leptos::prelude::*;
use leptoskit::prelude::*;

#[component]
fn Carousel(slides: Signal<Vec<String>>) -> impl IntoView {
    view! {
        <Folio items=slides render=|s: String| view! { <p>{s}</p> }>
            <FolioNav/>
        </Folio>
    }
}
```

Targets Leptos `0.9`. Add it with:

```toml
[dependencies]
leptos = { version = "0.9.0-alpha", features = ["csr"] }
leptoskit = "0.1"
```
