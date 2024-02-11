use sycamore::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::Element;

#[wasm_bindgen]
pub fn main() {
    let app_root = get_app_root("app").unwrap();

    sycamore::render_to(
        || {
            let state = create_signal(0);

            view! {
                p {
                    "Count:" (state.get())
                }
                button(on:click=move |_| state.update(|count| *count += 1)) {
                    "Click Me!"
                }
            }
        },
        &app_root,
    )
}

fn get_app_root(id: &str) -> Option<Element> {
    let Some(window) = web_sys::window() else {
        return None;
    };
    let Some(document) = window.document() else {
        return None;
    };
    let Some(root) = document.get_element_by_id(id) else {
        return None;
    };

    Some(root)
}
