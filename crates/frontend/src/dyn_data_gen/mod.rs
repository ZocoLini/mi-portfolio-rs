// TODO: Make the social links autogenerables
// TODO: Make the contact Info autogenerables
// TODO: Make the skills panes autogenerables
// TODO: Make the works autogenerables

use gloo_net::http::Request;
use serde::Deserialize;
use yew::platform::spawn_local;
use yew::{html, use_effect_with, Html, UseStateHandle};

pub trait DynGenerable {
    type Data;

    fn r#type(&self) -> String;
    fn resouce_id(&self) -> String;

    fn generate_dyn_html(&self, state: UseStateHandle<Option<Self::Data>>) -> Html {
        
        {
            let data = async move { self.data().await };
            let state = state.clone();
            use_effect_with((), move |_| {
                spawn_local(async move {
                    state.set(Some(data.await));
                });
                || ()
            });
        }

        if let Some(data) = &*state {
            self.html_with_data(&data)
        } else {
            html! {}
        }
    }

    fn html_with_data(&self, data: &Self::Data) -> Html;
    
    async fn data(&self) -> Self::Data
    where
        for<'a> Self::Data: Deserialize<'a>,
    {
        let fetched_data = Request::get(&format!(
            "resources/dyn_data/{}/{}.json",
            self.r#type(),
            self.resouce_id()
        )) // Ruta relativa al directorio dist
        .send()
        .await
        .expect("Failed to fetch JSON");

        serde_json::from_value::<Self::Data>(
            fetched_data.json().await.expect("Failed to parse JSON"),
        )
        .expect("Failed to deserialize DynGenerable object")
    }
}
