use crate::components::Icon;
use crate::dyn_data_gen::DynGenerable;
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

        </div>
    }
}

#[derive(Properties, PartialEq, Clone)]
struct WorkSectionProps {
    title: String,
    works: Vec<WorkProps>,
}

#[function_component(WorkSection)]
fn work_section(props: &WorkSectionProps) -> Html {
    html!(
        <div>
          <h1>{ &props.title }</h1>

          <div class="contenedor-works">
              {
                for props.works.iter().map(move |work_props|
                  html! { <Work work_id={ work_props.work_id.clone() }/> }
                )
              }
          </div>
        </div>
    )
}

#[derive(PartialEq, Clone, Deserialize)]
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

#[derive(Properties, PartialEq, Clone)]
struct WorkProps {
    work_id: String,
}

#[derive(Deserialize, MultiLang)]
pub struct WorkData {
    title: String,
    description: String,
    icon: Icon,
    info: Vec<(String, String)>,
    state: WorkState,
}

impl MultiLang for Vec<(String, String)> {
    fn translate(self) -> Self {
        let mut translated = Vec::new();

        for (key, value) in self.into_iter() {
            translated.push((key.translate(), value.translate()));
        }

        translated
    }
}

#[function_component(Work)]
fn work(props: &WorkProps) -> Html {
    let state = use_state(|| None);
    props.generate_dyn_html(state)
}

impl DynGenerable for WorkProps {
    type Data = WorkData;

    fn resouce_id(&self) -> String {
        self.work_id.to_string()
    }

    fn html_with_data(&self, data: &Self::Data) -> Html {
        html!(
          <a class="work primary-text" href="works/trt-api.html" target="_parent">
            <img src="../../resources/img/works/the-round-table/icono-trt.png" alt="the-round-table"/>
            <img src="../../resources/img/icon/api.png" alt="api" style="position: absolute; left: -5px; top: -15px; width: 36px; height: 36px;"/>
            <div class="work-info">
              <h2>{ &data.title }</h2>
              <ul>
                {
                  for data.info.iter().map(|(key, value)|
                    html! { <li><strong>{ key }</strong>{ value }</li>
                  })
                }
              </ul>
              <p> { &data.description } </p>
            </div>
            <img class="work-state" src="../../resources/img/icon/deployed.png" alt="Deployed"/>
          </a>
        )
    }
}
