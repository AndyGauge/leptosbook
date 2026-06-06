//! The smallest useful Folio: a list of cards you can page through with
//! swipe, arrow keys, trackpad, or the prev/next buttons.
//!
//! Run with `trunk serve --open` from this directory.

use leptos::mount::mount_to_body;
use leptos::prelude::*;
use leptoskit::prelude::*;

#[derive(Clone)]
struct Card {
    title: &'static str,
    body: &'static str,
}

#[component]
fn App() -> impl IntoView {
    // `Folio` takes a `Signal<Vec<T>>`. For a static list, derive one.
    let cards = Signal::derive(|| {
        vec![
            Card {
                title: "One",
                body: "Swipe, drag, or press → to advance.",
            },
            Card {
                title: "Two",
                body: "Press ← (or swipe back) to return.",
            },
            Card {
                title: "Three",
                body: "The counter below tracks your place.",
            },
            Card {
                title: "Four",
                body: "That's the whole API surface for paging.",
            },
        ]
    });

    view! {
        <main style="height:100vh;display:flex;flex-direction:column;font-family:system-ui">
            <Folio
                items=cards
                render=|c: Card| view! {
                    <section style="display:flex;flex-direction:column;align-items:center;\
                                    justify-content:center;height:100%;padding:2rem;text-align:center">
                        <h1 style="font-size:3rem;margin:0 0 1rem">{c.title}</h1>
                        <p style="font-size:1.25rem;opacity:0.8;max-width:24rem">{c.body}</p>
                    </section>
                }
            >
                <footer style="display:flex;justify-content:center;padding:1rem">
                    <FolioNav/>
                </footer>
            </Folio>
        </main>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> });
}
