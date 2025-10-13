use yew::{Html, function_component, html};

use crate::lang;

#[function_component(View)]
pub fn view() -> Html {
    html! {
        <div>
            <h1>{ lang::translate("%general.work-in-progress") }</h1>
        </div>
    }
}
