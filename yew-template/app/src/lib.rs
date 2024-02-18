mod app;
mod utils;

use app::App;
use log::Level;
use utils::get_app_root;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn main() {
    console_log::init_with_level(Level::Info).unwrap();
    console_error_panic_hook::set_once();

    let root = get_app_root("app").unwrap();
    let app = yew::Renderer::<App>::with_root(root);
    app.render();
}
