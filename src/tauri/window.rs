use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window.__TAURI__.window.appWindow"])]
    pub async fn minimize();

    #[wasm_bindgen(js_namespace = ["window.__TAURI__.window.appWindow"])]
    pub async fn toggleMaximize();

    #[wasm_bindgen(js_namespace = ["window.__TAURI__.window.appWindow"])]
    pub async fn close();
}
