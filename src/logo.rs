use leptos::*;

#[component]
pub fn Logo() -> impl IntoView {
    view! {
        <div class="text-left mb-12">
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
