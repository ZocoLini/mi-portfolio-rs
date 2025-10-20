use crate::data_gen::{DynGenerable, IntoHtml};
use crate::lang::{self, MultiLang};
use crate::styles::{Css, PaneType};
use crate::{backend, resources, styles};
use frontend::MultiLang;
use serde::Deserialize;
use std::collections::HashMap;
use std::ops::Add;
use std::string::ToString;
use stylist::css;
use yew::prelude::*;

#[function_component(View)]
pub fn view() -> Html {
    use_effect_with((), move |_| {
        backend::register_content_view("works");
    });
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
                gap: 10px;
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
    icons: Vec<String>,
}

impl IntoHtml for WorkData {
    fn into_html(self) -> Html {
        let css = r#"
            min-width: 350px;
            max-width: 500px;
            width: 90%;
            display: flex;
            position: relative;
            flex-direction: column;
            border-radius: 10px;
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

            ul, p, div {
              margin-left: 20px;
            }
        "#
        .to_string()
        .add(&styles::PaneStyle::new(PaneType::Secondary).css())
        .add(&styles::primary_text_style_as_string())
        .into_css();

        let header_css = css!(
            r#"
            display: flex;
            flex-direction: row;
            gap: 30px;

            img {
                width: 50px;
                height: 50px;
                margin-top: 10px;
            }
            "#
        );

        let other_icons_css = css!(
            r#"
            display: flex;
            flex-direction: row;
            flex-wrap: wrap;
            gap: 10px;

            img {
                width: 30px;
                height: 30px;
            }
            "#
        );

        html!(
          <a class={ css } href={format!("work/{}", self.work_id)} target="_parent">
            <div class={ header_css } >
              <img src={resources::get_work_icon(&self.image_id)} alt={self.work_id}/>
              <h2>{ &self.title }</h2>
            </div>
            <ul>
              {
                for self.info.iter().map(|(key, value)|
                  html! { <li><strong>{ key }{": "}</strong>{ value }</li>
                })
              }
            </ul>
            <p> { &self.description } </p>
            <div class={other_icons_css}>
              {
                  for self.icons.iter().map(|icon|
                      html! { <img src={resources::get_icon(icon)} alt={icon.to_string()} title={icon.to_string()}/> }
                  )
              }
            </div>
          </a>
        )
    }
}
