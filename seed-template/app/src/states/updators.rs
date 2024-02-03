use gloo_console::log;
use seed::app::Orders;
use wasm_bindgen::JsValue;

use super::messages::Msg;
use super::models::Model;

pub fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::DoSomething => {
            model.count += 1;

            let model_jsvalue = JsValue::from_str(&format!("{model:?}"));
            log!(model_jsvalue);
        }
    }
}
