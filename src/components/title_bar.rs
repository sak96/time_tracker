use stylist::yew::use_style;
use wasm_bindgen::prelude::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlMetaElement;
use yew::prelude::*;

#[function_component(ThemeButton)]
fn theme_button() -> Html {
    let is_dark = use_state(|| {
        let mut is_dark = false;
        if let Some(window) = web_sys::window() {
            if let Ok(Some(media)) = window.match_media("(prefers-color-scheme: dark)") {
                is_dark = media.matches();
            }
        }
        is_dark
    });
    let (data_theme, btn_text) = if *is_dark {
        ("dark", "🔆")
    } else {
        ("light", "🌙")
    };
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Some(meta) = document
                .get_elements_by_tag_name("meta")
                .named_item("color-scheme")
            {
                if let Ok(color_scheme) = meta.dyn_into::<HtmlMetaElement>() {
                    color_scheme.set_content(data_theme);
                }
            }
            if let Some(body) = document.body() {
                body.set_attribute("data-theme", data_theme).unwrap();
            }
        }
    }

    let onclick = {
        let is_dark = is_dark;
        Callback::from(move |_| is_dark.set(!*is_dark))
    };
    html! {
        <button {onclick} ~innerText={btn_text} />
    }
}

macro_rules! on_click_window_callback{ ($func: ident)  =>
    {Callback::from(|_| {
        spawn_local(async move {
            $crate::tauri::window::$func().await;

        });
    })}

}

#[function_component(TitleBar)]
pub fn title_bar() -> Html {
    let style = use_style!(
        r#"
        display: flex;
        flex-flow: row;
        align-items: center;
        vertical-align: baseline;
        h1 {
            flex: 1;
        }
        * {
           font-size: 1.5rem;
           width: 2em;
           margin: 0.5rem;
        }
        "#
    );
    html! {
        <div class={style}>
            <h1 ~innerText="Pomodoro Timer" />
            <ThemeButton />
            <button onclick={on_click_window_callback!(minimize)} style="color: orange;" ~innerText="🗕" />
            <button onclick={on_click_window_callback!(toggleMaximize)} style="color: green;" ~innerText="🗖 " />
            <button onclick={on_click_window_callback!(close)} style="color: red;" ~innerText="❌" />
        </div>
    }
}
