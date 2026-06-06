# Working with gestures

`Folio` handles touch, mouse-drag, trackpad, and keyboard out of the box. This
recipe covers tuning that behavior and reusing the recognizer on your own
elements.

## What Folio recognizes

| Input | Result |
|-------|--------|
| Swipe / drag / scroll **right** | next |
| Swipe / drag / scroll **left** | prev |
| `ArrowRight` / `ArrowDown` / `PageDown` / `Space` | next |
| `ArrowLeft` / `ArrowUp` / `PageUp` | prev |

## Tuning sensitivity

A turn fires once pointer travel exceeds `threshold` (px). Raise it to require a
more deliberate swipe; lower it for a hair-trigger:

```rust
<Folio items=items render=render threshold=120.0>
    <FolioNav/>
</Folio>
```

## Reusing the recognizer

The gesture math is exposed as a free function so you can build your own
swipeable surfaces — a dismissible card, a drawer, a rating slider:

```rust
use leptos::ev;
use leptos::prelude::*;
use leptosbook::{resolve, SwipeDir};

#[component]
fn SwipeToDismiss(on_dismiss: impl Fn() + 'static) -> impl IntoView {
    let start = RwSignal::new(Option::<(f64, f64)>::None);

    let down = move |e: ev::MouseEvent|
        start.set(Some((e.client_x() as f64, e.client_y() as f64)));

    let up = move |e: ev::MouseEvent| {
        if let Some((sx, sy)) = start.get() {
            start.set(None);
            let (dx, dy) = (e.client_x() as f64 - sx, e.client_y() as f64 - sy);
            if let Some(SwipeDir::Left | SwipeDir::Right) = resolve(dx, dy, 80.0) {
                on_dismiss();
            }
        }
    };

    view! { <div on:mousedown=down on:mouseup=up>"swipe me away"</div> }
}
```

`resolve(dx, dy, threshold)` returns the dominant `SwipeDir` once it clears the
threshold, or `None` if the movement was too small. Horizontal wins ties.

## A note on `SwipeConfig`

`SwipeConfig` (`threshold` / `keyboard` / `mouse`) is a builder for **your own**
handlers:

```rust
use leptosbook::SwipeConfig;
let cfg = SwipeConfig::default().threshold(100.0).no_keyboard();
```

It is **not yet** consumed by `<Folio>` — Folio currently takes only the
`threshold` prop and always listens for all input types. Wiring `SwipeConfig`
into Folio (to, say, disable keyboard nav) is on the roadmap.
