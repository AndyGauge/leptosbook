# Getting started

This recipe builds the smallest complete leptosbook app: a swipeable set of cards
with prev/next buttons.

## 1. Dependencies

```toml
[dependencies]
leptos = { version = "0.9.0-alpha", features = ["csr"] }
leptosbook = "0.1"
console_error_panic_hook = "0.1"
```

## 2. The app

```rust
use leptos::mount::mount_to_body;
use leptos::prelude::*;
use leptosbook::prelude::*;

#[derive(Clone)]
struct Card { title: &'static str, body: &'static str }

#[component]
fn App() -> impl IntoView {
    // Folio wants a `Signal<Vec<T>>`. For a static list, derive one.
    let cards = Signal::derive(|| vec![
        Card { title: "One",   body: "Swipe, drag, or press → to advance." },
        Card { title: "Two",   body: "Press ← (or swipe back) to return." },
        Card { title: "Three", body: "The counter tracks your place." },
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

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> });
}
```

## 3. Run it

leptosbook examples use [Trunk](https://trunkrs.dev):

```bash
trunk serve --open
```

You need an `index.html` next to `Cargo.toml`:

```html
<!DOCTYPE html>
<html>
  <head><meta charset="utf-8" /><link data-trunk rel="rust" /></head>
  <body></body>
</html>
```

That's it. You now have swipe, trackpad, mouse-drag, and arrow-key navigation
for free.

## What just happened

- `items` is a **signal**, so the folio re-renders if the list changes.
- `render` is called **only for the visible card**, once per turn.
- `<FolioNav/>` is a **descendant**, so it reads the navigation state from
  context automatically — no props to thread through.
