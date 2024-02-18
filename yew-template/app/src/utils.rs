use web_sys::Element;

pub fn get_app_root(id: &str) -> Option<Element> {
    web_sys::window()
        .and_then(|window| window.document())
        .and_then(|document| document.get_element_by_id(id))
}
