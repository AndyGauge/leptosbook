# An image carousel

A full-bleed image carousel with a caption and a counter — the classic gallery
pattern.

```rust
use leptos::prelude::*;
use leptoskit::prelude::*;

#[derive(Clone)]
struct Photo { url: &'static str, caption: &'static str }

#[component]
fn Gallery() -> impl IntoView {
    let photos = Signal::derive(|| vec![
        Photo { url: "/img/1.jpg", caption: "Sunrise over the bay" },
        Photo { url: "/img/2.jpg", caption: "City lights" },
        Photo { url: "/img/3.jpg", caption: "Quiet forest trail" },
    ]);

    view! {
        <div class="gallery">
            <Folio
                items=photos
                render=|p: Photo| view! {
                    <figure class="slide">
                        <img src=p.url alt=p.caption/>
                        <figcaption>{p.caption}</figcaption>
                    </figure>
                }
            >
                <FolioNav/>
            </Folio>
        </div>
    }
}
```

```css
.gallery { height: 70vh; display: flex; flex-direction: column; }
.slide { margin: 0; height: 100%; }
.slide img { width: 100%; height: 100%; object-fit: cover; }
.slide figcaption {
  position: absolute; bottom: 0; left: 0; right: 0;
  padding: 1rem; background: linear-gradient(transparent, rgba(0,0,0,.6));
  color: white;
}
```

## Tips

- Give the carousel a fixed height; the page slot fills its parent.
- `object-fit: cover` keeps images from distorting across aspect ratios.
- Want lazy loading? Add `loading="lazy"` — but remember `render` only runs for
  the visible slide, so off-screen images aren't in the DOM until you reach them.
