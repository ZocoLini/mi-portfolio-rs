use crate::ui_structure::{home, work};
use crate::Route::{Home, Work};
use yew::platform::spawn_local;
use yew::{function_component, html, use_state, Html};
use yew_router::prelude::*;

mod components;
mod dyn_data_gen;
mod lang;
mod resources;
mod styles;
mod ui_structure;

// TODO: Memory leak somewhere
// TODO: Stop passing Strings to Properties and use Yew Attr instead
// TODO: Remove the clones used in several places to improver memory usage and performance
// TODO: Dont load translations if they are already loaded
// TODO: Review the en_EN translations

#[derive(PartialEq, Clone, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[at("/work/:id")]
    Work { id: String },
}

fn switch(route: Route) -> Html {
    match route {
        Home => {
            html! { <home::View /> }
        }
        Work { id } => {
            html! { <work::View work_id={ id } /> }
        }
    }
}

#[function_component(App)]
fn app() -> Html {
    let translations_loaded = use_state(|| false);

    if !*translations_loaded {
        let state = translations_loaded.clone();
        spawn_local(async move {
            #[cfg(debug_assertions)]
            web_sys::console::log_1(&"Loading translations".into());

            lang::load_default_translations().await;
            state.set(true);
        });
    }
    
    html! {
            if *translations_loaded {
                <BrowserRouter>
                    <Switch<Route> render={switch} />
                </BrowserRouter>
            } else {
                <components::LoadingSpinner/>
            }
        }
}

fn main() {
    spawn_local(async {
        yew::Renderer::<App>::new().render();
    });
}
