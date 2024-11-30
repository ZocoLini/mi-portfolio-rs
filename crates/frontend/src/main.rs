use crate::ui_structure::home;
use yew::platform::spawn_local;
use yew::{function_component, html, use_state, Html};

mod components;
mod dyn_data_gen;
mod lang;
mod styles;
mod ui_structure;

#[function_component(App)]
fn app() -> Html {
    let state = use_state(|| false);

    {
        let state = state.clone();
        spawn_local(async move {
            lang::load_translations().await;
            state.set(true);
        });
    }
    
    html! {
        if *state {
            <home::View />
        } else {
            <components::loading::LoadingSpinner/>
        }
    }
}

fn main() {
    spawn_local(async {
        yew::Renderer::<App>::new().render();
    });
}
