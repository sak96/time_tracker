use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/public/glue.js")]
extern "C" {
    #[wasm_bindgen(js_name = notify, catch)]
    pub fn notify(msg: String) -> Result<JsValue, JsValue>;
}


