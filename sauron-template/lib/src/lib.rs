use app::App;
use log::Level;
use sauron::document;
use sauron::wasm_bindgen;
use sauron::Program;

mod app;
mod model;
mod msg;

#[wasm_bindgen]
pub fn main(id: &str) {
    console_log::init_with_level(Level::Info).unwrap();
    console_error_panic_hook::set_once();

    let target = document().get_element_by_id(id).unwrap();
    Program::replace_mount(App::new(), &target);
}
