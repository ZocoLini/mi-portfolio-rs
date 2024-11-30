use crate::dyn_data_gen::DynGenerable;
use crate::lang::MultiLang;
use crate::styles::text;
use serde::Deserialize;
use std::clone::Clone;
use std::string::ToString;
use yew::prelude::*;
use frontend::MultiLang;
use crate::components::Icon;
use crate::lang;

#[function_component(View)]
pub fn view() -> Html {
    html! {
        <div class="iframe-pane">
          <div class="iframe-inner-pane">
            <h2>{ lang::translate("%general.about-me") }</h2>
            <p class={ text::primary_text_style() }>{ lang::translate("%about.p-1") }</p>
            <p class={ text::primary_text_style() }>{ lang::translate("%about.p-2") }</p>

            <h2>{ lang::translate("%general.skills") }</h2>

            <div id="contenedor-habilidades">
              <Skill skill_id="problem-solving"/>
              <Skill skill_id="selftaught"/>
              <Skill skill_id="java-javafx"/>
              <Skill skill_id="rust"/>
              <Skill skill_id="cs-unity"/>
              <Skill skill_id="microservices"/>
              <Skill skill_id="teamwork-leadership"/>
            </div>
          </div>
        </div>
    }
}

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

    fn r#type(&self) -> String {
        "skill".to_string()
    }

    fn resouce_id(&self) -> String {
        self.skill_id.clone()
    }

    fn html_with_data(&self, data: &Self::Data) -> Html {
        html! {
            <skill class="habilidad">
                { data.icon.html() }
                <div class="habilidad-texto">
                    <h3>{ &data.title }</h3>
                    <p>{ &data.description }</p>
                </div>
            </skill>
        }
    }
}
