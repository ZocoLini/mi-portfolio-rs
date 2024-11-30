// TODO: Make the social links autogenerables
// TODO: Make the contact Info autogenerables
// TODO: Make the works autogenerables

use crate::lang::MultiLang;
use gloo_net::http::Request;
use serde::Deserialize;
use yew::platform::spawn_local;
use yew::{html, use_effect_with, Html, Properties, UseStateHandle};
use crate::components;

pub trait DynGenerable: Clone + Properties + PartialEq
where
    Self: 'static,
{
    type Data: MultiLang;

    fn r#type(&self) -> String;
    fn resouce_id(&self) -> String;

    fn generate_dyn_html(&self, state: UseStateHandle<Option<Self::Data>>) -> Html
    where
        for<'a> <Self as DynGenerable>::Data: Deserialize<'a>,
    {
        if let Some(data) = &*state {
            self.html_with_data(data)
        } else {
            let a = {
                let data: Self = self.clone();
                let state = state.clone();
                spawn_local(async move {
                    state.set(Some(data.data().await));
                });
            };

            use_effect_with((), move |_| a);
            
            self.html_without_data()
        }
    }

    fn html_with_data(&self, data: &Self::Data) -> Html;
    fn html_without_data(&self) -> Html {
        html! {
            <components::LoadingSpinner />
        }
    }

    async fn data(&self) -> Self::Data
    where
        for<'a> Self::Data: Deserialize<'a>,
    {
        let fetched_data = Request::get(&format!(
            "resources/dyn-data/{}/{}.json",
            self.r#type(),
            self.resouce_id()
        ))
        .send()
        .await
        .expect("Failed to fetch JSON");

        serde_json::from_value::<Self::Data>(
            fetched_data.json().await.expect("Failed to parse JSON"),
        )
        .expect("Failed to deserialize DynGenerable object")
        .translate()
    }
}
