use frontend::MultiLang;
use serde::Deserialize;
use std::ops::Add;
use yew::{function_component, html, use_state, Html, Properties};

use crate::{
    components::Icon,
    dyn_data_gen::{DynGenerable, IntoHtml},
    lang::{self, MultiLang},
    styles::{self, Css},
};

#[function_component(View)]
pub fn view() -> Html {
    html! {
        <div>
            <h1>{ lang::translate("%general.knowledge") }</h1>

            <KnowledgeContainer />
        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
struct KnowledgeProps;

#[derive(Deserialize, MultiLang, Clone)]
struct KnowledgeData {
    categories: Vec<KnowledgeCategoryData>,
}

#[function_component(KnowledgeContainer)]
fn knowledge_container(props: &KnowledgeProps) -> Html {
    let state = use_state(|| None);
    props.generate_dyn_html(state)
}

impl DynGenerable for KnowledgeProps {
    type Data = KnowledgeData;

    fn resouce_id(&self) -> String {
        "knowledge".to_string()
    }

    fn html_with_data(&self, data: Self::Data) -> Html {
        let css = r#"
            #knowledge {
                display: flex;
                flex-wrap: wrap;
                justify-content: center;
            }
        "#
        .to_string()
        .into_css();

        html!(
            <div class={ css }>
            {
                for data.categories.into_iter().map(move |category| {
                    category.clone().into_html()
                })
            }
            </div>
        )
    }
}

#[derive(Deserialize, MultiLang, Clone)]
struct KnowledgeCategoryData {
    name: String,
    items: Vec<KnowledgeItemData>,
}

impl IntoHtml for KnowledgeCategoryData {
    fn into_html(self) -> Html {
        let css = r#"
            #category {
                display: flex;
                flex-wrap: wrap;
                justify-content: center;
            }

            img
            {
              width: 50px;
              height: 50px;
              margin-top: 10px;
              margin-right: 10px;
            }
        "#
        .to_string()
        .add(&styles::PaneStyle::new(styles::PaneType::Primary).css())
        .into_css();

        html! {
            <category class={css}>
                <h3>{ &self.name }</h3>
                <div id="category">
                    {
                        for self.items.into_iter().map(move |item| {
                            item.clone().into_html()
                        })
                    }
                </div>
            </category>
        }
    }
}

#[derive(Deserialize, MultiLang, Clone)]
struct KnowledgeItemData {
    name: String,
    level: f32,
    icon: Icon,
}

impl IntoHtml for KnowledgeItemData {
    fn into_html(self) -> Html {
        let css = r#"
            min-width: 300px;
            max-width: 400px;
            width: 45%;
            display: flex;
            overflow: hidden;

            img
            {
              width: 50px;
              height: 50px;
              margin-top: 10px;
              margin-right: 10px;
            }

            #progress-bar {
              height: 100%;
              width: 100%;
              background-color: #4caf50;
              text-align: center;
              color: white;
              border-radius: 5px;
              line-height: 25px;
              transition: width 0.4s ease;
            }
        "#
        .to_string()
        .add(&styles::PaneStyle::new(styles::PaneType::Secondary).css())
        .into_css();

        html! {
            <knowledge class={css}>
                { self.icon.html() }
                <div>
                    <h3>{ &self.name }</h3>
                    <div id="progress-bar"/>
                </div>
            </knowledge>
        }
    }
}
