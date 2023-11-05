use base64::{engine::general_purpose, Engine};
use leptos::*;
use percent_encoding::{percent_decode, utf8_percent_encode, NON_ALPHANUMERIC};
use rand::seq::SliceRandom;
use wasm_bindgen::JsValue;
use web_sys::Window;

use crate::dri_components::DRIHero;
use crate::logo::Logo;

fn win() -> Window {
    web_sys::window().expect("no global `window` exists")
}

fn update_url(text: &String, window: &Window) {
    let data: Vec<String> = text.split("\n").map(String::from).collect();
    let params = encode_url_string(&data);
    let history = window.history().expect("should have a history object");
    let location = window.location();
    let new_url = format!(
        "{}?{}",
        location.pathname().expect("should have a pathname"),
        params
    );
    history
        .push_state_with_url(&JsValue::NULL, "", Some(&new_url))
        .expect("failed to push state");
}

fn decode_url_string(input: &str) -> Vec<String> {
    let decoded = percent_decode(input.as_bytes()).decode_utf8_lossy();
    let decoded_base64 = general_purpose::STANDARD
        .decode(decoded.to_string())
        .expect("Failed to decode base64");
    String::from_utf8(decoded_base64)
        .expect("Failed to convert to String")
        .split("\n")
        .map(String::from)
        .collect()
}

fn encode_url_string(data: &[String]) -> String {
    let joined = data.join("\n");
    let b64 = general_purpose::STANDARD.encode(joined.as_bytes());
    utf8_percent_encode(&b64, NON_ALPHANUMERIC).to_string()
}

#[component]
pub fn App() -> impl IntoView {
    let location = win().location();

    let (names, set_names) = create_signal("".to_string());

    if let Ok(initial_data) = location.search() {
        if !initial_data.is_empty() {
            let decoded_str = decode_url_string(&initial_data[1..]);
            logging::log!("decoded params: {}", decoded_str.join("\n"));
            set_names.set(decoded_str.join("\n"));
        }
    }

    let team = create_memo(move |_| {
        names
            .get()
            .trim()
            .split("\n")
            .map(String::from)
            .collect::<Vec<String>>()
    });

    let is_shuffle_disabled = move || team.get().len() < 2;

    view! {
        <div id="root">
        <Logo/>
            <DRIHero team=team />
            <div class="bg-zinc-800/70 rounded-2xl p-6 mb-8 text-lg text-gray-400" style="border: 1px solid rgba(255,255,255, .1);">
                <div class="mb-1 text-left ">
                    <h2 class="font-bold text-3xl">How to</h2>
                    <ol class="mb-4 space-y-1 list-decimal list-inside ">
                        <li>"Enter a name for each person on a new line."</li>
                        <li>"Share the url to share the list (everything is encoded)"</li>
                    </ol>
                    <p>"The DRI automatically rotates each week."</p>
                </div>
                <textarea
                    placeholder="Enter at least 2 people to get started."
                    class="my-4 font-mono rounded-md p-4"
                    style="width: 100%; height: 200px"
                    aria-expanded="true"
                    prop:value=names
                    on:input= move |ev| {
                        set_names.set(event_target_value(&ev).trim_start().to_string());
                        update_url(&event_target_value(&ev), &win());
                    }
                />
            <div class="text-left">
                <button
                    class="mr-2"
                    on:click=move |_| {
                        set_names.set("".to_string());
                        update_url(&"".to_string(), &win());
                    }
                >"Clear"</button>
                <button
                    prop:disabled=is_shuffle_disabled
                    prop:aria-disabled=is_shuffle_disabled
                    on:click= move |_| {
                        let mut data: Vec<String> = names.get()
                            .split("\n")
                            .map(String::from)
                            .collect();
                        let mut updated_names = data.join("\n");
                        while names.get() == updated_names {
                            data.shuffle(&mut rand::thread_rng());
                            updated_names = data.join("\n");
                        }
                        set_names.set(updated_names.clone());
                        update_url(&updated_names, &win());
                    }>"Randomize order"</button>
            </div>
            </div>
        </div>
    }
}
