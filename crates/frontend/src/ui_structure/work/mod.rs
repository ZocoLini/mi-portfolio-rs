use crate::components::{IconButton, IconizedItem};
use crate::dyn_data_gen::DynGenerable;
use crate::lang::MultiLang;
use crate::styles::Css;
use crate::{resources, styles};
use frontend::MultiLang;
use serde::Deserialize;
use std::collections::HashMap;
use std::convert::From;
use std::ops::Add;
use std::string::ToString;
use yew::{function_component, html, use_state, Callback, Html, Properties};

#[derive(Clone, PartialEq, Properties)]
pub struct ViewProps {
    pub work_id: String,
}

#[derive(Clone, Deserialize, MultiLang)]
pub struct ViewData {
    name: String,
    technicaldata: Vec<IconizedItemData>,
    features: Vec<IconizedItemData>,
    description: Vec<String>,
    state: HashMap<String, String>,
    multimedia: MultimediaData,
    links: Vec<LinkItemData>,
}

#[derive(Clone, Deserialize, MultiLang)]
struct IconizedItemData {
    icon_id: String,
    title: String,
    detail: String,
}

#[derive(Clone, Deserialize, MultiLang)]
struct LinkItemData {
    icon_id: String,
    title: String,
    url: String,
}

#[derive(Clone, Deserialize, MultiLang)]
struct MultimediaData {
    r#type: String,
    images_ids: Vec<String>,
}

#[function_component(View)]
pub fn view(params: &ViewProps) -> Html 
{
    let state = use_state(|| None);
    params.generate_dyn_html(state)
}

impl DynGenerable for ViewProps {
    type Data = ViewData;

    fn resouce_id(&self) -> String {
        format!("works/{}.json", self.work_id)
    }

    fn html_with_data(&self, data: Self::Data) -> Html {
        let css = r#"
display: flex;
margin: 0 auto;
width: min(85%, 1200px);
padding-top: 100px;

h2, h1 {
  text-align: center;
}

.controlled-size {
  width: min(85%, 1200px);
}

#main-pane {
  display: flex;
}

@media (max-width: 1080px) {
  #main-pane {
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  min-width: 350px;
  max-width: 700px;
  width: 80%;
}
    "#
        .to_string()
        .into_css();

        html! {
            <main class={ css }>
              <div>
                <div id="main-pane">
                  <LeftPane />

                  <center-pane id="central-pane">
                    <Description />
                    <State />
                  </center-pane>
                </div>

                <Multimedia />
              </div>

              <NavigationBar />
            </main>
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
struct LeftPaneProps {}

#[function_component(LeftPane)]
fn left_pane(props: &LeftPaneProps) -> Html {
    let css = r#"
height: fit-content;
min-width: 350px;
max-width: 550px;
width: 90%;
display: flex;
flex-direction: column;
align-items: center;

#fichaTecnica section {
  margin-bottom: 15px;
  width: 90%;
}

#iconoProyecto-container {
  height: 100px;
  width: 200px;
  overflow: visible;
  position: relative;
}

#iconoProyecto {
  height: 200px;
  width: 200px;
  position: absolute;
  border-radius: 10%;
  top: -100px;
}
        "#
    .to_string()
    .add(&styles::PaneStyle::new(styles::PaneType::Primary).css())
    .into_css();

    html! {
        <left-pane class={ css }>
            <div id="iconoProyecto-container">
              <img id="iconoProyecto" src={ resources::get_work_icon_src("leba")} alt="leba"/>
            </div>
            <h1>{"Leba"}</h1>
            <Technicaldata />
            <Features />
        </left-pane>
    }
}

#[derive(Clone, PartialEq, Properties)]
struct TechnicaldataProps {}

#[function_component(Technicaldata)]
fn metadata(_props: &TechnicaldataProps) -> Html {
    let css = r#"
width: 90%;
    "#
    .to_string()
    .add(&styles::PaneStyle::new(styles::PaneType::Secondary).css())
    .into_css();

    html! {
        <metadata class={ css }>
            <h2>{"Ficha Técnica"}</h2>
            <IconizedItem
                    icon_id="codigo.png"
                    alt_text="code"
                    title="Tecnologías utilizadas"
                    detail="C# y Unity"
            />
            <IconizedItem
                    icon_id="codigo.png"
                    alt_text="code"
                    title="Tecnologías utilizadas"
                    detail="C# y Unity"
            />
            <IconizedItem
                    icon_id="codigo.png"
                    alt_text="code"
                    title="Tecnologías utilizadas"
                    detail="C# y Unity"
            />
            <IconizedItem
                    icon_id="codigo.png"
                    alt_text="code"
                    title="Tecnologías utilizadas"
                    detail="C# y Unity"
            />
            <IconizedItem
                    icon_id="codigo.png"
                    alt_text="code"
                    title="Tecnologías utilizadas"
                    detail="C# y Unity"
            />
        </metadata>
    }
}

#[derive(Clone, PartialEq, Properties)]
struct FeaturesProps {}

#[function_component(Features)]
fn features(_props: &FeaturesProps) -> Html {
    let css = r#"
width: 90%;
    "#
    .to_string()
    .add(&styles::PaneStyle::new(styles::PaneType::Secondary).css())
    .into_css();

    html! {
        <features class={ css }>
            <h2>{"Características"}</h2>
            <IconizedItem
                    icon_id="plataformas.png"
                    alt_text="platforms"
                    title=""
                    detail="Plataformeo vertical"
            />
            <IconizedItem
                    icon_id="plataformas.png"
                    alt_text="platforms"
                    title=""
                    detail="Plataformeo vertical"
            />
            <IconizedItem
                    icon_id="plataformas.png"
                    alt_text="platforms"
                    title=""
                    detail="Plataformeo vertical"
            />
            <IconizedItem
                    icon_id="plataformas.png"
                    alt_text="platforms"
                    title=""
                    detail="Plataformeo vertical"
            />
        </features>
    }
}

#[derive(Clone, PartialEq, Properties)]
struct DescriptionProps {}

#[function_component(Description)]
fn description(_props: &DescriptionProps) -> Html {
    let css = r#"
p {
    padding: 5px 15px;
}
        "#
    .to_string()
    .add(&styles::PaneStyle::new(styles::PaneType::Primary).css())
    .into_css();

    html! {
        <div class={ css }>
            <h2>{"Descripción del proyecto"}</h2>
            <div>
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
        </div>
    }
}

#[derive(Clone, PartialEq, Properties)]
struct StateProps {}

#[function_component(State)]
fn state(_props: &StateProps) -> Html {
    let css = r#"
li {
  padding: 5px 0px 0px 15px;
}
        "#
    .to_string()
    .add(&styles::PaneStyle::new(styles::PaneType::Primary).css())
    .into_css();

    html! {
        <div class={ css }>
            <h2>{"Estado"}</h2>
            <div>
              <ul>
                <li>
                  <strong>{"Disponibilidad: "}</strong> {"El juego está disponible en "}<a target="_blank"
                  href="https://play.google.com/store/apps/details?id=com.LebaStudios.Leba&hl=es_419&gl=ES">{"Play Store"}</a>
                  {" para su descarga."}
                </li>
              </ul>
            </div>
        </div>
    }
}

#[derive(Clone, PartialEq, Properties)]
struct MultimediaProps {}

#[function_component(Multimedia)]
fn multimedia(_props: &MultimediaProps) -> Html {
    let css = r#"
margin: 0 auto;

#img-container {
  width: min(85%, 1200px);
  margin: 0 auto;
}

#corrillo-de-imagenes-movil {
  display: flex;
  gap: 10px;
  flex-wrap: wrap;
  justify-content: center;
}

#corrillo-de-imagenes-movil img {
  width: 200px;
  height: auto;
  border-radius: 10px;
}

#corrillo-de-imagenes-pc {
  justify-content: center;
  display: flex;
  gap: 10px;
  flex-wrap: wrap;
}

#corrillo-de-imagenes-pc img {
  width: 100%;
  height: auto;
  border-radius: 10px;
}
    "#
    .to_string()
    .add(&styles::PaneStyle::new(styles::PaneType::Primary).css())
    .into_css();

    html! {
        <div class={ css }>
            <h2>{"Multimedia"}</h2>
            <div id="img-container">
                <div id="corrillo-de-imagenes-movil">
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
        </div>
    }
}

#[derive(Clone, PartialEq, Properties)]
struct NavigationBarProps {}

#[function_component(NavigationBar)]
fn navigation_bar(_props: &NavigationBarProps) -> Html {
    let css = r#"
position: relative;
width: fit-content;
height: fit-content;
display: flex;
flex-direction: column;
gap: 10px;

a {
    text-decoration: none;
}
    "#
    .to_string()
    .add(&styles::PaneStyle::new(styles::PaneType::Primary).css())
    .into_css();

    let callback = Callback::from(|_| ());

    html! {
        <nav-bar class={ css }>
            <a href="/" target="_parent">
                <IconButton icon_id="home.png" label="" onclick={ &callback } />
            </a>
            <a href="#img-container" target="_parent">
                <IconButton icon_id="multimedia.png" label="" onclick={ &callback } />
            </a>
            <a href="" target="_parent">
                <IconButton icon_id="playstore.png" label="" onclick={ &callback } />
            </a>
        </nav-bar>
    }
}
