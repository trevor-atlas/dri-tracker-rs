use chrono::{Datelike, Duration, NaiveDate, Utc};
use leptos::*;
use wasm_bindgen::prelude::*;

const COMPONENTS: [(&'static str, &'static str); 3] = [
    (
        "Phillip J. Fry 🍕",
        "previous text-4xl text-slate-50/[.35] relative",
    ),
    ("Turanga Leela 👁", "logo-subtext italic my-1 relative"),
    (
        "Dr. John A. Zoidberg 🦀",
        "logo-subtext opacity-60 upcoming text-4xl text-slate-50/[.55] relative",
    ),
];

#[component]
pub fn DRIComponent(team: Memo<Vec<String>>, index: i64) -> impl IntoView {
    let (default_name, css_classes) = COMPONENTS[index as usize];
    let week_number = move || (Utc::now() + Duration::weeks(index - 1)).iso_week().week() as usize;
    let dri = move || {
        if team.get().len() >= 2 {
            team.get()[week_number() % team.get().len()].clone()
        } else {
            default_name.to_string()
        }
    };
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let reload = Closure::<dyn Fn()>::new(move || {
        logging::log!("reloading to update");
        let _ = window.location().reload(); ()
    });

        create_effect(move |_| {
            if index == 1 {
                let window = web_sys::window().expect("no global `window` exists");
                document.set_title(format!("DRI: {}", dri()).as_str());
                let _ = window.set_interval_with_callback_and_timeout_and_arguments_0(reload.as_ref().unchecked_ref(),  1000 * 60 * 60);
            }
        });

    view! {
        <div class="flex flex-row items-center">
            <div
                class="relative bg-slate-500 text-slate-50 rounded-lg inline-block px-1 py-0.5 mr-2"
                style="width: 32px; height: 32px;"
            >
                <span class="absolute text-xl leading-1" style="top: 2px">{week_number}" "</span>
            </div>
                <p class={format!("{} inline whitespace-nowrap", css_classes)}>
                    { dri }
                </p>
        </div>
    }
}

#[component]
pub fn DRIHero(team: Memo<Vec<String>>) -> impl IntoView {
    let year = move || Utc::now().year();
    let week_number = move || Utc::now().iso_week().week() as usize;
    let last_day = move || NaiveDate::from_ymd_opt(year(), 12, 31).unwrap();
    let weeks_this_year = move || last_day().iso_week().week();

    view! {
      <div class="primary-info-callout mb-12 text-slate-50 text-opacity-90 text-left">
        <span class="uppercase text-slate-50/80 text-1xl mb-2 inline-block">
            "Week "
            <span class="font-bold">{ format!("{}", week_number()) }</span>{" / "}<span class="font-bold">{format!("{}", weeks_this_year())}</span>
            " of "
            <span>{ year() }</span>
        </span>
        <div class="flex flex-col justify-start text-[3rem]">
            {(0..3).map(|i| {
                view! {<DRIComponent team=team.clone() index=i />}
            }).collect_view()}
        </div>
      </div>
    }
}
