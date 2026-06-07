# Changelog

All notable changes to this project are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-06-06

First public release, extracted from a real-world Leptos app.

### Added

- **`<Folio>`** — paginated container with touch, mouse-drag, trackpad, and
  keyboard navigation; direction-aware slide animations; SSR and hydrate support
  via feature flags. Props: `items`, `render`, `threshold`, `inject_css`,
  `on_page_change`, `empty_fallback`, `children`.
- **`<FolioNav>`** — prev/next buttons with a `current / total` counter and
  auto-disable at the ends (`prev_label`, `next_label`).
- **`<FolioTabs>`** — tab strip driven by `active` + `on_select`, so it composes
  with a folio (via context) or any other indexed state.
- **`use_folio_context()` / `FolioContext`** — read navigation state
  (`current_page`, `total_pages`, `anim_epoch`, `last_dir`) and drive it
  (`go_next`, `go_prev`, `go_to`) from any descendant.
- **`TurnDir`** — `Forward` / `Backward`, reported via `last_dir`.
- **Gesture toolkit** — `resolve()`, `SwipeDir`, and a `SwipeConfig` builder for
  rolling your own swipeable surfaces.
- **`<InstallPrompt>`** — PWA "Add to Home Screen" prompt for iOS and
  Chrome/Android, with configurable `title`/`description` and dismissal
  remembered in `localStorage`.
- **`FOLIO_CSS` / `INSTALL_CSS`** — default stylesheets, injected automatically
  (toggle with `inject_css`).

### Documentation

- README with a quick start and a full prop reference.
- GUIDE.md with patterns, recipes, and gotchas.
- mdBook cookbook (image carousel, onboarding flow, custom navigation, tabs &
  dots, gestures, styling), deployed to GitHub Pages.
- Runnable examples: `basic-folio`, `gesture-demo`, `onboarding-tour`.

### Known limitations

- `SwipeConfig` is not yet consumed by `<Folio>` — Folio honors only `threshold`.

## Roadmap

- Wire `SwipeConfig` (keyboard/mouse toggles) into `<Folio>`.
- Carousel / wrap-around mode.
- Virtualization for very large lists.

[Unreleased]: https://github.com/AndyGauge/leptosbook/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/AndyGauge/leptosbook/releases/tag/v0.1.0
