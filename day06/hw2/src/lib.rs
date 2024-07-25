use comrak::{markdown_to_html, Options as ComrakOptions};
use wasm_bindgen::prelude::*;
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

fn markdown_2_html(markdown: &str) -> String {
    let options = ComrakOptions::default();
    markdown_to_html(markdown, &options)
}

#[wasm_bindgen]
pub fn handle_convert(event: web_sys::InputEvent) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let mark_text = document.get_element_by_id("mark-text").unwrap();
    let html_text = document.get_element_by_id("html-text").unwrap();
    let mark_text_value = mark_text
        .dyn_into::<web_sys::HtmlTextAreaElement>()
        .unwrap()
        .value();

    let content = markdown_2_html(&mark_text_value);
    html_text
        .dyn_into::<web_sys::HtmlTextAreaElement>()
        .unwrap()
        .set_value(&content);
    web_sys::console::log_1(&JsValue::from_str("convert"));
}
#[wasm_bindgen]
pub fn handle_reset(_event: web_sys::MouseEvent) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let mark_text = document.get_element_by_id("mark-text").unwrap();
    let html_text = document.get_element_by_id("html-text").unwrap();
    mark_text
        .dyn_into::<web_sys::HtmlTextAreaElement>()
        .unwrap()
        .set_value("");
    html_text
        .dyn_into::<web_sys::HtmlTextAreaElement>()
        .unwrap()
        .set_value("");
    web_sys::console::log_1(&JsValue::from_str("reset"));
}
#[wasm_bindgen]
pub fn bind_event() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let convert_btn = document.get_element_by_id("convert-btn").unwrap();
    let reset_btn = document.get_element_by_id("reset-btn").unwrap();
    let on_reset = Closure::wrap(
        Box::new(move |event: web_sys::MouseEvent| handle_reset(event)) as Box<dyn FnMut(_)>,
    );

    let on_convert = Closure::wrap(
        Box::new(move |event: web_sys::InputEvent| handle_convert(event)) as Box<dyn FnMut(_)>,
    );
    reset_btn.add_event_listener_with_callback("click", on_reset.as_ref().unchecked_ref())?;
    convert_btn.add_event_listener_with_callback("click", on_convert.as_ref().unchecked_ref())?;
    on_reset.forget();
    on_convert.forget();
    Ok(())
}
#[wasm_bindgen]
pub fn gen_elements() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let app = document.get_element_by_id("app").unwrap();

    let markdown_container = document.create_element("textarea").unwrap();
    let html_container = document.create_element("textarea").unwrap();
    markdown_container
        .set_attribute("name", "mark-text")
        .unwrap();
    markdown_container.set_attribute("id", "mark-text").unwrap();
    markdown_container
        .set_attribute("placeholder", "请输入markdown文本")
        .unwrap();
    markdown_container.set_attribute("rows", "10").unwrap();
    markdown_container.set_attribute("cols", "150").unwrap();

    html_container.set_attribute("id", "html-text").unwrap();
    html_container.set_attribute("disabled", "true").unwrap();
    html_container.set_attribute("cols", "150").unwrap();
    html_container.set_attribute("rows", "10").unwrap();
    app.append_child(&markdown_container).unwrap();
    app.append_child(&html_container).unwrap();
    let bnt_container = document.create_element("div").unwrap();
    let convert_btn = document.create_element("button").unwrap();
    let reset_btn = document.create_element("button").unwrap();
    convert_btn.set_inner_html("转换");
    reset_btn.set_inner_html("重置");
    convert_btn.set_attribute("id", "convert-btn").unwrap();
    reset_btn.set_attribute("id", "reset-btn").unwrap();
    bnt_container.append_child(&reset_btn).unwrap();
    bnt_container.append_child(&convert_btn).unwrap();
    app.append_child(&bnt_container).unwrap();
    Ok(())
}
#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();
    let app = document.create_element("div").unwrap();
    app.set_attribute("id", "app").unwrap();
    body.append_child(&app).unwrap();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
