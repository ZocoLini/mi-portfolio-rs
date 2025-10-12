use crate::data_gen::{DynGenerable, IntoHtml};
use crate::lang::{self, MultiLang};
use crate::styles::{Css, PaneType};
use crate::{resources, styles};
use frontend::MultiLang;
use serde::Deserialize;
use std::collections::HashMap;
use std::ops::Add;
use std::string::ToString;
use stylist::css;
use yew::prelude::*;

#[function_component(View)]
pub fn view() -> Html {
    let css = r#"

    "#
    .to_string()
    .into_css();

    html! {
        <div class={css}>
            <Works />
        </div>
    }
}

#[derive(Deserialize, MultiLang, Clone)]
struct WorksData {
    works: Vec<WorkData>,
}

#[derive(Properties, PartialEq, Clone)]
struct WorksProps;

#[function_component(Works)]
fn works(props: &WorksProps) -> Html {
    let state = use_state(|| None);
    props.generate_dyn_html(state)
}

impl DynGenerable for WorksProps {
    type Data = WorksData;

    fn resouce_id(&self) -> String {
        "works".to_string()
    }

    fn html_with_data(&self, data: Self::Data) -> Html {
        let css = r#"
            display: flex;
            flex-direction: column;
            gap: 20px;
            align-items: center;
            
            #contenedor-works {
                display: flex;
                flex-wrap: wrap;
                justify-content: center;
                gap: 20px;
            }

            h2 {
                text-align: center;
            }
        "#
        .to_string()
        .into_css();

        html!(
            <div class={ css }>
                <h1>{lang::translate("%work.view.title")}</h1>
                <div id="contenedor-works">
                    {
                      for data.works.iter().map(move |work|
                        work.clone().into_html()
                      )
                    }
                </div>
            </div>
        )
    }
}

#[derive(Deserialize, MultiLang, Clone)]
pub struct WorkData {
    title: String,
    image_id: String,
    work_id: String,
    description: String,
    info: HashMap<String, String>,
}

impl IntoHtml for WorkData {
    fn into_html(self) -> Html {
        let css = r#"
            min-width: 300px;
            max-width: 400px;
            width: 45%;
            display: flex;
            position: relative;
            border-radius: 10px;
            padding: 0 10px;
            transition: background-color 0.3s, color 0.3s;
            text-decoration: none;

            :hover
            {
              background-color: var(--color-button-hover);
              color: white;
            }
            ul
            {
              padding: 0;
              list-style-type: none;
            }

            li {
              margin-bottom: 5px;
            }
        "#
        .to_string()
        .add(&styles::PaneStyle::new(PaneType::Secondary).css())
        .add(&styles::primary_text_style_as_string())
        .into_css();

        let work_icon_css = css!(
            r#"
            width: 50px;
            height: 50px;
            margin-top: 10px;
            margin-right: 10px;
            "#
        );

        html!(
          <a class={ css } href={format!("work/{}", self.work_id)} target="_parent">
            <img class={ work_icon_css } src={resources::get_work_icon(&self.image_id)} alt={self.work_id}/>
            <div class="work-info">
              <h2>{ &self.title }</h2>
              <ul>
                {
                  for self.info.iter().map(|(key, value)|
                    html! { <li><strong>{ key }{": "}</strong>{ value }</li>
                  })
                }
              </ul>
              <p> { &self.description } </p>
            </div>
          </a>
        )
    }
}
