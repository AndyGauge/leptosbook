use leptos::prelude::*;
use leptos_meta::Style;

pub const INSTALL_CSS: &str = r#"
.install-prompt {
  position: fixed;
  bottom: 5.5rem;
  left: 50%;
  transform: translateX(-50%);
  width: min(90vw, 360px);
  background: #1a0f08;
  border: 1px solid rgba(255,255,255,0.15);
  border-radius: 12px;
  padding: 1rem 1rem 1rem 1.25rem;
  display: flex;
  align-items: flex-start;
  gap: 0.75rem;
  box-shadow: 0 4px 24px rgba(0,0,0,0.5);
  z-index: 100;
  animation: install-slide-up 0.3s cubic-bezier(.4,0,.2,1) both;
}

@keyframes install-slide-up {
  from { opacity: 0; transform: translateX(-50%) translateY(10px); }
  to   { opacity: 1; transform: translateX(-50%) translateY(0); }
}

.install-prompt-dismiss {
  position: absolute;
  top: 0.4rem;
  right: 0.5rem;
  background: none;
  border: none;
  color: inherit;
  opacity: 0.45;
  cursor: pointer;
  font-size: 1.1rem;
  line-height: 1;
  padding: 0.25rem 0.4rem;
}
.install-prompt-dismiss:hover { opacity: 0.9; }

.install-prompt-body {
  display: flex;
  align-items: flex-start;
  gap: 0.75rem;
  padding-right: 1.5rem;
}

.install-prompt-icon {
  font-size: 1.4rem;
  flex-shrink: 0;
  margin-top: 0.1rem;
}

.install-prompt-body strong {
  display: block;
  margin-bottom: 0.2rem;
  font-size: 0.9rem;
  color: rgba(255,255,255,0.9);
}

.install-prompt-desc {
  font-size: 0.78rem;
  color: rgba(255,255,255,0.55);
  margin: 0 0 0.65rem;
  line-height: 1.4;
}

.install-prompt-btn {
  background: #8b4513;
  color: #fdf8f0;
  border: none;
  border-radius: 6px;
  padding: 0.35rem 0.9rem;
  font-family: inherit;
  font-size: 0.82rem;
  cursor: pointer;
  letter-spacing: 0.03em;
}
.install-prompt-btn:hover { opacity: 0.85; }
"#;

#[component]
pub fn InstallPrompt(
    /// Heading shown on the Chrome/Android prompt (e.g. your app's name).
    #[prop(default = "Install this app".to_string())]
    title: String,
    /// Supporting line shown beneath the heading on the Chrome/Android prompt.
    #[prop(default = "Add it to your home screen for the best experience.".to_string())]
    description: String,
    #[prop(default = true)] inject_css: bool,
) -> impl IntoView {
    let show = RwSignal::new(false);
    let is_ios = RwSignal::new(false);
    // The view closure re-runs reactively, so stash the strings in StoredValue
    // rather than moving owned `String`s out of an `FnMut`.
    let title = StoredValue::new(title);
    let description = StoredValue::new(description);

    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::prelude::*;
        use wasm_bindgen::JsCast;

        Effect::new(move |_| {
            let Some(window) = web_sys::window() else {
                return;
            };

            // Already running as installed PWA — stay silent
            if let Ok(Some(mq)) = window.match_media("(display-mode: standalone)") {
                if mq.matches() {
                    return;
                }
            }

            // Previously dismissed
            if let Ok(Some(storage)) = window.local_storage() {
                if storage.get_item("pwa_dismissed").ok().flatten().is_some() {
                    return;
                }
            }

            let ua = window.navigator().user_agent().unwrap_or_default();
            let on_ios = ua.contains("iPhone") || ua.contains("iPad") || ua.contains("iPod");

            if on_ios {
                // navigator.standalone is proprietary iOS
                let standalone = js_sys::Reflect::get(
                    &window.navigator(),
                    &wasm_bindgen::JsValue::from_str("standalone"),
                )
                .ok()
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
                if !standalone {
                    is_ios.set(true);
                    show.set(true);
                }
            } else {
                // Chrome/Edge/Android: listen for beforeinstallprompt
                let closure =
                    Closure::<dyn FnMut(web_sys::Event)>::new(move |e: web_sys::Event| {
                        e.prevent_default();
                        // Stash prompt on window for retrieval at click time
                        let _ = js_sys::Reflect::set(
                            &web_sys::window().unwrap(),
                            &wasm_bindgen::JsValue::from_str("__pwa_prompt"),
                            &e,
                        );
                        show.set(true);
                    });
                let _ = window.add_event_listener_with_callback(
                    "beforeinstallprompt",
                    closure.as_ref().unchecked_ref(),
                );
                closure.forget();
            }
        });
    }

    let dismiss = move |_| {
        show.set(false);
        #[cfg(target_arch = "wasm32")]
        if let Some(w) = web_sys::window() {
            if let Ok(Some(s)) = w.local_storage() {
                let _ = s.set_item("pwa_dismissed", "1");
            }
        }
    };

    let install_app = move |_| {
        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;
            if let Some(window) = web_sys::window() {
                let prompt =
                    js_sys::Reflect::get(&window, &wasm_bindgen::JsValue::from_str("__pwa_prompt"))
                        .unwrap_or(wasm_bindgen::JsValue::UNDEFINED);
                if !prompt.is_undefined() {
                    if let Ok(fn_val) =
                        js_sys::Reflect::get(&prompt, &wasm_bindgen::JsValue::from_str("prompt"))
                    {
                        if let Ok(f) = fn_val.dyn_into::<js_sys::Function>() {
                            let _ = f.call0(&prompt);
                        }
                    }
                }
            }
        }
        show.set(false);
    };

    view! {
        {inject_css.then(|| view! { <Style>{INSTALL_CSS}</Style> })}
        <Show when=move || show.get()>
            <div class="install-prompt">
                <button class="install-prompt-dismiss" on:click=dismiss>"×"</button>
                {move || if is_ios.get() {
                    view! {
                        <div class="install-prompt-body">
                            <span class="install-prompt-icon">"⬆"</span>
                            <div>
                                <strong>"Add to Home Screen"</strong>
                                <p class="install-prompt-desc">
                                    "Tap the Share button, then \"Add to Home Screen\"."
                                </p>
                            </div>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div class="install-prompt-body">
                            <div>
                                <strong>{title.get_value()}</strong>
                                <p class="install-prompt-desc">
                                    {description.get_value()}
                                </p>
                                <button class="install-prompt-btn" on:click=install_app>
                                    "Install App"
                                </button>
                            </div>
                        </div>
                    }.into_any()
                }}
            </div>
        </Show>
    }
}
