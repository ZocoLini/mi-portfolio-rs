use yew::{function_component, html, use_effect_with, Html, Properties};

use crate::backend;

#[derive(Properties, PartialEq)]
pub struct ViewProps {
    pub file: String,
}

#[function_component(View)]
pub fn view(params: &ViewProps) -> Html {
    let file = params.file.clone();
    use_effect_with((), move |_| {
        backend::register_content_view(&format!("doc-{}", file));
    });
    html! {
        <iframe
                src={format!("static/docs/{}", params.file)}
                width="98%"
                height="100%"
                title={format!("Document: {}", params.file)}
            />
    }
}