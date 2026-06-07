# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

`leptosbook` — a standalone, publishable Leptos `0.9` UI library for paginated,
gesture-driven interfaces (carousels, onboarding flows, slide decks, swipeable
dashboards). Extracted from a real app; the canonical API reference is the source
itself. Published to crates.io as `leptosbook`; docs site is the mdBook cookbook
at https://andygauge.github.io/leptosbook/.

## Workspace layout

The repo root is **both** the library package and a Cargo workspace. Members are
the examples (`members = ["examples/*"]`):

- Root package (`src/`) = the `leptosbook` library.
- `examples/{basic-folio,gesture-demo,onboarding-tour}` = standalone **CSR Trunk
  apps**, each its own crate depending on the library by path.

## Commands

```bash
# Library
cargo test -p leptosbook                         # unit tests (gesture math lives here)
cargo test -p leptosbook gesture::tests::ties    # a single test by path
cargo build -p basic-folio -p gesture-demo -p onboarding-tour   # build all examples
cargo fmt --all -- --check                       # CI fmt gate

# Run an example in the browser (needs `trunk` + the wasm32 target)
cd examples/basic-folio && trunk serve --open

# Cookbook (needs `mdbook`)
mdbook build cookbook                             # outputs cookbook/book/ (gitignored)
mdbook serve cookbook --open

# Pre-release sanity
cargo publish -p leptosbook --dry-run
cargo package -p leptosbook --list               # confirm the shipped file set
```

### Clippy — features are mutually exclusive

`ssr` and `hydrate` cannot be enabled together (Leptos rejects it), so **never**
run `clippy --all-features`. Lint each feature set on its own — this is exactly
what CI does:

```bash
cargo clippy -p leptosbook --no-default-features -- -D warnings
cargo clippy -p leptosbook --features ssr -- -D warnings
cargo clippy -p leptosbook --features hydrate -- -D warnings
```

## Architecture — the one idea

Everything centers on the **`Folio`** component (`src/folio.rs`). It renders a
`Signal<Vec<T>>` one item at a time and owns *all* navigation state, which it
publishes through `FolioContext` via `provide_context`. Descendants
(`FolioNav`, your own controls) call `use_folio_context()` to read state and
drive turns — there is no prop-drilling.

- `src/context.rs` — `FolioContext` (the shared state) and `TurnDir`
  (`Forward`/`Backward`). **Navigation is done by calling the `Arc<dyn Fn>`
  closures** on the context (`go_next`, `go_prev`, `go_to`), not by mutating
  signals. State is read from `current_page` / `total_pages` / `last_dir`.
- `src/gesture.rs` — pure recognizer: `resolve(dx, dy, threshold) -> Option<SwipeDir>`
  plus the `SwipeConfig` builder. This is the only logic with real unit tests
  (sign, threshold inclusivity, axis dominance, tie-breaks-horizontal).
- `src/folio.rs` — wires touch/mouse/wheel/keyboard events through `resolve`,
  clamps indices, picks the slide-animation class from `last_dir`.
- `src/install.rs` — `InstallPrompt` PWA component (`title`/`description` props),
  with its own `INSTALL_CSS`.
- `src/lib.rs` — re-exports, the `prelude`, and the `FOLIO_CSS` constant.

### Conventions specific to this crate

- **`FolioNav` reads context implicitly; `FolioTabs` does not.** `FolioTabs`
  takes explicit `tabs`/`active`/`on_select`, so binding it to a folio means
  pulling `current_page` + `go_to` out of the context yourself (see the
  `onboarding-tour` example). Keep this decoupling — it's intentional.
- **`leptos_meta` must stay a non-optional dependency.** `folio.rs` and
  `install.rs` use `leptos_meta::Style` unconditionally; gating it behind a
  feature breaks the default build.
- **CSS is injected via the `*_CSS` constants** behind an `inject_css` prop
  (default `true`). Don't add a build step for styles.
- **Docs must match the real API.** README, GUIDE, and the cookbook describe
  exact signatures; when you change a public signature, update all three. A
  prior pass shipped a fabricated API — verify against `src/` before writing
  examples.
- **Known gap:** `SwipeConfig` is a builder for user code; it is *not* wired into
  `<Folio>` yet (Folio honors only `threshold`). Don't document it as if it is.

## Releasing

Use the **`/release` skill** (`.claude/skills/release/SKILL.md`) — it bumps the
version, rolls `CHANGELOG.md`, runs the full verification suite + `cargo publish
--dry-run`, tags `vX.Y.Z`, and (after confirmation) creates a GitHub Release that
triggers `.github/workflows/publish.yml` to `cargo publish`. Publishing requires
the `CARGO_REGISTRY_TOKEN` repo secret. crates.io versions are immutable — a bad
release means cutting a new patch, never re-tagging.

The published package ships only `src/` + docs + licenses; `examples/`,
`cookbook/`, `.github/`, and `.claude/` are excluded via `Cargo.toml` `exclude`.
