use leptos::prelude::*;
use std::sync::Arc;

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
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TurnDir {
    Forward,
    Backward,
}

/// Call from any component nested inside `<Folio>` to read or drive navigation.
pub fn use_folio_context() -> FolioContext {
    use_context::<FolioContext>()
        .expect("`use_folio_context` must be called inside a `<Folio>` component")
}
