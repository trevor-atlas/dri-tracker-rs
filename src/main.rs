use leptos::*;
use leptos_meta::*;
use leptos_router::*;
mod app;
mod dri_components;
mod logo;
use app::App;

#[component]
pub fn Root() -> impl IntoView {
    provide_meta_context();
    view! {
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <Routes>
                <Route path="/dri-tracker-rs" view=  move || view! { <App/> }/>
            </Routes>
        </Router>
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
