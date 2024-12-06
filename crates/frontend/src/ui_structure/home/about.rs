use crate::components::Icon;
use crate::dyn_data_gen::{DynGenerable, IntoHtml};
use crate::{lang, styles};
use crate::lang::MultiLang;
use crate::styles::{Css};
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
            <p class={ styles::primary_text_style() }>{ lang::translate("%about.p-1") }</p>
            <p class={ styles::primary_text_style() }>{ lang::translate("%about.p-2") }</p>

            <SkillsContainer />
        </div>
    }
}

// region: Skill container

#[derive(Properties, PartialEq, Clone)]
struct SkillsContainerProps;

#[derive(Deserialize, MultiLang, Clone)]
struct SkillsContainerData {
    skills: Vec<SkillData>,
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

    fn html_with_data(&self, data: Self::Data) -> Html {
        let css = r#"
            #skills {
                display: flex;
                flex-wrap: wrap;
                justify-content: center;
            }
        "#
        .to_string()
        .into_css();

        html!(
            <div class={ css }>
                <h2>{ lang::translate("%general.skills") }</h2>
                <div id="skills">
                    {
                        for data.skills.into_iter().map(move |skill| {
                            skill.clone().into_html()
                        })
                    }
                </div>
            </div>
        )
    }
}
// enregion: Skill container

// region: Skill component

#[derive(Deserialize, MultiLang, Clone)]
struct SkillData {
    title: String,
    description: String,
    icon: Icon,
}

impl IntoHtml for SkillData {
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
        "#
        .to_string()
        .add(&styles::PaneStyle::new(styles::PaneType::Secondary).css())
        .into_css();

        html! {
            <skill class={css}>
                { self.icon.html() }
                <div>
                    <h3>{ &self.title }</h3>
                    <p>{ &self.description }</p>
                </div>
            </skill>
        }
    }
}
// endregion
