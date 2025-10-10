use crate::ui_structure::{home, work};
use yew::platform::spawn_local;
use yew::{function_component, html, use_state, Html};
use yew_router::prelude::*;

mod components;
mod data_gen;
mod lang;
mod resources;
mod styles;
mod ui_structure;

// TODO: Stop passing Strings to Properties and use Yew Attr instead
// TODO: Remove the clones used in several places to improve memory usage and performance
// TODO: Dont load translations if they are already loaded

#[derive(PartialEq, Clone, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[at("/work/:id")]
    Work { id: String },
    #[at("/doc/:file")]
    Doc { file: String },
}

fn switch(route: Route) -> Html {
    match route {
        Route::Home => {
            html! { <home::View /> }
        }
        Route::Work { id } => {
            html! { <work::View work_id={ id } /> }
        }
        Route::Doc { file } => {
            html! {
                <iframe
                        src={format!("static/docs/{}", file)}
                        width="98%"
                        height="100%"
                        title={format!("Document: {}", file)}
                    />
            }
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

            lang::load_local_translations().await;
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

fn main() 
{
    spawn_local(async {
        yew::Renderer::<App>::new().render();
    });
}
