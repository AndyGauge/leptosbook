# Custom navigation from context

`FolioNav` is convenient, but sometimes you want your own controls — a floating
button, a step label, a "skip to end" link. Any descendant of `<Folio>` can call
`use_folio_context()` and drive navigation directly.

## The context

```rust
pub struct FolioContext {
    pub current_page: ReadSignal<usize>,        // reactive index
    pub total_pages:  Signal<usize>,            // reactive length
    pub go_next: Arc<dyn Fn() + Send + Sync>,   // forward one (clamped)
    pub go_prev: Arc<dyn Fn() + Send + Sync>,   // back one (clamped)
    pub go_to:   Arc<dyn Fn(usize) + Send + Sync>, // jump (clamped)
    pub anim_epoch: ReadSignal<u64>,            // ticks each turn
    pub last_dir:   ReadSignal<Option<TurnDir>>,// Forward / Backward
}
```

You **navigate by calling the closures** and **read state from the signals**.

## A bespoke control bar

```rust
use leptos::prelude::*;
use leptoskit::prelude::*;

#[component]
fn ControlBar() -> impl IntoView {
    let ctx = use_folio_context();
    let (go_prev, go_next, go_to) =
        (ctx.go_prev.clone(), ctx.go_next.clone(), ctx.go_to.clone());

    let at_start = move || ctx.current_page.get() == 0;
    let at_end   = move || ctx.current_page.get() + 1 >= ctx.total_pages.get();

    view! {
        <div class="control-bar">
            <button on:click=move |_| go_prev() disabled=at_start>"‹"</button>
            <span>{move || format!("{} of {}",
                ctx.current_page.get() + 1, ctx.total_pages.get())}</span>
            <button on:click=move |_| go_next() disabled=at_end>"›"</button>
            <button class="skip" on:click=move |_| {
                let last = ctx.total_pages.get().saturating_sub(1);
                go_to(last);
            }>"Skip to end"</button>
        </div>
    }
}
```

## Reacting to a turn

Use `anim_epoch` or `current_page` in an effect to fire side effects — analytics,
autoplay pausing, lazy fetches:

```rust
let ctx = use_folio_context();
Effect::new(move |_| {
    let page = ctx.current_page.get();   // re-runs on every turn
    track_event("slide_view", page);
});
```

And `last_dir` tells you the direction of the most recent turn:

```rust
let dir = move || match ctx.last_dir.get() {
    Some(TurnDir::Forward)  => "→",
    Some(TurnDir::Backward) => "←",
    None => "·",
};
```

## Clone before you move

Each handler closure that uses a context closure needs its own clone — they're
`Arc`s, so cloning is cheap:

```rust
let go_next = ctx.go_next.clone();          // once per handler
view! { <button on:click=move |_| go_next()>"Next"</button> }
```
