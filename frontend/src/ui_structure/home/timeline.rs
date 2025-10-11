use chrono::NaiveDate;
use frontend::MultiLang;
use serde::Deserialize;
use std::ops::Add;
use yew::{Html, Properties, function_component, html, use_state};

use crate::{
    components::Icon,
    data_gen::{DynGenerable, IntoHtml},
    lang::{self, MultiLang},
    styles::{self, Css, PaneType},
};

#[function_component(View)]
pub fn view() -> Html {
    let css = r#"

    "#
    .to_string()
    .into_css();

    html! {
        <div class={css}>
            <Works />
        </div>
    }
}

#[derive(Deserialize, MultiLang, Clone, PartialEq)]
struct JobData {
    title: String,
    company: String,
    description: String,
    start_date: NaiveDate,
    end_date: NaiveDate,
    icon: Icon,
}

#[derive(Deserialize, MultiLang, Clone, PartialEq)]
struct FormationData {
    title: String,
    institution: String,
    description: String,
    start_date: NaiveDate,
    end_date: NaiveDate,
    icon: Icon,
}

impl IntoHtml for JobData {
    fn into_html(self) -> Html {
        let height = self
            .end_date
            .signed_duration_since(self.start_date)
            .num_days() - 30;

        let css = format!(
            r#"
            width: 400px;
            height: {height}px;
            display: flex;
            overflow-y: scroll;
            border: 2px solid #ccc;

            img
            {{
              width: 50px;
              height: 50px;
              margin-top: 10px;
              margin-right: 10px;
            }}
        "#
        )
        .add(&styles::PaneStyle::new(PaneType::Secondary).css())
        .add(&styles::primary_text_style_as_string())
        .into_css();

        html!(
        <formation class={ css }>
            { self.icon.html() }
            <div>
                <h3>{ &self.title }</h3>
                <p>{ &self.description }</p>
            </div>
        </formation>
        )
    }
}

impl IntoHtml for FormationData {
    fn into_html(self) -> Html {
        let height = self
            .end_date
            .signed_duration_since(self.start_date)
            .num_days() - 30;

        let css = format!(
            r#"
            width: 400px;
            height: {height}px;
            display: flex;
            overflow-y: scroll;
            border: 2px solid #ccc;

            img
            {{
              width: 50px;
              height: 50px;
              margin-top: 10px;
              margin-right: 10px;
            }}
        "#
        )
        .add(&styles::PaneStyle::new(PaneType::Secondary).css())
        .add(&styles::primary_text_style_as_string())
        .into_css();

        html!(
        <formation class={ css }>
            { self.icon.html() }
            <div>
                <h3>{ &self.title }</h3>
                <p>{ &self.description }</p>
            </div>
        </formation>
        )
    }
}

#[derive(Deserialize, MultiLang, Clone)]
struct TimelineData {
    jobs: Vec<JobData>,
    formations: Vec<FormationData>,
}

impl TimelineData {
    fn first_date(&self) -> &NaiveDate {
        let mut first_date = &self.jobs.first().unwrap().start_date;

        for job in &self.jobs {
            if job.start_date < *first_date {
                first_date = &job.start_date;
            }
        }

        for formation in &self.formations {
            if formation.start_date < *first_date {
                first_date = &formation.start_date;
            }
        }

        first_date
    }

    fn last_date(&self) -> &NaiveDate {
        let mut last_date = &self.jobs.last().unwrap().end_date;

        for job in &self.jobs {
            if job.end_date > *last_date {
                last_date = &job.end_date;
            }
        }

        for formation in &self.formations {
            if formation.end_date > *last_date {
                last_date = &formation.end_date;
            }
        }

        last_date
    }

    fn x_position_of_job(&self, job: &JobData) -> i32 {
        let mut depth = 0;

        for formation in &self.formations {
            if formation.start_date <= job.start_date && formation.end_date >= job.start_date {
                depth += 1;
            }
        }

        for j in &self.jobs {
            if j == job {
                break;
            }

            if j.start_date <= job.start_date && j.end_date >= job.start_date {
                depth += 1;
            }
        }

        depth * 430
    }

    fn x_position_of_formation(&self, formation: &FormationData) -> i32 {
        let mut depth = 0;

        for f in &self.formations {
            if f == formation {
                break;
            }

            if f.start_date <= formation.start_date && f.end_date >= formation.start_date {
                depth += 1;
            }
        }

        depth * 420
    }
}

#[derive(Properties, PartialEq, Clone)]
struct TimelineProps;

#[function_component(Works)]
fn works(props: &TimelineProps) -> Html {
    let state = use_state(|| None);
    props.generate_dyn_html(state)
}

impl DynGenerable for TimelineProps {
    type Data = TimelineData;

    fn resouce_id(&self) -> String {
        "timeline".to_string()
    }

    fn html_with_data(&self, data: Self::Data) -> Html {
        let first_date = data.first_date();
        let last_date = data.last_date();

        let days_diff = last_date.signed_duration_since(*first_date).num_days();

        let css = format!(
            r#"
            display: flex;
            position: relative;
            width: 100vw; height: 100vh;
            height: {days_diff}px;
            overflow-y: hidden;
        "#
        )
        .into_css();

        html!(
            <div>
                <h1>{lang::translate("%timeline.view.title")}</h1>
                <div class={ css }>
                    {
                        for data.jobs.iter().map(|job|
                            html! {
                                <div style={format!("position: absolute; left: {}px; top: {}px;", data.x_position_of_job(job), job.start_date.signed_duration_since(*first_date).num_days())}>
                                    { job.clone().into_html() }
                                </div>
                            }
                        )
                    }
                    {
                        for data.formations.iter().map(|formation|
                            html! {
                                <div style={format!("position: absolute; left: {}px; top: {}px;", data.x_position_of_formation(formation), formation.start_date.signed_duration_since(*first_date).num_days())}>
                                    { formation.clone().into_html() }
                                </div>
                            }
                        )
                    }
                </div>
            </div>
        )
    }
}
