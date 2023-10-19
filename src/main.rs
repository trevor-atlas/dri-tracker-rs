use base64::{engine::general_purpose, Engine};
use chrono::{DateTime, Datelike, Duration, NaiveDate, Utc};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use percent_encoding::{percent_decode, utf8_percent_encode, NON_ALPHANUMERIC};
use rand::seq::SliceRandom;
use wasm_bindgen::JsValue;
use web_sys::{Document, History, HtmlInputElement, HtmlTextAreaElement, Location, Window};

const WEEK_MILLISECONDS: Duration = chrono::Duration::milliseconds(7 * 24 * 60 * 60 * 1000);

#[component]
pub fn Root() -> impl IntoView {
    provide_meta_context();
    view! {
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <Routes>
                <Route path="" view=  move || view! { <App/> }/>
            </Routes>
        </Router>
    }
}

#[component]
fn Dri(names: ReadSignal<String>) -> impl IntoView {
    let year = move || Utc::now().year();
    let week_number = move || Utc::now().iso_week().week() as usize;
    let prev_week_number = move || (Utc::now() - WEEK_MILLISECONDS).iso_week().week() as usize;
    let next_week_number = move || (Utc::now() + WEEK_MILLISECONDS).iso_week().week() as usize;
    let last_day = move || NaiveDate::from_ymd_opt(year(), 12, 31).unwrap();
    let weeks_this_year = move || last_day().iso_week().week();
    let team = move || {
        names
            .get()
            .trim()
            .split("\n")
            .map(String::from)
            .collect::<Vec<String>>()
    };

    let previous_dri = move || team()[prev_week_number() % team().len()].clone();
    let current_dri = move || team()[week_number() % team().len()].clone();
    let next_dri = move || team()[next_week_number() % team().len()].clone();
    let should_show = move || {
        logging::log!("team size: {}", team().len());
        return team().len() >= 2;
    };

    view! {
      <div class="primary-info-callout mb-8 text-slate-50 text-opacity-90 text-left">
        <small class="uppercase text-slate-50/80 text-md text-xl mb-2 inline-block">
            "Week "
            <span class="font-bold">{ format!("{}/{}", week_number(), weeks_this_year()) }</span>
            " of "
            <span class="font-bold">{ year() }</span>
        </small>
          <Show
            when=should_show
            fallback=|| view! { <p>"Enter at least 2 people to get started"</p> }
          >
            <div class="flex flex-col justify-start text-6xl">
              <p
                class="previous text-4xl text-slate-50/[.35] relative"
              >
                <span
                    class="bg-slate-500/[.55] rounded-lg inline-block px-1 py-0.5 text-lg font-black"
                    style="position: absolute; left: -2.8rem; top: .35rem;"
                >
                    {prev_week_number}
                </span>
                { format!("Last week was {}", previous_dri()) }</p>
              <p
                class="logo-subtext italic my-2 relative"
              >
                <span
                    class="bg-slate-500 text-slate-50 rounded-lg inline-block px-1 py-0.5 text-2xl font-black"
                    style="position: absolute; left: -3rem; top: .85rem; -webkit-background-clip: none; -webkit-text-fill-color: initial;"
                >
                    {week_number}
                </span>
                { format!("Now is {}", current_dri()) }
              </p>
              <p
                class="logo-subtext opacity-60 upcoming text-4xl text-slate-50/[.55] relative"
              >
                <span
                    class="bg-slate-500/[.55] rounded-lg inline-block px-1 py-0.5 text-lg font-black"
                    style="position: absolute; left: -2.8rem; top: .35rem; -webkit-background-clip: none; -webkit-text-fill-color: initial;"
                >
                    {next_week_number}
                </span>
                { format!("Next week is {}", next_dri()) }</p>
            </div>
          </Show>
      </div>
    }
}

fn win() -> Window {
    web_sys::window().expect("no global `window` exists")
}

#[component]
pub fn Logo() -> impl IntoView {
    view! {
        <div class="text-left flex justify-center bg-gray-800 py-4 mb-8">
            <div class="flex flex-row text-bold">
                <div class="mr-1">
                    <h1 class="text-[4rem] mb-0" style="line-height: 1.15">"🫡"</h1>
                </div>
                <div>
                    <h1 class="logo-title text-4xl text-gray-300 font-black">"DRI Tracker"</h1>
                    <h2 class="logo-subtext ml-1 mb-0 text-2xl text-gray-400 font-normal">"Who is the DRI this week?"</h2>
                </div>
            </div>
        </div>
    }
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

    view! {
            <Logo/>
        <div id="root">
            <Dri names=names />
            <div class="bg-zinc-800/70 rounded-2xl p-6 mb-8 text-lg text-gray-300" style="border: 1px solid rgba(255,255,255, .1);">
            <div class="mb-1 text-left text-gray-300">
                <h3 class="font-bold text-2xl">DRIs</h3>
                <p>"Enter a name for each person on a new line, the previous, current and upcoming DRI automatically rotates each week."</p>
                <p>"Share the url to share the list (everything is encoded)"</p>
            </div>
            <div class="py-4 text-left">
                <button
                    class="mr-2"
                    on:click=move |_| {
                        set_names.set("".to_string());
                        update_url(&"".to_string(), &win());
                    }
                >"Clear"</button>
                <button
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
                <textarea
                    class="font-mono rounded-md p-4"
                    style="width: 100%; height: 200px"
                    aria-expanded="true"
                    prop:value=names
                    on:input= move |ev| {
                        // event_target_value is a Leptos helper function
                        // it functions the same way as event.target.value
                        // in JavaScript, but smooths out some of the typecasting
                        // necessary to make this work in Rust
                        set_names.set(event_target_value(&ev).trim_start().to_string());
                        update_url(&event_target_value(&ev), &win());
                    }
                />
            </div>
        </div>
    }
}

fn main() {
    logging::log!("csr mode - mounting to body");
    mount_to_body(|| {
        view! {
            <Root/>
        }
    })
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
