# An onboarding flow

A first-launch "welcome tour" with a few steps, dots that track progress, and a
footer that turns into a **Get started** button on the last step.

```rust
use std::sync::Arc;
use leptos::prelude::*;
use leptoskit::prelude::*;

#[derive(Clone)]
struct Step { icon: &'static str, title: &'static str, blurb: &'static str }

const STEPS: &[Step] = &[
    Step { icon: "👋", title: "Welcome",   blurb: "A 20-second tour." },
    Step { icon: "⚡", title: "Fast",      blurb: "Keyboard and gestures everywhere." },
    Step { icon: "🚀", title: "You're set", blurb: "Swipe back any time." },
];

#[component]
fn Onboarding(on_finish: Arc<dyn Fn() + Send + Sync>) -> impl IntoView {
    let steps = Signal::derive(|| STEPS.to_vec());
    view! {
        <Folio
            items=steps
            render=|s: Step| view! {
                <section class="step">
                    <div class="step-icon">{s.icon}</div>
                    <h1>{s.title}</h1>
                    <p>{s.blurb}</p>
                </section>
            }
        >
            <Dots/>
            <Footer on_finish=on_finish/>
        </Folio>
    }
}
```

The footer reads `FolioContext` to know whether it's on the last step:

```rust
#[component]
fn Footer(on_finish: Arc<dyn Fn() + Send + Sync>) -> impl IntoView {
    let ctx = use_folio_context();
    let go_next = ctx.go_next.clone();
    let last = move || ctx.current_page.get() + 1 >= ctx.total_pages.get();

    view! {
        <footer class="onb-footer">
            <Show
                when=last
                fallback=move || {
                    let go_next = go_next.clone();
                    view! { <button on:click=move |_| go_next()>"Next"</button> }
                }
            >
                {
                    let on_finish = on_finish.clone();
                    view! { <button class="primary" on:click=move |_| on_finish()>"Get started"</button> }
                }
            </Show>
        </footer>
    }
}
```

For the `Dots` component, see the next recipe,
[Tabs and progress dots](./tabs-and-dots.md).

## Why pass `on_finish` in?

The folio owns *navigation* state, not *application* state. "Onboarding is
complete" belongs to your app, so hand the component a callback rather than
trying to stuff app logic into the folio.
