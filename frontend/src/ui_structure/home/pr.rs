use yew::{Html, function_component, platform::spawn_local, use_state};

use gloo_net::http::Request;
use serde::Deserialize;
use std::{collections::BTreeMap, ops::Add};
use web_sys::window;
use yew::{Callback, html};

use crate::{
    components::{self, Icon, IconButton}, data_gen::IntoHtml, lang, styles::{self, Css, PaneType}, HttpReqState
};

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Owner {
    login: String,
    avatar_url: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct GithubProject {
    name: String,
    description: String,
    url: String,
    owner: Owner,
}

impl IntoHtml for GithubProject {
    fn into_html(self) -> yew::Html {
        let css = r#"
                min-width: 250px;
                width: 30%;
                max-width: 30%;
                height: 88%;
                display: flex;
                position: relative;
                flex-direction: column;
                border-radius: 10px;

                p, div {
                  margin-left: 20px;
                }
                p {
                    flex: 1;
                }
            "#
        .to_string()
        .add(&styles::PaneStyle::new(PaneType::Secondary).css())
        .add(&styles::primary_text_style_as_string())
        .into_css();

        let header_css = r#"
                display: flex;
                flex-direction: row;
                align-items: center;
                gap: 30px;

                img {
                    width: 50px;
                    height: 50px;
                }

                h2 {
                    flex: 1;
                }
            "#
        .to_string()
        .into_css();

        let bottom_css = r#"
                display: flex;
                flex-direction: row;
                justify-content: flex-end;

            "#
        .to_string()
        .into_css();

        let on_click = {
            let url = self.url.replace("api.github.com/repos/", "github.com/");
            Callback::from(move |_| {
                if let Some(win) = window() {
                    let _ = win.open_with_url_and_target(&url, "_blank");
                }
            })
        };

        html!(
          <div class={ css }>
            <div class={ header_css }>
              <img src={self.owner.avatar_url} alt={self.owner.login}/>
              <h2>{ &self.name }</h2>
            </div>
            <p> { &self.description } </p>
            <div class={ bottom_css }>
                <IconButton icon_id="link.png" label="" onclick={ on_click }/>
            </div>
          </div>
        )
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct CollaboratedProjects {
    items: Vec<(GithubProject, MadePrs)>,
}

impl IntoHtml for CollaboratedProjects {
    fn into_html(self) -> yew::Html {
        let css = r#"
            min-width: 500px;
            display: flex;
            flex-direction: column;
            gap: 20px;
            align-items: top;

            .project-entry {
                height: 250px;
                display: flex;
                flex-direction: row;
                gap: 20px;
                flex-wrap: nowrap;
            }
        "#
        .to_string()
        .into_css();

        html!(
            <div class={ css }>
                    {
                      for self.items.into_iter().map(move |(project, prs)|
                        html! {
                            <div class="project-entry">
                                { project.into_html() }
                                { prs.into_html() }
                            </div>
                        }
                      )
                    }
            </div>
        )
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Pr {
    #[serde(rename = "url")]
    pr_url: String,
    repository_url: String,
    title: String,
}

impl IntoHtml for Pr {
    fn into_html(self) -> yew::Html {
        let css = r#"
            display: flex;
            gap: 10px;
            flex: 1;
            max-height: 25%;
            min-width: 300px;
            align-items: center;
            border-radius: 20px;
            overflow: hidden;
        "#
        .to_string()
        .add(&styles::PaneStyle::new(PaneType::Secondary).css())
        .into_css();

        let on_click = {
            let url = self.pr_url.replace("api.github.com/repos/", "github.com/");
            Callback::from(move |_| {
                if let Some(win) = window() {
                    let _ = win.open_with_url_and_target(&url, "_blank");
                }
            })
        };

        html!(
            <div class={ css }>
                <Icon id="pr.png" alt="Pull Request" icon_size=50/>
                <div style="flex: 1;">
                    <p>{self.title}</p>
                </div>
                <IconButton icon_id="link.png" label="" onclick={ on_click }/>
            </div>
        )
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct MadePrs {
    items: Vec<Pr>,
}

impl IntoHtml for MadePrs {
    fn into_html(self) -> yew::Html {
        let css = r#"
            display: flex;
            flex: 1;
            flex-direction: column;
            justify-content: flex-start;
            overflow-y: scroll;
        "#
        .to_string()
        .into_css();

        html!(
            <div class={ css }>
            {
              for self.items.into_iter().map(move |pr|
                  pr.into_html()
              )
            }
            </div>
        )
    }
}

pub async fn get_collaborated_projects() -> HttpReqState<CollaboratedProjects> {
    let url = "https://api.github.com/search/issues?q=type:pr+author:ZocoLini";

    let response = match Request::get(url).send().await {
        Ok(response) => response,
        Err(_) => return HttpReqState::None,
    };

    let mut prs = match response.json::<MadePrs>().await {
        Ok(data) => data,
        Err(_) => return HttpReqState::None,
    };

    prs.items.retain(|pr| {
        !pr.repository_url
            .starts_with("https://api.github.com/repos/ZocoLini")
    });

    let mut pr_per_project: BTreeMap<String, MadePrs> = BTreeMap::new();

    for pr in prs.items {
        let repo_url = pr.repository_url.clone();

        match pr_per_project.get_mut(&repo_url) {
            Some(prs) => prs.items.push(pr),
            None => {
                pr_per_project.insert(repo_url, MadePrs { items: vec![pr] });
            }
        };
    }

    let mut collaborated_projects = CollaboratedProjects { items: Vec::new() };

    for (repo_url, prs) in pr_per_project.into_iter().rev() {
        let response = match Request::get(&repo_url).send().await {
            Ok(response) => response,
            Err(_) => continue,
        };

        let project = match response.json::<GithubProject>().await {
            Ok(data) => data,
            Err(_) => continue,
        };

        collaborated_projects.items.push((project, prs));
    }

    HttpReqState::Loaded(collaborated_projects)
}

#[function_component(View)]
pub fn view() -> Html {
    let prs = use_state(|| HttpReqState::Loading);

    if HttpReqState::Loading == *prs {
        let state = prs.clone();
        spawn_local(async move {
            #[cfg(debug_assertions)]
            web_sys::console::log_1(&"Loading prs".into());

            let prs = get_collaborated_projects().await;
            state.set(prs);
        });
    }

    html! {
        <div>
        <h1>{lang::translate("%pr.view.title")}</h1>
        {
            match *prs {
                HttpReqState::Loading => {
                    html! {
                        <components::LoadingSpinner/>
                    }
                }
                HttpReqState::Loaded(ref ps) => {
                    html! {
                        { ps.clone().into_html() }
                    }
                }
                HttpReqState::None => {
                    html! {
                        <p>{ "No external PRs found" }</p>
                    }
                }
            }
        }
        </div>
    }
}
