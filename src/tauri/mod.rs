use gloo_utils::format::JsValueSerdeExt;
use js_sys::{Function, Promise};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

pub mod window;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window.__TAURI__.notification"], js_name = sendNotification, catch)]
    pub fn notify(msg: &str) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_namespace = ["window.__TAURI__.event"], js_name = "listen")]
    fn listen_(event: &str, handler: &Closure<dyn FnMut(JsValue)>) -> Promise;

    #[wasm_bindgen(js_namespace = ["window.__TAURI__.tauri"], js_name = "invoke")]
    async fn invoke_(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

pub async fn invoke<T: Serialize + 'static, O: DeserializeOwned + 'static>(
    cmd: &str,
    args: T,
) -> Result<O, String> {
    let cmd = cmd.to_string();
    let args = JsValue::from_serde::<T>(&args).map_err(|e| e.to_string())?;
    invoke_(&cmd, args)
        .await
        .into_serde()
        .map_err(|e| e.to_string())
}

pub struct EventListener(String, Promise, Closure<dyn FnMut(JsValue)>);

impl Drop for EventListener {
    fn drop(&mut self) {
        let promise = self.1.clone();
        let event_name = self.0.clone();
        spawn_local(async move {
            let cleanup = async move {
                let unlisten: Function = wasm_bindgen_futures::JsFuture::from(promise)
                    .await
                    .map_err(|e| format!("{:?}", e))?
                    .into();
                unlisten
                    .call0(&JsValue::undefined())
                    .map_err(|e| format!("{:?}", e))?;
                Ok::<(), String>(())
            };
            if let Err(e) = cleanup.await {
                log(&format!(
                    "listener for {} cleanup failed with error {}",
                    event_name, e
                ));
            }
        });
    }
}

pub fn listen<T: DeserializeOwned + 'static>(
    event: &str,
    mut handler: Box<dyn FnMut(T)>,
) -> EventListener {
    let event_name = event.to_string();
    let converted_handler = Box::new(move |jsvalue: JsValue| {
        #[derive(Deserialize)]
        struct Payload<T> {
            payload: T,
        }
        match jsvalue.into_serde() {
            Ok(Payload { payload }) => handler(payload),
            Err(e) => log(&format!(
                "listener for {} failed to value parse as {}\n value: {:?}\n error {}",
                event_name,
                std::any::type_name::<Payload<T>>(),
                jsvalue,
                e
            )),
        }
    });
    let handler = Closure::new(converted_handler);
    let promise = listen_(event, &handler);
    EventListener(event.to_string(), promise, handler)
}
