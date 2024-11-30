// TODO: Make the social links autogenerables
// TODO: Make the contact Info autogenerables
// TODO: Make the works autogenerables

use gloo_net::http::Request;
use serde::Deserialize;
use yew::platform::spawn_local;
use yew::{html, use_effect_with, Html, Properties, UseStateHandle};

pub trait DynGenerable: Clone + Properties + PartialEq where Self: 'static {
    type Data;

    fn r#type(&self) -> String;
    fn resouce_id(&self) -> String;

    fn generate_dyn_html(
        &self,
        state: UseStateHandle<Option<Self::Data>>
    ) -> Html
    where
        for<'a> <Self as DynGenerable>::Data: Deserialize<'a>
    {
        let state_moved = state.clone();
        
        let a = {
            let data: Self = self.clone();
            spawn_local(async move {
                state_moved.set(Some(data.data().await));
                drop(data)
            });
        };

        use_effect_with((), move |_| a);

        if let Some(data) = &*state {
            self.html_with_data(data)
        } else {
            self.html_without_data()
        }
    }

    fn html_with_data(&self, data: &Self::Data) -> Html;
    fn html_without_data(&self) -> Html
    {
        html! {}
    }
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
