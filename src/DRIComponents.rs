use chrono::{Datelike, Duration, NaiveDate, Utc};
use leptos::*;

const WEEK_MILLISECONDS: Duration = chrono::Duration::milliseconds(7 * 24 * 60 * 60 * 1000);

const FRY: &'static str = "Phillip J. Fry";
#[component]
pub fn PreviousDri(team: Memo<Vec<String>>) -> impl IntoView {
    let prev_week_number = move || (Utc::now() - WEEK_MILLISECONDS).iso_week().week() as usize;
    let previous_dri = move || {
        if team.get().len() >= 2 {
            team.get()[prev_week_number() % team.get().len()].clone()
        } else {
            FRY.to_string()
        }
    };
    view! {
        <p
        class="previous text-4xl text-slate-50/[.35] relative whitespace-nowrap"
        >
        <span
            class="bg-slate-500/[.55] rounded-lg inline-block px-1 py-0.5 text-lg font-black"
            style="position: absolute; left: -2.8rem; top: .35rem;"
        >
            {prev_week_number}
        </span>
        "Last week was " { previous_dri }
        </p>
    }
}

const LEELA: &'static str = "Turanga Leela";

#[component]
pub fn CurrentDri(team: Memo<Vec<String>>) -> impl IntoView {
    let week_number = move || (Utc::now()).iso_week().week() as usize;
    let current_dri = move || {
        if team.get().len() >= 2 {
            team.get()[week_number() % team.get().len()].clone()
        } else {
            LEELA.to_string()
        }
    };
    view! {
              <p
                class="logo-subtext italic my-1 relative whitespace-nowrap"
              >
                <span
                    class="bg-slate-500 text-slate-50 rounded-lg inline-block px-1 py-0.5 text-2xl font-black"
                    style="position: absolute; left: -3rem; top: 1.3rem; -webkit-background-clip: none; -webkit-text-fill-color: initial;"
                >
                    {week_number}
                </span>
                "Now is " { current_dri }
              </p>
    }
}

const ZOIDBERG: &'static str = "Dr. John A. Zoidberg";

#[component]
pub fn UpcomingDri(team: Memo<Vec<String>>) -> impl IntoView {
    let upcoming_week_number = move || (Utc::now() + WEEK_MILLISECONDS).iso_week().week() as usize;
    let upcoming_dri = move || {
        if team.get().len() >= 2 {
            team.get()[upcoming_week_number() % team.get().len()].clone()
        } else {
            ZOIDBERG.to_string()
        }
    };
    view! {
        <p class="logo-subtext opacity-60 upcoming text-4xl text-slate-50/[.55] relative whitespace-nowrap">
            <span
                class="bg-slate-500/[.55] rounded-lg inline-block px-1 py-0.5 text-lg font-black"
                style="position: absolute; left: -2.8rem; top: .35rem; -webkit-background-clip: none; -webkit-text-fill-color: initial;"
            >
                {upcoming_week_number}
            </span>
            "Next week is " { upcoming_dri }
        </p>
    }
}

#[component]
pub fn DRIHero(names: ReadSignal<String>) -> impl IntoView {
    let year = move || Utc::now().year();
    let week_number = move || Utc::now().iso_week().week() as usize;
    let last_day = move || NaiveDate::from_ymd_opt(year(), 12, 31).unwrap();
    let weeks_this_year = move || last_day().iso_week().week();
    let team = create_memo(move |_| {
        names
            .get()
            .trim()
            .split("\n")
            .map(String::from)
            .collect::<Vec<String>>()
    });

    view! {
      <div class="primary-info-callout mb-12 text-slate-50 text-opacity-90 text-left">
        <span class="uppercase text-slate-50/80 text-1xl mb-2 inline-block">
            "Week "
            <span class="font-bold">{ format!("{}/{}", week_number(), weeks_this_year()) }</span>
            " of "
            <span class="font-bold">{ year() }</span>
        </span>
            <div class="flex flex-col justify-start text-[3rem] ml-12">
                 <PreviousDri team=team />
                 <CurrentDri team=team />
                 <UpcomingDri team=team />
            </div>
      </div>
    }
}
