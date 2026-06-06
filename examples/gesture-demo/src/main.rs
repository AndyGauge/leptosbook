//! Two things in one page:
//!
//! 1. A live read-out of the raw gesture recogniser (`resolve`) — drag inside
//!    the pad and watch which `SwipeDir` a displacement resolves to.
//! 2. A `Folio` driven by a *custom* status bar that reads `FolioContext`
//!    (`current_page`, `total_pages`, `last_dir`) and drives it via the
//!    `go_prev` / `go_next` / `go_to` closures.
//!
//! Run with `trunk serve --open` from this directory.

use leptos::ev;
use leptos::mount::mount_to_body;
use leptos::prelude::*;
use leptosbook::prelude::*;

#[component]
fn App() -> impl IntoView {
    let pages = Signal::derive(|| (1..=5).map(|n| format!("Page {n}")).collect::<Vec<_>>());

    view! {
        <main style="height:100vh;display:flex;flex-direction:column;font-family:system-ui;gap:1rem;padding:1rem">
            <RecognizerPad/>
            <div style="flex:1;display:flex;flex-direction:column;border:1px solid #ccc;border-radius:8px;overflow:hidden">
                <Folio
                    items=pages
                    render=|p: String| view! {
                        <div style="display:grid;place-items:center;height:100%;font-size:2.5rem">{p}</div>
                    }
                >
                    <StatusBar/>
                </Folio>
            </div>
        </main>
    }
}

/// Drag inside the pad; on release we feed the (dx, dy) into `resolve`.
#[component]
fn RecognizerPad() -> impl IntoView {
    let start = RwSignal::new(Option::<(f64, f64)>::None);
    let result = RwSignal::new("drag inside the pad →".to_string());

    let on_down =
        move |e: ev::MouseEvent| start.set(Some((e.client_x() as f64, e.client_y() as f64)));
    let on_up = move |e: ev::MouseEvent| {
        if let Some((sx, sy)) = start.get() {
            start.set(None);
            let (dx, dy) = (e.client_x() as f64 - sx, e.client_y() as f64 - sy);
            result.set(match resolve(dx, dy, 60.0) {
                Some(dir) => format!("{dir:?}  (dx={dx:.0}, dy={dy:.0})"),
                None => format!("below threshold  (dx={dx:.0}, dy={dy:.0})"),
            });
        }
    };

    view! {
        <div
            on:mousedown=on_down
            on:mouseup=on_up
            style="height:6rem;border:2px dashed #888;border-radius:8px;display:grid;\
                   place-items:center;user-select:none;cursor:grab;font-family:monospace"
        >
            {move || result.get()}
        </div>
    }
}

/// A bespoke navigation bar built entirely from `FolioContext`.
#[component]
fn StatusBar() -> impl IntoView {
    let ctx = use_folio_context();

    let (go_prev, go_next, go_to) = (ctx.go_prev.clone(), ctx.go_next.clone(), ctx.go_to.clone());
    let at_start = move || ctx.current_page.get() == 0;
    let at_end = move || ctx.current_page.get() + 1 >= ctx.total_pages.get();
    let last_dir = move || match ctx.last_dir.get() {
        None => "—",
        Some(TurnDir::Forward) => "Forward",
        Some(TurnDir::Backward) => "Backward",
    };

    view! {
        <footer style="display:flex;align-items:center;gap:1rem;padding:0.75rem;border-top:1px solid #ccc;font-family:monospace">
            <button on:click=move |_| go_prev() disabled=at_start>"prev"</button>
            <button on:click=move |_| go_next() disabled=at_end>"next"</button>
            <span>{move || format!("page {} / {}", ctx.current_page.get() + 1, ctx.total_pages.get())}</span>
            <span style="opacity:0.6">"last turn: " {last_dir}</span>
            <button style="margin-left:auto" on:click=move |_| go_to(0)>"⇤ first"</button>
        </footer>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> });
}
