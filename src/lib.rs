//! # leptoskit
//!
//! SvelteKit-inspired gesture and book-navigation primitives for Leptos.
//!
//! ## Quick start
//!
//! ```rust,ignore
//! use leptoskit::prelude::*;
//!
//! #[component]
//! fn App() -> impl IntoView {
//!     let posts: Signal<Vec<Post>> = ...;
//!
//!     view! {
//!         <Folio items=posts render=|p| view! { <PostCard post=p/> }>
//!             // Anything here can call use_folio_context()
//!             <FolioNav/>
//!         </Folio>
//!     }
//! }
//! ```

pub mod context;
pub mod folio;
pub mod gesture;
pub mod install;

pub use context::{use_folio_context, FolioContext, TurnDir};
pub use folio::{Folio, FolioNav, FolioTabs};
pub use gesture::{resolve, SwipeConfig, SwipeDir};
pub use install::InstallPrompt;

pub mod prelude {
    pub use crate::context::{use_folio_context, FolioContext, TurnDir};
    pub use crate::folio::{Folio, FolioNav, FolioTabs};
    pub use crate::gesture::{resolve, SwipeConfig, SwipeDir};
    pub use crate::install::InstallPrompt;
}

/// Default stylesheet.  Inject with `<Style>{leptoskit::FOLIO_CSS}</Style>`
/// or let `<Folio inject_css=true/>` (the default) handle it automatically.
pub const FOLIO_CSS: &str = r#"
.folio {
  display: flex;
  flex-direction: column;
  width: 100%;
  flex: 1;
  min-height: 0;
  overflow: hidden;
  outline: none;
  touch-action: pan-y;
  user-select: none;
}

.folio-page-slot {
  flex: 1;
  position: relative;
  overflow: hidden;
  min-height: 0;
}

.folio-page {
  position: absolute;
  inset: 0;
}

/* ── Entry animations ─────────────────────────────────────────── */

.folio-enter-left {
  animation: folio-enter-left 0.28s cubic-bezier(.4,0,.2,1) both;
}
.folio-enter-right {
  animation: folio-enter-right 0.28s cubic-bezier(.4,0,.2,1) both;
}

@keyframes folio-enter-left {
  from { opacity: 0; transform: translateX(48px);  }
  to   { opacity: 1; transform: translateX(0); }
}
@keyframes folio-enter-right {
  from { opacity: 0; transform: translateX(-48px); }
  to   { opacity: 1; transform: translateX(0); }
}

/* ── Nav bar ──────────────────────────────────────────────────── */

.folio-nav {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.folio-nav-btn {
  background: none;
  border: 1px solid currentColor;
  color: inherit;
  padding: 0.4rem 1rem;
  font-family: inherit;
  font-size: 0.9rem;
  cursor: pointer;
  border-radius: 2px;
  opacity: 0.7;
  transition: opacity 0.15s;
}
.folio-nav-btn:hover:not(:disabled) { opacity: 1; }
.folio-nav-btn:disabled { opacity: 0.2; cursor: default; }

.folio-nav-counter {
  font-size: 0.8rem;
  font-style: italic;
  min-width: 4rem;
  text-align: center;
  opacity: 0.6;
}

/* ── Tab bar ──────────────────────────────────────────────────── */

.folio-tabs {
  display: flex;
  gap: 1.25rem;
}

.folio-tab {
  background: none;
  border: none;
  border-bottom: 2px solid transparent;
  font-family: inherit;
  font-size: 0.85rem;
  color: inherit;
  opacity: 0.5;
  cursor: pointer;
  padding: 0.2rem 0;
  transition: opacity 0.2s, border-color 0.2s;
  letter-spacing: 0.05em;
}
.folio-tab.active {
  opacity: 1;
  border-bottom-color: currentColor;
}

/* ── Empty page ───────────────────────────────────────────────── */

.folio-empty {
  display: flex;
  align-items: center;
  justify-content: center;
}
"#;
