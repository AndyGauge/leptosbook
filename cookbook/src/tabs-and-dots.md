# Tabs and progress dots

leptoskit ships `FolioTabs` for a labeled tab strip, and it's easy to hand-roll
progress dots. Both follow the same idea: read `current_page` from context, and
jump with `go_to`.

## FolioTabs, wired to a folio

`FolioTabs` is intentionally **decoupled** from context — you pass it `active`
and `on_select`. That makes it reusable outside a folio, but inside one you wire
it up in three lines:

```rust
use std::sync::Arc;
use leptos::prelude::*;
use leptoskit::prelude::*;

#[component]
fn SectionTabs() -> impl IntoView {
    let ctx = use_folio_context();
    let go_to = ctx.go_to.clone();
    let on_select: Arc<dyn Fn(usize) + Send + Sync> = Arc::new(move |i| go_to(i));

    view! {
        <FolioTabs
            tabs=vec!["Overview", "Details", "Pricing"]
            active=ctx.current_page
            on_select=on_select
        />
    }
}
```

`tabs` is a `Vec<&'static str>`, so the labels are fixed at compile time — ideal
for a known set of sections.

## Hand-rolled progress dots

When you just want minimal dots that scale to any length, render straight from
the context:

```rust
#[component]
fn Dots() -> impl IntoView {
    let ctx = use_folio_context();
    let go_to = ctx.go_to.clone();

    view! {
        <div class="dots">
            {move || {
                let cur = ctx.current_page.get();
                let go_to = go_to.clone();
                (0..ctx.total_pages.get()).map(move |i| {
                    let go_to = go_to.clone();
                    view! {
                        <button
                            class:on=move || i == cur
                            on:click=move |_| go_to(i)
                            aria-label=format!("Go to item {}", i + 1)
                        />
                    }
                }).collect_view()
            }}
        </div>
    }
}
```

```css
.dots { display: flex; gap: .5rem; justify-content: center; padding: 1rem; }
.dots button {
  width: .6rem; height: .6rem; border-radius: 50%;
  border: none; background: rgba(255,255,255,.3); cursor: pointer;
  transition: background .2s, transform .2s;
}
.dots button.on { background: white; transform: scale(1.3); }
```

## Which to use?

- **`FolioTabs`** — a few, named sections (Overview / Details / Pricing).
- **Hand-rolled dots** — many, unnamed items (a 12-photo gallery), or when you
  want full control over the markup and ARIA.
