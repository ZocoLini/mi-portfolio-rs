use crate::dyn_data_gen::{DynGenerable, IntoHtml};
use crate::lang::MultiLang;
use crate::resources;
use crate::styles::Css;
use frontend::MultiLang;
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::Display;
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
    sections: Vec<WorkSectionData>,
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

        "#
        .to_string()
        .into_css();

        html!(
            <div class={ css }>
                {
                    for data.sections.iter().map(|work_section|
                        work_section.clone().into_html()
                    )
                }
            </div>
        )
    }
}

#[derive(Deserialize, MultiLang, Clone)]
struct WorkSectionData {
    title: String,
    works: Vec<WorkData>,
}

impl IntoHtml for WorkSectionData {
    fn into_html(self) -> Html {
        let css = r#"
            #contenedor-works {
                display: flex;
                flex-wrap: wrap;
                justify-content: center;
                gap: 20px;
            }
        "#
        .to_string()
        .into_css();

        html!(
        <div class={ css }>
          <h1>{ &self.title }</h1>

          <div id="contenedor-works">
              {
                for self.works.iter().map(move |work|
                  work.clone().into_html()
                )
              }
          </div>
        </div>
        )
    }
}

#[derive(PartialEq, Clone, Deserialize, Copy)]
enum WorkState {
    Building,
    Deployed,
    Concept,
}

impl WorkState {
    fn icon(&self) -> Html {
        let id = match self {
            WorkState::Building => "building",
            WorkState::Deployed => "deployed",
            WorkState::Concept => "concept",
        };

        let css = r#"
            position: absolute;
            top: -25px;
            right: 0;
            margin: 10px;
            min-width: 100px;
            max-height: 33px;
            z-index: 10;
        "#
        .to_string()
        .into_css();

        html! {
            <img class={ css }
                src={format!("{}.png", resources::get_icon_src(id))}
                alt={self.to_string()}/>
        }
    }
}

impl Display for WorkState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            WorkState::Building => "building".to_string(),
            WorkState::Deployed => "deployed".to_string(),
            WorkState::Concept => "concept".to_string(),
        };
        write!(f, "{}", str)
    }
}

impl MultiLang for WorkState {
    fn translate(self) -> Self {
        self
    }
}

#[derive(Deserialize, MultiLang, Clone)]
pub struct WorkData {
    title: String,
    icon_id: String,
    #[serde(default)]
    is_api: bool,
    description: String,
    info: HashMap<String, String>,
    state: WorkState,
}

impl IntoHtml for WorkData {
    fn into_html(self) -> Html {
        let css = r#"
            min-width: 275px;
            max-width: 500px;
            width: 45%;
            display: flex;
            position: relative;
            border-radius: 10px;
            padding: 0 10px;
            transition: background-color 0.3s, color 0.3s;
            background-color: var(--color-ibox-background);
            text-decoration: none;
            color: var(--color-primary-text);
              
            :hover
            {
              background-color: var(--color-button-hover);
              color: white;
            }
            img
            {
              width: 50px;
              height: 50px;
              margin-top: 10px;
              margin-right: 10px;
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
        .into_css();

        let api_icon_css = css!(
            r#"
            position: absolute;
            left: -5px;
            top: -15px;
            width: 36px;
            height: 36px;
            "#
        );

        html!(
          <a class={ css} href="works/trt-api.html" target="_parent">
            <img src={resources::get_work_icon_src(&self.icon_id)} alt={self.icon_id}/>
            if self.is_api {
                <img class={ api_icon_css } src={resources::get_icon_src("api.png")} alt="api"/>
            }
            <div class="work-info">
              <h2>{ &self.title }</h2>
              <ul>
                {
                  for self.info.iter().map(|(key, value)|
                    html! { <li><strong>{ key }</strong>{ value }</li>
                  })
                }
              </ul>
              <p> { &self.description } </p>
            </div>
            {
                self.state.icon()
            }
          </a>
        )
    }
}
