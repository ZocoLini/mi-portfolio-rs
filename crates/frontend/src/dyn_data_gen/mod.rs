// TODO: Make the social links autogenerables
// TODO: Make the contact Info autogenerables

use crate::lang::MultiLang;
use gloo_net::http::Request;
use serde::Deserialize;
#[cfg(debug_assertions)]
use web_sys::console;
use yew::platform::spawn_local;
use yew::{html, Html, Properties, UseStateHandle};
use crate::{components, resources};

pub trait DynGenerable: Clone + Properties + PartialEq
where
    Self: 'static,
{
    type Data: MultiLang + Clone;

    fn resouce_id(&self) -> String;

    fn generate_dyn_html(&self, state: UseStateHandle<Option<Self::Data>>) -> Html
    where
        for<'a> <Self as DynGenerable>::Data: Deserialize<'a>,
    {
        if let Some(data) = state.clone().as_ref() {
            self.html_with_data((*data).clone())
        } else {
            {
                let data: Self = self.clone();
                let state = state.clone();
                spawn_local(async move {
                    state.set(Some(data.data().await));
                });
            };
            
            self.html_without_data()
        }
    }

    fn html_with_data(&self, data: Self::Data) -> Html;
    fn html_without_data(&self) -> Html {
        html! {
            <components::LoadingSpinner />
        }
    }

    async fn data(&self) -> Self::Data
    where
        for<'a> Self::Data: Deserialize<'a>,
    {
        let data_path = resources::get_dyn_data_src(&self.resouce_id());

        #[cfg(debug_assertions)]
        console::log_1(&(&data_path).into());

        let fetched_data = Request::get(&data_path)
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

pub trait IntoHtml {
    fn into_html(self) -> Html;
}