use crate::dyn_data_gen::DynGenerable;
use serde::Deserialize;
use std::clone::Clone;
use std::string::ToString;
use yew::prelude::*;
use crate::lang::MultiLang;
use macros::MultiLang;
use crate::components::icon::Icon;

#[function_component(View)]
pub fn view() -> Html {
    html! {
        <div class="iframe-pane">
          <div class="iframe-inner-pane">
            <h2>{"Sobre mi"}</h2>
            <p class="primary-text">{"
              Soy un programador apasionado por la resolución de problemas y la creación de soluciones
              innovadoras, amante del borrow checker y de aprender . Con una formación en
              Matemáticas y una trayectoria autodidacta en programación, tengo experiencia en el desarrollo de aplicaciones y la
              resolución de problemas utilizando lenguajes como C, C #, Java y, recientemente, Rust."}
            </p>
            <p class="primary-text">{"
              Actualmente me considero plenamente preparado para entrar a entornos de desarrollo profesionales y seguir creciendo
              como programador, aprendiendo nuevas tecnologías y mejorando mis habilidades. Este año también acabaré el ciclo
              superior de Desarrollo de Aplicaciones Multiplataforma y estoy buscando una oportunidad laboral que tenga interés en
              mi perfil y en permitirme hacer la FCT con ellos al final del actual curso 2024-2025."}
            </p>

            <h2>{"Habilidades"}</h2>

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
        "skill-panes".to_string()
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
