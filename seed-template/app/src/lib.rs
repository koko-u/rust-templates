use seed::prelude::*;
use states::updators;

mod init;
mod states;
mod views;

#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init::init, updators::update, views::view);
}
