use std::collections::HashMap;
use crate::components::Icon;
use crate::dyn_data_gen::{DynGenerable, IntoHtml};
use crate::lang::MultiLang;
use crate::styles::Css;
use frontend::MultiLang;
use serde::Deserialize;
use std::string::ToString;
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

        "#
        .to_string()
        .into_css();

        html!(
        <div class={ css }>
          <h1>{ &self.title }</h1>

          <div class="contenedor-works">
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

impl MultiLang for WorkState {
    fn translate(self) -> Self {
        self
    }
}

#[derive(Deserialize, MultiLang, Clone)]
pub struct WorkData {
    title: String,
    ref_id: String,
    #[serde(default)]
    is_api: bool,
    description: String,
    icon: Icon,
    info: HashMap<String, String>,
    state: WorkState,
}

impl IntoHtml for WorkData {
    fn into_html(self) -> Html {
        let css = r#"

        "#
        .to_string()
        .into_css();

        html!(
          <a class="work primary-text" href="works/trt-api.html" target="_parent">
            <img src="../../resources/img/works/the-round-table/icono-trt.png" alt="the-round-table"/>
            <img src="../../resources/img/icon/api.png" alt="api" style="position: absolute; left: -5px; top: -15px; width: 36px; height: 36px;"/>
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
            <img class="work-state" src="../../resources/img/icon/deployed.png" alt="Deployed"/>
          </a>
        )
    }
}
