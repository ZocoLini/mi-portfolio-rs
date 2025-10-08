use crate::components::{IconButton, IconizedItem};
use crate::dyn_data_gen::DynGenerable;
use crate::lang::MultiLang;
use crate::styles::Css;
use crate::{lang, resources, styles};
use frontend::MultiLang;
use serde::Deserialize;
use std::clone::Clone;
use std::collections::HashMap;
use std::convert::From;
use std::fmt::Display;
use std::ops::Add;
use std::string::ToString;
use stylist::css;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct ViewProps {
    pub work_id: String,
}

#[derive(Clone, Deserialize, MultiLang, PartialEq)]
pub struct ViewData {
    name: String,
    image_id: String,
    id: String,
    #[serde(default)]
    is_api: bool,
    technicaldata: Vec<IconizedItemData>,
    features: Vec<IconizedItemData>,
    downloads: Option<Vec<DownloadsItemData>>,
    description: Vec<String>,
    state: Option<HashMap<String, String>>,
    multimedia: Option<MultimediaData>,
    links: Vec<LinkItemData>,
}

#[derive(Clone, Deserialize, MultiLang, PartialEq)]
struct IconizedItemData {
    icon_id: String,
    title: String,
    detail: String,
    link: Option<String>,
}

#[derive(Clone, Deserialize, MultiLang, PartialEq)]
struct LinkItemData {
    icon_id: String,
    title: String,
    url: String,
}

#[derive(Clone, Deserialize, MultiLang, PartialEq)]
struct DownloadsItemData {
    link: String,
    icon: IconizedItemData,
}

#[derive(Deserialize, Clone, PartialEq)]
enum MultimediaType {
    Phone,
    Pc,
}

impl MultiLang for MultimediaType {
    fn translate(self) -> Self {
        self
    }
}

impl Display for MultimediaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            MultimediaType::Phone => "phone",
            MultimediaType::Pc => "pc",
        }
        .to_string();
        write!(f, "{}", str)
    }
}

#[derive(Clone, Deserialize, MultiLang, PartialEq)]
struct MultimediaData {
    r#type: MultimediaType,
    images_ids: Vec<String>,
}

#[function_component(View)]
pub fn view(params: &ViewProps) -> Html {
    let state = use_state(|| None);
    params.generate_dyn_html(state)
}

impl DynGenerable for ViewProps {
    type Data = ViewData;

    fn resouce_id(&self) -> String {
        format!("works/{}", self.work_id)
    }

    fn html_with_data(&self, data: Self::Data) -> Html {
        let css = r#"
display: flex;
margin: 0 auto;
width: 80%;
padding-top: 100px;

h2, h1 {
  text-align: center;
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

  max-width: 450px;
  width: 80%;
}
    "#
        .to_string()
        .into_css();

        // TODO: Remove the necessity of cloning the data. Implement the Components trait manually
        //  to incorporate lifetime into de props

        html! {
            <main class={ css }>
              <div>
                <div id="main-pane">
                  <LeftPane view_data={ data.clone() }/>

                  <center-pane>
                    <Description view_data={ data.clone() }/>
                    <State view_data={ data.clone() }/>
                  </center-pane>
                </div>

                <Multimedia view_data={ data.clone() }/>
              </div>

              <NavigationBar view_data={ data.clone() }/>
            </main>
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
struct LeftPaneProps {
    view_data: ViewData,
}

#[function_component(LeftPane)]
fn left_pane(props: &LeftPaneProps) -> Html {
    let css = r#"
height: fit-content;
min-width: 350px;
max-width: 450px;
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

    let api_icon_css = css!("position: absolute; left: -5px; top: -15px;");

    let cloned_name = props.view_data.name.clone();
    let name = &props.view_data.name;

    html! {
        <left-pane class={ css }>
            <div id="iconoProyecto-container">
              <img id="iconoProyecto" src={ resources::get_work_icon_src(&props.view_data.image_id) } alt={ cloned_name }/>
              if props.view_data.is_api {
                  <img class={ api_icon_css } src={resources::get_icon_src("api.png")} alt="api"/>
              }
            </div>
            <h1>{ name }</h1>
            <Technicaldata view_data={ props.view_data.clone() }/>
            <Features view_data={ props.view_data.clone() }/>

            if props.view_data.downloads.is_some() {
                <Downloads view_data={ props.view_data.clone() }/>
            }
        </left-pane>
    }
}

#[derive(Clone, PartialEq, Properties)]
struct TechnicaldataProps {
    view_data: ViewData,
}

#[function_component(Technicaldata)]
fn metadata(props: &TechnicaldataProps) -> Html {
    let css = r#"
width: 90%;
    "#
    .to_string()
    .add(&styles::PaneStyle::new(styles::PaneType::Secondary).css())
    .into_css();

    let props = props.clone();

    html! {
        <technicaldata class={ css }>
            <h2>{ lang::translate("%work-view.section-title.technicaldata") }</h2>
            {
                for props.view_data.technicaldata.into_iter().map(move |data| {
                    html! {
                        <IconizedItem
                            icon_id={ data.icon_id.clone() }
                            alt_text={ data.icon_id }
                            title={ data.title }
                            detail={ data.detail }
                        />
                    }
                })
            }
        </technicaldata>
    }
}

#[derive(Clone, PartialEq, Properties)]
struct FeaturesProps {
    view_data: ViewData,
}

#[function_component(Features)]
fn features(props: &FeaturesProps) -> Html {
    let css = r#"
width: 90%;
    "#
    .to_string()
    .add(&styles::PaneStyle::new(styles::PaneType::Secondary).css())
    .into_css();

    let props = props.clone();

    html! {
        <features class={ css }>
            <h2>{ lang::translate("%work-view.section-title.features") }</h2>
            {
                for props.view_data.features.into_iter().map(move |feature| {
                    html! {
                        <IconizedItem
                            icon_id={ feature.icon_id.clone() }
                            alt_text={ feature.icon_id }
                            title={ feature.title }
                            detail={ feature.detail }
                        />
                    }
                })
            }
        </features>
    }
}

#[derive(Clone, PartialEq, Properties)]
struct DownloadsProps {
    view_data: ViewData,
}

#[function_component(Downloads)]
fn features(props: &DownloadsProps) -> Html {
    let css = r#"
width: 90%;

p { text-decoration: none;
}
    "#
    .to_string()
    .add(&styles::PaneStyle::new(styles::PaneType::Secondary).css())
    .into_css();

    let link_css = r#"
:hover p {
    color: white;
}

iconized-item:hover {
    background-color: var(--color-button-hover);
}

iconized-item {
    border-radius: 5px;
    padding: 2px;
}
    "#
    .to_string()
    .into_css();

    let props = props.clone();

    html! {
        <downloads class={ css }>
            <h2>{ lang::translate("%work-view.section-title.downloads") }</h2>
            {
                for props.view_data.downloads.unwrap().into_iter().map(move |download| {
                    html! {
                        <a class={ link_css.clone() } href={ download.link } >
                            <IconizedItem
                            icon_id={ download.icon.icon_id.clone() }
                            alt_text={ download.icon.icon_id }
                            title={ download.icon.title }
                            detail={ download.icon.detail }
                            />
                        </a>
                    }
                })
            }
        </downloads>
    }
}

#[derive(Clone, PartialEq, Properties)]
struct DescriptionProps {
    view_data: ViewData,
}

#[function_component(Description)]
fn description(props: &DescriptionProps) -> Html {
    let css = r#"
p {
    padding: 5px 15px;
}
        "#
    .to_string()
    .add(&styles::PaneStyle::new(styles::PaneType::Primary).css())
    .into_css();

    let props = props.clone();

    html! {
        <div class={ css }>
            <h2>{ lang::translate("%work-view.section-title.project-description") }</h2>
            <div>
                {
                    for props.view_data.description.into_iter().map(move |paragraph| {
                        html! { <p>{ paragraph } </p> }
                    })
                }
            </div>
        </div>
    }
}

#[derive(Clone, PartialEq, Properties)]
struct StateProps {
    view_data: ViewData,
}

#[function_component(State)]
fn state(props: &StateProps) -> Html {
    let css = r#"
li {
  padding: 5px 0px 0px 15px;
}
        "#
    .to_string()
    .add(&styles::PaneStyle::new(styles::PaneType::Primary).css())
    .into_css();

    let props = props.clone();

    match props.view_data.state {
        Some(state) => html! {
            <div class={ css }>
                <h2>{ lang::translate("%work-view.section-title.state") }</h2>
                <div>
                  <ul>
                    {
                        for state.iter().map(move |(key, value)| {
                            html! { <li><strong>{ key }</strong>{": "} { value }</li> }
                        })
                    }
                  </ul>
                </div>
            </div>
        },
        None => html! {},
    }
}

#[derive(Clone, PartialEq, Properties)]
struct MultimediaProps {
    view_data: ViewData,
}

#[function_component(Multimedia)]
fn multimedia(props: &MultimediaProps) -> Html {
    let css = r#"
margin: 0 auto;

#img-container {
  width: min(85%, 1200px);
  margin: 0 auto;
}

#imgs-phone {
  display: flex;
  gap: 10px;
  flex-wrap: wrap;
  justify-content: center;
}

#imgs-phone img {
  width: 200px;
  height: auto;
  border-radius: 10px;
}

#imgs-pc {
  justify-content: center;
  display: flex;
  gap: 10px;
  flex-wrap: wrap;
}

#imgs-pc img {
  width: 100%;
  height: auto;
  border-radius: 10px;
}
    "#
    .to_string()
    .add(&styles::PaneStyle::new(styles::PaneType::Primary).css())
    .into_css();

    if props.view_data.multimedia.is_none() {
        return html!();
    }

    let multimedia = props.clone().view_data.multimedia.unwrap();
    let images_ids = multimedia.images_ids;
    let multimedia_type = multimedia.r#type.to_string();
    let work_id = props.view_data.id.clone();

    html! {
        <div class={ css }>
            <h2>{ lang::translate("%work-view.section-title.multimedia") }</h2>
            <div id="img-container">
                <div id={ format!("imgs-{}", multimedia_type)}>
                    {
                        for images_ids.into_iter().map(move |img_id| {
                            html! {
                                <img src={ resources::get_work_image_src(&work_id, &img_id) }
                                    alt={ img_id }/>
                            }
                        })
                    }
                </div>
            </div>
        </div>
    }
}

#[derive(Clone, PartialEq, Properties)]
struct NavigationBarProps {
    view_data: ViewData,
}

#[function_component(NavigationBar)]
fn navigation_bar(props: &NavigationBarProps) -> Html {
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
    let props = props.clone();

    let href = format!("work/{}#img-container", props.view_data.id);

    html! {
        <nav-bar class={ css }>
            <a href="" target="_parent">
                <IconButton icon_id="home.png" label="" onclick={ &callback } />
            </a>
            if props.view_data.multimedia.is_some() {
                <a href={ href } target="_parent">
                    <IconButton icon_id="multimedia.png" label="" onclick={ &callback } />
                </a>
            }
            {
                for props.view_data.links.into_iter().map(move |link| {
                    html! {
                        <a href={ link.url.clone() } target="_parent">
                            <IconButton icon_id={ link.icon_id.clone() } label="" onclick={ &callback } />
                        </a>
                    }
                })
            }
        </nav-bar>
    }
}
