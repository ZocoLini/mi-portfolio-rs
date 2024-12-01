use crate::components::Icon;
use crate::dyn_data_gen::DynGenerable;
use crate::lang;
use crate::lang::MultiLang;
use crate::styles::pane::PaneType::Secondary;
use crate::styles::{pane, text, Css};
use frontend::MultiLang;
use serde::Deserialize;
use std::clone::Clone;
use std::ops::Add;
use std::string::ToString;
use yew::prelude::*;

#[function_component(View)]
pub fn view() -> Html {
    html! {
        <div>
            <h1>{ lang::translate("%general.about-me") }</h1>
            <p class={ text::primary_text_style() }>{ lang::translate("%about.p-1") }</p>
            <p class={ text::primary_text_style() }>{ lang::translate("%about.p-2") }</p>

            <SkillsContainer />
        </div>
    }
}

// region: Skill component

#[derive(Properties, PartialEq, Clone)]
struct SkillProps {
    skill_id: String,
}

#[derive(Deserialize, MultiLang)]
struct SkillData {
    title: String,
    description: String,
    icon: Icon,
}

#[function_component(Skill)]
fn skill(props: &SkillProps) -> Html {
    let state = use_state(|| None);

    props.generate_dyn_html(state)
}

impl DynGenerable for SkillProps {
    type Data = SkillData;

    fn resouce_id(&self) -> String {
        self.skill_id.clone()
    }

    fn html_with_data(&self, data: &Self::Data) -> Html {
        let css = r#"
            min-width: 300px;
            max-width: 500px;
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
        "#
        .to_string()
        .add(&pane::PaneStyle::new(Secondary).css())
        .into_css();

        html! {
            <skill class={css}>
                { data.icon.html() }
                <div>
                    <h3>{ &data.title }</h3>
                    <p>{ &data.description }</p>
                </div>
            </skill>
        }
    }
}

// endregion

// region: Skill container

#[derive(Properties, PartialEq, Clone)]
struct SkillsContainerProps;

#[derive(Deserialize, MultiLang)]
struct SkillsContainerData {
    skills: Vec<SkillData>,
}

impl MultiLang for Vec<SkillData> {
    fn translate(self) -> Self {
        self.into_iter()
            .map(|x| x.translate())
            .collect()
    }
}

#[function_component(SkillsContainer)]
fn skills_container(props: &SkillsContainerProps) -> Html {
    let state = use_state(|| None);
    props.generate_dyn_html(state)
}

impl DynGenerable for SkillsContainerProps {
    type Data = SkillsContainerData;

    fn resouce_id(&self) -> String {
        "skills".to_string()
    }

    fn html_with_data(&self, data: &Self::Data) -> Html {
        let css = r#"
            #skills {
                display: flex;
                flex-wrap: wrap;
                justify-content: center;
            }
        "#
        .to_string()
        .into_css();

        // TODO: Mostrar las Skills cargadas en data
        
        html!(
            <div class={ css }>
                <h2>{ lang::translate("%general.skills") }</h2>

                <div id="skills">
                </div>
            </div>
        )
    }
}

// enregion: Skill container
