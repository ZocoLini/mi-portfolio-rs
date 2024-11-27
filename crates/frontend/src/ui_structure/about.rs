use crate::dyn_data_gen::DynGenerable;
use serde::Deserialize;
use yew::prelude::*;

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
              <div class="habilidad">
                <img src="../img/icon/puzzle.png" alt="puzzle"/>
                <div class="habilidad-texto">
                  <h3>{"Resolución de problemas y Pensamiento Crítico"}</h3>
                  <p>{"
                    El grado en Matemáticas me ha dotado de habilidades sólidas en resolución de problemas y pensamiento
                    crítico aplicables al desarrollo de software."}
                  </p>
                </div>
              </div>

              <div class="habilidad">
                <img src="../img/icon/aprendizaje.png" alt="aprendizaje"/>
                <div class="habilidad-texto">
                  <h3>{"Aprendizaje Autodidacta"}</h3>
                  <p>{"
                    Mi formación autodidacta demuestra mi capacidad para aprender de forma independiente y adaptarme a nuevas
                    tecnologías y conceptos."}
                  </p>
                </div>
              </div>

              <div class="habilidad">
                <img src="../img/icon/java-logo.png" alt="java"/>
                <div class="habilidad-texto">
                  <h3>{"Java y JavaFX"}</h3>
                  <p>{"
                    Tanto Java como Java FX me han permitido expandir mi conjunto de habilidades en programación y mis
                    posibilidades al poder crear tanto aplicaciones gráficas multiplataforma como microservicios."}
                  </p>
                </div>
              </div>

              <div class="habilidad">
                <img src="../img/icon/rust.png" alt="rust"/>
                <div class="habilidad-texto">
                  <h3>{"Rust"}</h3>
                  <p>{"
                    Siendo Rust un lenguaje que me encanta, mi experiencia con el me ha permitido explorar nuevas formas de
                    programar y desarrollar aplicaciones de
                    alto rendimiento creando REST APIs y aplicaciones de consola que se ejecutan de forma eficiente."}
                  </p>
                </div>
              </div>

              <div class="habilidad">
                <img src="../img/icon/csharp-logo.png" alt="csharp"/>
                <div class="habilidad-texto">
                  <h3>{"C # y Unity"}</h3>
                  <p>
                    {"Con dos años de experiencia utilizando Unity y C #, he desarrollado habilidades sólidas en el desarrollo de
                    aplicaciones, demostrando así mi capacidad para trabajar en entornos de desarrollo complejos."}
                  </p>
                </div>
              </div>

              <div class="habilidad">
                <img src="../img/icon/microservices.png" alt="microservicios"/>
                <div class="habilidad-texto">
                  <h3>{"Microservicios"}</h3>
                  <p>{"
                    Como parte de los proyectos en los que he trabajado, he podido aprender a usar frameworks como Spring Boot
                    y Rocket para crear microservicios, lo que me hace capaz de desarrollar aplicaciones escalables y
                    mantenibles."}
                  </p>
                </div>
              </div>

              <div class="habilidad">
                <img src="../img/icon/liderazgo.png" alt="liderazgo"/>
                <div class="habilidad-texto">
                  <h3>{"Liderazgo y trabajo en equipo"}</h3>
                  <p>
                    {"Mi capacidad para liderar, colaborar en equipo y para comunicar ideas de manera efectiva se ve reforzada por
                    mi experiencia como profesor."}
                  </p>
                </div>
              </div>
            </div>
          </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
struct SkillProps {}

#[derive(Deserialize)]
struct SkillData {}

struct Skill {}

impl Component for Skill {
    type Message = ();
    type Properties = SkillProps;

    fn create(ctx: &Context<Self>) -> Self {
        todo!()
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        self.generate()
    }
}

impl DynGenerable for Skill {
    type Data = ();

    fn r#type(&self) -> String {
        todo!()
    }

    fn generate(&self) -> Html {
        let data = self.data();

        html!()
    }
}
