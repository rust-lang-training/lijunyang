extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use web_sys::js_sys::Function;
use web_sys::window;
#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(msg: &str) {
    return alert(&format!("Hello, {}!", msg));
}

#[wasm_bindgen]
pub fn sum(a: i32, b: i32) -> i32 {
    return a + b;
}

#[wasm_bindgen]
pub fn test_setTimeout() {
    let func = Function::new_no_args(r#"alert("hello wasm)"#);
    let window = window().unwrap();
    window.set_timeout_with_callback_and_timeout_and_arguments_0(&func, 1000);
}

#[wasm_bindgen]
pub fn dom() {
    let window = window().unwrap();
    let doc = window.document().unwrap();
    let test_node = doc.get_element_by_id("test").unwrap();
    test_node.set_text_content(Some("rust 操作 dom"));
    window.alert_with_message("我是 通过 web_sys 生成的");
}
