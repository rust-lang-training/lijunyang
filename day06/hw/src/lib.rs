use comrak::{markdown_to_html, Options as ComrakOptions};
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
pub fn add(left: usize, right: usize) -> usize {
    left + right
}
#[wasm_bindgen]
extern "C" {}

#[wasm_bindgen(js_name = "markdown2html")]
pub fn markdown_2_html(markdown: &str) -> String {
    let options = ComrakOptions::default();
    markdown_to_html(markdown, &options)
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
