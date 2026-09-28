use leptos::prelude::*;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

/// Injected by `<Folio>` — accessible from any descendant via `use_folio_context()`.
#[derive(Clone)]
pub struct FolioContext {
    pub current_page: ReadSignal<usize>,
    pub total_pages: Signal<usize>,
    /// Navigate forward one page (no-op when already at the last page).
    pub go_next: Arc<dyn Fn() + Send + Sync + 'static>,
    /// Navigate back one page (no-op when already at the first page).
    pub go_prev: Arc<dyn Fn() + Send + Sync + 'static>,
    /// Jump to an arbitrary page (clamped to valid range).
    pub go_to: Arc<dyn Fn(usize) + Send + Sync + 'static>,
    /// Increments on every page turn; you can bind CSS animation classes to it.
    pub anim_epoch: ReadSignal<u64>,
    /// Direction of the most recent page turn (None before the first navigation).
    pub last_dir: ReadSignal<Option<TurnDir>>,
    /// True while any [`use_folio_lock`] holds the folio. Page turns of every
    /// kind (keys, swipes, wheel, `go_next`/`go_prev`/`go_to`) are ignored.
    pub locked: Signal<bool>,
    /// Number of active locks; managed by [`use_folio_lock`].
    pub(crate) lock_count: RwSignal<u32>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TurnDir {
    Forward,
    Backward,
}

/// Hold the surrounding `<Folio>` on its current page while `when` is true —
/// e.g. while a form on the page is open, so a stray key or swipe can't turn
/// the page and throw away what the user is typing.
///
/// Locks nest: the folio stays locked while *any* holder's `when` is true. A
/// held lock is released automatically when the calling component unmounts.
///
/// ```ignore
/// let (editing, set_editing) = signal(false);
/// use_folio_lock(editing);
/// ```
///
/// Must be called inside a `<Folio>` (typically from a component rendered by
/// its `render` closure).
pub fn use_folio_lock(when: impl Into<Signal<bool>>) {
    let count = use_folio_context().lock_count;
    let when = when.into();
    // Whether this call site currently holds one of the folio's locks.
    let held = Arc::new(AtomicBool::new(false));

    let h = held.clone();
    Effect::new(move |_| {
        let want = when.get();
        if h.swap(want, Ordering::SeqCst) != want {
            count.try_update(|c| *c = if want { *c + 1 } else { c.saturating_sub(1) });
        }
    });

    on_cleanup(move || {
        if held.swap(false, Ordering::SeqCst) {
            // The folio may be unmounting too; its signal may already be gone.
            count.try_update(|c| *c = c.saturating_sub(1));
        }
    });
}

/// Call from any component nested inside `<Folio>` to read or drive navigation.
pub fn use_folio_context() -> FolioContext {
    use_context::<FolioContext>()
        .expect("`use_folio_context` must be called inside a `<Folio>` component")
}
