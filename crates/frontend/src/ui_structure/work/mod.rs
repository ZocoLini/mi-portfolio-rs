use yew::{function_component, html, Html, Properties};
use crate::styles::Css;

#[derive(Clone, PartialEq, Properties)]
pub struct ViewProps
{
    pub work_id: String,
}

#[function_component(View)]
pub fn view(params: &ViewProps) -> Html {
    let css = r#"

    "#
        .to_string()
        .into_css();

    html! {
        <main class="centered flex controlled-size">
          <div>
            <div id="main-pane" class="flex">
              <div id="fichaTecnica" class="pane vertical-flex centered-flex">
                <div id="iconoProyecto-container">
                  <img id="iconoProyecto" src="../../../resources/img/works/leba/icon.png" alt="leba"/>
                </div>
                <h1>{"Leba"}</h1>
        
                <section class="iBox left-icon-list">
                  <h2>{"Ficha Técnica"}</h2>
                  <div class="icon-label">
                    <img class="icon" src="../../../resources/img/icon/codigo.png" alt="Codigo"/>
                    <div class="vertical-flex">
                      <p class="third-text">{"Tecnologías utilizadas"}</p>
                      <p class="second-text">{"C# y Unity"}</p>
                    </div>
                  </div>
        
                  <div class="icon-label">
                    <img class="icon" src="../../../resources/img/icon/empresa.png" alt="Equipo"/>
                    <div class="vertical-flex">
                      <p class="third-text">{"Equipo o Colaboradores"}</p>
                      <p class="second-text">{"Borja Castellano"}</p>
                    </div>
                  </div>
        
                  <div class="icon-label">
                    <img class="icon" src="../../../resources/img/icon/calendario.png" alt="Calendario"/>
                    <div class="vertical-flex">
                      <p class="third-text">{"Fecha de creación"}</p>
                      <p class="second-text">{"2021"}</p>
                    </div>
                  </div>
        
                  <div class="icon-label">
                    <img class="icon" src="../../../resources/img/icon/dado.png" alt="Rol"/>
                    <div class="vertical-flex">
                      <p class="third-text">{"Rol en el proyecto"}</p>
                      <p class="second-text">{"Arquitectira, diseño y desarrollo"}</p>
                    </div>
                  </div>
                </section>
        
                <section class="iBox left-icon-list">
                  <h2>{"Características"}</h2>
                  <div class="icon-label">
                    <img class="icon" src="../../../resources/img/icon/plataformas.png"
                         title="Plataformeo vertical"
                         alt="Plataformeo vertical"/>
                    <p>{"Plataformeo vertical"}</p>
                  </div>
                  <div class="icon-label">
                    <img class="icon" src="../../../resources/img/icon/logros.png"
                         title="Logros" alt="Logros"/>
                    <p>{"Skins y Logros"}</p>
                  </div>
                  <div class="icon-label">
                    <img class="icon" src="../../../resources/img/icon/procedural.png"
                         title="Generación procedural"
                         alt="Generación procedural"/>
                    <p>{"Generación procedural"}</p>
                  </div>
                  <div class="icon-label">
                    <img class="icon" src="../../../resources/img/icon/combate.png"
                         title="Combate" alt="Combate"/>
                    <p>{"Leaderboard"}</p>
                  </div>
                </section>
              </div>
        
              <div id="central-pane">
                <section id="description-pane" class="pane">
                  <h2>{"Descripción del proyecto"}</h2>
        
                  <div class="">
                    <p>
                      {"Juego de platformeo vertical con generación procedural de niveles. La mecánica principal del juego es ir
                      saltando de pared en pared para ascender tan alto como puedas. Por el camino tendrás que esquivar
                      obstaculos que no te lo pondrán fácil y enemigos con mecánicas distintivas y únicas."}
                    </p>
                    <p>
                      {"El juego cuenta con un sistema de skins y logros que irás desbloqueando a medida que vayas jugando.
                      Además cuenta con servidor propio y un sistema de clasificación global de jugadores basado en su
                      puntuación máxima."}
                    </p>
                    <p>
                      {"En cuanto a la arquitectura del código, se aplican diferentes patrones de diseño que permiten una fácil
                      escalabilidad y mantenimiento de este tales como el patrón estrategia o estado."}
                    </p>
                  </div>
                </section>
        
                <section id="estado-pane" class="pane">
                  <h2>{"Estado"}</h2>
        
                  <div>
                    <ul>
                      <li>
                        <strong>{"Disponibilidad:"}</strong> {"El juego está disponible en "}<a target="_blank"
                        href="https://play.google.com/store/apps/details?id=com.LebaStudios.Leba&hl=es_419&gl=ES">{"Play Store"}</a>
                        {"para su descarga."}
                      </li>
                    </ul>
                  </div>
                </section>
              </div>
            </div>
        
            <article id="contenedor-multimedia" class="centered">
              <section class="pane">
                <h2>{"Multimedia"}</h2>
        
                <div class="controlled-size centered ">
                  <div class="corrillo-de-imagenes-movil">
                    <img src="../../../resources/img/works/leba/captura1.jpg" alt="Captura 1"/>
                    <img src="../../../resources/img/works/leba/captura2.jpg" alt="Captura 2"/>
                    <img src="../../../resources/img/works/leba/captura3.jpg" alt="Captura 3"/>
                    <img src="../../../resources/img/works/leba/captura4.jpg" alt="Captura 4"/>
                    <img src="../../../resources/img/works/leba/captura5.jpg" alt="Captura 5"/>
                    <img src="../../../resources/img/works/leba/captura6.jpg" alt="Captura 6"/>
                    <img src="../../../resources/img/works/leba/captura7.jpg" alt="Captura 7"/>
                    <img src="../../../resources/img/works/leba/captura8.jpg" alt="Captura 8"/>
                  </div>
                </div>
              </section>
            </article>
          </div>
        
          <div id="nav-bar" class="vertical-flex fit-content">
            <a class="pane-button" href="../../index.html" title="Home">
              <img src="../../../resources/img/icon/home.png" alt="Home"/>
            </a>
            <a class="pane-button flex" href="#contenedor-multimedia" title="Multimedia">
              <img src="../../../resources/img/icon/multimedia.png" alt="Multimedia"/>
            </a>
            <a class="pane-button flex"
               href="https://play.google.com/store/apps/details?id=com.LebaStudios.Leba&hl=es_419&gl=ES"
               target="_blank">
              <img src="../../../resources/img/icon/playstore.png" alt="PlayStore"/>
            </a>
          </div>
        </main>
    }
}