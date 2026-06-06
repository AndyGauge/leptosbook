//! A product onboarding / feature tour — the kind of "welcome carousel" most
//! apps show on first launch. Demonstrates the real-world shape: `FolioTabs`
//! wired to the shared `FolioContext` so the step dots and swipe gestures stay
//! in sync, plus a footer `FolioNav`.
//!
//! Run with `trunk serve --open` from this directory.

use std::sync::Arc;

use leptos::mount::mount_to_body;
use leptos::prelude::*;
use leptoskit::prelude::*;

#[derive(Clone)]
struct Step {
    icon: &'static str,
    title: &'static str,
    blurb: &'static str,
}

const STEPS: &[Step] = &[
    Step {
        icon: "👋",
        title: "Welcome",
        blurb: "Here's a 20-second tour of what you can do.",
    },
    Step {
        icon: "⚡",
        title: "Fast by default",
        blurb: "Everything is keyboard- and gesture-driven.",
    },
    Step {
        icon: "🧩",
        title: "Composable",
        blurb: "Drop pieces in where you need them — no lock-in.",
    },
    Step {
        icon: "🚀",
        title: "You're set",
        blurb: "Swipe back any time. Let's go.",
    },
];

#[component]
fn App() -> impl IntoView {
    let steps = Signal::derive(|| STEPS.to_vec());

    view! {
        <main style="height:100vh;display:flex;flex-direction:column;\
                     font-family:system-ui;background:#0f172a;color:#e2e8f0">
            <Folio
                items=steps
                render=|s: Step| view! {
                    <section style="display:flex;flex-direction:column;align-items:center;\
                                    justify-content:center;height:100%;padding:2rem;text-align:center">
                        <div style="font-size:4rem;margin-bottom:1rem">{s.icon}</div>
                        <h1 style="margin:0 0 0.75rem;font-size:2rem">{s.title}</h1>
                        <p style="font-size:1.2rem;opacity:0.75;max-width:26rem">{s.blurb}</p>
                    </section>
                }
            >
                <StepDots/>
                <footer style="display:flex;justify-content:center;padding:1rem 1rem 1.5rem">
                    <FolioNav prev_label="‹ back".to_string() next_label="next ›".to_string()/>
                </footer>
            </Folio>
        </main>
    }
}

/// Bridges `FolioContext` to `FolioTabs`: the active dot follows the current
/// step, and tapping a dot jumps the tour via `go_to`.
#[component]
fn StepDots() -> impl IntoView {
    let ctx = use_folio_context();
    let go_to = ctx.go_to.clone();
    let on_select: Arc<dyn Fn(usize) + Send + Sync> = Arc::new(move |i| go_to(i));

    view! {
        <div style="display:flex;justify-content:center;padding:1rem">
            <FolioTabs
                tabs=vec!["Welcome", "Speed", "Compose", "Done"]
                active=ctx.current_page
                on_select=on_select
            />
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> });
}
