# Forms inside a page

Pages often hold a small form: a reply box, a rating, a note. Two things keep
the folio from fighting it.

## Form fields keep their input

You don't have to do anything for this part. `<Folio>` ignores input aimed at a
form field:

- keys typed into an `input`, `textarea`, `select` or `contenteditable` element
  (so `Space` and the arrow keys edit text instead of turning the page);
- mouse drags and touch swipes that *start* in one (text selection, not a swipe);
- `Space` on a focused button or link, which activates it;
- any key your own handler already called `prevent_default()` on.

## Lock the page while the form is open

Focus can leave the field (onto a Submit button, say) and a phone swipe can land
outside it. If turning the page would throw away what the user typed, hold a lock
with `use_folio_lock` for as long as the form is open:

```rust
use leptos::prelude::*;
use leptosbook::prelude::*;

#[component]
fn NoteForm() -> impl IntoView {
    let (open, set_open) = signal(false);
    let (text, set_text) = signal(String::new());

    // Locked exactly while `open` is true; released on unmount.
    use_folio_lock(open);

    view! {
        <Show
            when=move || open.get()
            fallback=move || view! {
                <button on:click=move |_| set_open.set(true)>"Add a note"</button>
            }
        >
            <textarea
                prop:value=text
                on:input=move |e| set_text.set(event_target_value(&e))
            />
            <button on:click=move |_| set_open.set(false)>"Done"</button>
        </Show>
    }
}
```

Render it from your `Folio`'s `render` closure; like `use_folio_context`, the
hook must run inside a `<Folio>`.

While any lock is held:

- keys, swipes, drags and the trackpad don't turn the page;
- `go_next`, `go_prev` and `go_to` are no-ops;
- `<FolioNav/>` disables both buttons.

Locks nest, so several forms can each hold one; the folio unlocks when the last
is released. Read `use_folio_context().locked` to disable your own navigation
controls the same way.
