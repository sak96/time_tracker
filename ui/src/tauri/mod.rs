use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window.__TAURI__.notification"], js_name = sendNotification, catch)]
    pub fn notify(msg: String) -> Result<JsValue, JsValue>;
}
