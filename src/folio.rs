use std::sync::Arc;

use leptos::ev;
use leptos::prelude::*;
use leptos_meta::Style;

use crate::{
    context::{FolioContext, TurnDir},
    gesture::{resolve, SwipeDir},
    FOLIO_CSS,
};

#[component]
pub fn Folio<T, IV>(
    /// Ordered list of items to page through.
    items: Signal<Vec<T>>,
    /// Renders a single item — only called for the visible page.
    render: impl Fn(T) -> IV + Clone + Send + Sync + 'static,
    /// Swipe pixel threshold (default 60).
    #[prop(default = 60.0)]
    threshold: f64,
    /// Inject the built-in CSS (default true).
    #[prop(default = true)]
    inject_css: bool,
    /// Called after each page turn with the new zero-based index.
    #[prop(optional)]
    on_page_change: Option<Arc<dyn Fn(usize) + Send + Sync + 'static>>,
    /// Children rendered inside the folio container (e.g. `<FolioNav/>`).
    #[prop(optional)]
    children: Option<Children>,
    /// View to show when the items list is empty.
    #[prop(optional)]
    empty_fallback: Option<ChildrenFn>,
) -> impl IntoView
where
    T: Clone + Send + Sync + 'static,
    IV: IntoView + 'static,
{
    // ── Page state ────────────────────────────────────────────────────────
    let (current_page, set_current_page) = signal(0usize);
    let (anim_epoch, set_anim_epoch) = signal(0u64);
    let (last_dir, set_last_dir) = signal(Option::<TurnDir>::None);

    let total_pages = Signal::derive(move || items.get().len());

    // Drag-start positions — use plain signal() pairs so .set()/.get_untracked()
    // are unambiguously on WriteSignal<T> / ReadSignal<T> (no storage generic).
    let (touch_start, set_touch_start) = signal(Option::<(f64, f64)>::None);
    let (mouse_start, set_mouse_start) = signal(Option::<(f64, f64)>::None);
    let (wheel_acc, set_wheel_acc) = signal(0.0f64);
    let (wheel_locked, set_wheel_locked) = signal(false);

    // ── Turn ──────────────────────────────────────────────────────────────
    // Wrap in Arc so we can clone into several event-handler closures.
    let do_turn: Arc<dyn Fn(SwipeDir) + Send + Sync> = Arc::new(move |dir: SwipeDir| {
        let total = items.get().len();
        let cur = current_page.get_untracked();
        let (next, tdir) = match dir {
            SwipeDir::Right | SwipeDir::Down => {
                ((cur + 1).min(total.saturating_sub(1)), TurnDir::Forward)
            }
            SwipeDir::Left | SwipeDir::Up => (cur.saturating_sub(1), TurnDir::Backward),
        };
        if next == cur {
            return;
        }
        set_last_dir.set(Some(tdir));
        set_anim_epoch.update(|e| *e += 1);
        set_current_page.set(next);
        if let Some(ref cb) = on_page_change {
            cb(next);
        }
    });

    // Context-exposed navigation helpers
    let go_next: Arc<dyn Fn() + Send + Sync> = {
        let dt = do_turn.clone();
        Arc::new(move || dt(SwipeDir::Right))
    };
    let go_prev: Arc<dyn Fn() + Send + Sync> = {
        let dt = do_turn.clone();
        Arc::new(move || dt(SwipeDir::Left))
    };
    let go_to: Arc<dyn Fn(usize) + Send + Sync> = {
        let dt = do_turn.clone();
        Arc::new(move |idx: usize| {
            let total = items.get().len();
            let cur = current_page.get_untracked();
            let next = idx.min(total.saturating_sub(1));
            if next == cur {
                return;
            }
            dt(if next > cur {
                SwipeDir::Right
            } else {
                SwipeDir::Left
            });
        })
    };

    provide_context(FolioContext {
        current_page,
        total_pages,
        go_next: go_next.clone(),
        go_prev: go_prev.clone(),
        go_to,
        anim_epoch,
        last_dir,
    });

    // Clone do_turn for each event handler
    let dt_te = do_turn.clone();
    let dt_mu = do_turn.clone();
    let dt_key = do_turn.clone();
    let dt_wheel = do_turn.clone();

    let render = StoredValue::new(render);

    view! {
        {inject_css.then(|| view! { <Style>{FOLIO_CSS}</Style> })}

        <div
            class="folio"
            tabindex="0"

            // Touch
            on:touchstart=move |e: ev::TouchEvent| {
                if let Some(t) = e.touches().item(0) {
                    set_touch_start.set(Some((t.client_x() as f64, t.client_y() as f64)));
                }
            }
            on:touchend=move |e: ev::TouchEvent| {
                if let (Some((sx, sy)), Some(t)) = (touch_start.get_untracked(), e.changed_touches().item(0)) {
                    set_touch_start.set(None);
                    if let Some(dir) = resolve(t.client_x() as f64 - sx, t.client_y() as f64 - sy, threshold) {
                        e.prevent_default();
                        dt_te(dir);
                    }
                }
            }

            // Mouse drag
            on:mousedown=move |e: ev::MouseEvent| {
                set_mouse_start.set(Some((e.client_x() as f64, e.client_y() as f64)));
            }
            on:mouseup=move |e: ev::MouseEvent| {
                if let Some((sx, sy)) = mouse_start.get_untracked() {
                    set_mouse_start.set(None);
                    if let Some(dir) = resolve(e.client_x() as f64 - sx, e.client_y() as f64 - sy, threshold) {
                        e.prevent_default();
                        dt_mu(dir);
                    }
                }
            }

            // Trackpad / horizontal wheel
            on:wheel=move |e: ev::WheelEvent| {
                let dx = e.delta_x();
                let dy = e.delta_y();
                if dx.abs() <= dy.abs() { return; }
                e.prevent_default();
                if wheel_locked.get_untracked() { return; }
                let acc = wheel_acc.get_untracked() + dx;
                if acc.abs() >= threshold {
                    set_wheel_locked.set(true);
                    set_wheel_acc.set(0.0);
                    dt_wheel(if acc < 0.0 { SwipeDir::Left } else { SwipeDir::Right });
                    #[cfg(target_arch = "wasm32")]
                    {
                        use wasm_bindgen::prelude::*;
                        use wasm_bindgen::JsCast as _;
                        let cb = Closure::once(move || set_wheel_locked.set(false));
                        let _ = web_sys::window().and_then(|w| {
                            w.set_timeout_with_callback_and_timeout_and_arguments_0(
                                cb.as_ref().unchecked_ref(), 600,
                            ).ok()
                        });
                        cb.forget();
                    }
                } else {
                    set_wheel_acc.set(acc);
                }
            }

            // Keyboard
            on:keydown=move |e: ev::KeyboardEvent| {
                let dir = match e.key().as_str() {
                    "ArrowRight" | "ArrowDown" | "PageDown" | " " => Some(SwipeDir::Right),
                    "ArrowLeft"  | "ArrowUp"   | "PageUp"         => Some(SwipeDir::Left),
                    _ => None,
                };
                if let Some(d) = dir { dt_key(d); }
            }
        >
            // Page slot — wrapped so children (footer/nav) sit below, not under
            <div class="folio-page-slot">
                {move || {
                    let epoch    = anim_epoch.get();
                    let anim_cls = match last_dir.get() {
                        None                    => "folio-enter-right",
                        Some(TurnDir::Forward)  => "folio-enter-left",
                        Some(TurnDir::Backward) => "folio-enter-right",
                    };
                    let items_now = items.get();
                    let page = current_page.get().min(items_now.len().saturating_sub(1));

                    match items_now.into_iter().nth(page) {
                        Some(item) => view! {
                            <div class=format!("folio-page {anim_cls}") data-epoch=epoch>
                                {render.get_value()(item)}
                            </div>
                        }.into_any(),
                        None => match empty_fallback {
                            Some(ref fb) => view! {
                                <div class="folio-page folio-empty">{fb()}</div>
                            }.into_any(),
                            None => view! { <div class="folio-page folio-empty"/> }.into_any(),
                        },
                    }
                }}
            </div>

            {children.map(|c| c())}
        </div>
    }
}

// ─── FolioNav ──────────────────────────────────────────────────────────────

#[component]
pub fn FolioNav(
    #[prop(default = "←".to_string())] prev_label: String,
    #[prop(default = "→".to_string())] next_label: String,
) -> impl IntoView {
    let ctx = crate::context::use_folio_context();
    let prev_fn = ctx.go_prev.clone();
    let next_fn = ctx.go_next.clone();
    let at_start = move || ctx.current_page.get() == 0;
    let at_end = move || ctx.current_page.get() + 1 >= ctx.total_pages.get();
    let page_label = move || {
        let t = ctx.total_pages.get();
        if t == 0 {
            "—".to_string()
        } else {
            format!("{} / {}", ctx.current_page.get() + 1, t)
        }
    };

    view! {
        <nav class="folio-nav">
            <button
                class="folio-nav-btn"
                disabled=at_start
                on:click=move |_| prev_fn()
            >
                {prev_label}
            </button>
            <span class="folio-nav-counter">{page_label}</span>
            <button
                class="folio-nav-btn"
                disabled=at_end
                on:click=move |_| next_fn()
            >
                {next_label}
            </button>
        </nav>
    }
}

// ─── FolioTabs ─────────────────────────────────────────────────────────────

#[component]
pub fn FolioTabs(
    tabs: Vec<&'static str>,
    active: ReadSignal<usize>,
    on_select: Arc<dyn Fn(usize) + Send + Sync + 'static>,
) -> impl IntoView {
    let tabs = StoredValue::new(tabs);
    view! {
        <nav class="folio-tabs">
            {move || {
                let cb = on_select.clone();
                tabs.get_value().into_iter().enumerate().map(move |(i, label)| {
                    let cb = cb.clone();
                    view! {
                        <button
                            class=move || if active.get() == i { "folio-tab active" } else { "folio-tab" }
                            on:click=move |_| cb(i)
                        >
                            {label}
                        </button>
                    }
                }).collect_view()
            }}
        </nav>
    }
}
