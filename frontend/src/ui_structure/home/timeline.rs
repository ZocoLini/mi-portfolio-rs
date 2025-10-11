use chrono::{Datelike, NaiveDate};
use frontend::MultiLang;
use serde::Deserialize;
use std::ops::Add;
use stylist::StyleSource;
use yew::{Html, Properties, function_component, html, use_state};

const EVENT_PANE_X_OFFSET: i64 = 150;
const PIXELS_PER_DAY: i64 = 1;
const PANE_WIDTH: i64 = 400;
const PANE_GAP: i64 = 30;

use crate::{
    components::Icon,
    data_gen::{DynGenerable, IntoHtml},
    lang::{self, MultiLang},
    styles::{self, Css},
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
            .num_days()
            * PIXELS_PER_DAY
            - PANE_GAP;

        let css = event_pane_style(height);

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
            .num_days()
            * PIXELS_PER_DAY
            - PANE_GAP;

        let css = event_pane_style(height);

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

fn event_pane_style(height: i64) -> StyleSource {
    format!(
        r#"
        width: {PANE_WIDTH}px;
        height: {height}px;
        border-radius: 20px;
        gap: 15px;
        display: flex;
        padding: 10px;
        overflow-y: scroll;
        background-color: red;

        img
        {{
          width: 50px;
          height: 50px;
          margin-top: 5px;
        }}
    "#
    )
    .add(&styles::primary_text_style_as_string())
    .into_css()
}

#[derive(Deserialize, MultiLang, Clone)]
struct TimelineData {
    jobs: Vec<JobData>,
    formations: Vec<FormationData>,
}

impl TimelineData {
    fn start_date(&self) -> &NaiveDate {
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

    fn end_date(&self) -> &NaiveDate {
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

    fn x_position_of_job(&self, job: &JobData) -> i64 {
        let mut depth = 0;

        for formation in &self.formations {
            if formation.end_date <= job.start_date {
                continue;
            }

            if formation.start_date >= job.end_date {
                continue;
            }

            depth += 1;
        }

        for j in &self.jobs {
            if j == job {
                break;
            }

            if j.end_date <= job.start_date {
                continue;
            }

            if j.start_date >= job.end_date {
                continue;
            }

            depth += 1;
        }

        depth * (PANE_WIDTH + PANE_GAP)
    }

    fn x_position_of_formation(&self, formation: &FormationData) -> i64 {
        let mut depth = 0;

        for f in &self.formations {
            if f == formation {
                break;
            }

            if f.end_date <= formation.start_date {
                continue;
            }

            if f.start_date >= formation.end_date {
                continue;
            }

            depth += 1;
        }

        depth * (PANE_WIDTH + PANE_GAP)
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
        let start_date = data.start_date();
        let end_date = data.end_date();

        let heigth = end_date.signed_duration_since(*start_date).num_days() * PIXELS_PER_DAY + 10;

        let css = format!(
            r#"
            display: flex;
            position: relative;
            height: {heigth}px;
            overflow-y: hidden;
        "#
        )
        .into_css();

        let timeline_dates = TimelineDatesComponent::new(start_date.clone(), end_date.clone());
        let timeline_dates_html = timeline_dates.into_html();

        html!(
            <div>
                <h1>{lang::translate("%timeline.view.title")}</h1>
                <div class={ css }>
                    { timeline_dates_html }
                    {
                        for data.jobs.iter().map(|job|
                            html! {
                                <div style={
                                    format!("position: absolute; left: {}px; top: {}px;",
                                        data.x_position_of_job(job) + EVENT_PANE_X_OFFSET,
                                        job.start_date.signed_duration_since(*start_date).num_days() * PIXELS_PER_DAY
                                    )
                                }>
                                    { job.clone().into_html() }
                                </div>
                            }
                        )
                    }
                    {
                        for data.formations.iter().map(|formation|
                            html! {
                                <div style={
                                    format!("position: absolute; left: {}px; top: {}px;",
                                        data.x_position_of_formation(formation) + EVENT_PANE_X_OFFSET,
                                        formation.start_date.signed_duration_since(*start_date).num_days() * PIXELS_PER_DAY
                                    )
                                }>
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

struct TimelineDatesComponent {
    start_date: chrono::NaiveDate,
    end_date: chrono::NaiveDate,
}

impl TimelineDatesComponent {
    fn new(start_date: chrono::NaiveDate, end_date: chrono::NaiveDate) -> Self {
        TimelineDatesComponent {
            start_date,
            end_date,
        }
    }
}

impl IntoHtml for TimelineDatesComponent {
    fn into_html(self) -> Html {
        let css = format!(
            r#"
        "#
        )
        .into_css();

        // Generar una lista de fechas (un punto por mes)
        let mut dates = vec![];
        let mut current = self.start_date;
        while current <= self.end_date {
            dates.push(current);
            let next_month = if current.month() == 12 {
                NaiveDate::from_ymd_opt(current.year() + 1, 1, 1).unwrap()
            } else {
                NaiveDate::from_ymd_opt(current.year(), current.month() + 1, 1).unwrap()
            };
            current = next_month;
        }

        // Parámetros gráficos del SVG
        let width = EVENT_PANE_X_OFFSET;
        let margin_x = 10.0;
        let mut current_y = 10;
        let r = 5.0;

        // Calcular posiciones Y (distancia según días del mes)
        let mut positions = vec![];
        for date in &dates {
            positions.push((date, current_y));
            let days_in_month = match date.month() {
                1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
                4 | 6 | 9 | 11 => 30,
                2 => {
                    if chrono::NaiveDate::from_ymd_opt(date.year(), 2, 29).is_some() {
                        29
                    } else {
                        28
                    }
                }
                _ => 30,
            };
            current_y += days_in_month * PIXELS_PER_DAY;
        }

        let heigth = self
            .end_date
            .signed_duration_since(self.start_date)
            .num_days()
            * PIXELS_PER_DAY
            + 10;

        html!(
            <div class={ css }>
            <svg
                       viewBox={format!("0 0 {} {}", width, heigth)}
                       style={format!("width:100%; height:{heigth}px; display:block;", )}
                       xmlns="http://www.w3.org/2000/svg"
                   >
                       // Línea base
                       <line
                           x1={margin_x.to_string()}
                           y1="0"
                           x2={margin_x.to_string()}
                           y2={heigth.to_string()}
                           stroke="yellow"
                           stroke-width="3"
                           stroke-linecap="round"
                       />

                       // Puntos de cada mes
                       {
                           for positions.iter().enumerate().map(|(i, (d, y))| {
                               let label = d.format("%b %Y").to_string();
                               html! {
                                   <g key={i}>
                                       <circle
                                           cx={margin_x.to_string()}
                                           cy={y.to_string()}
                                           r={r.to_string()}
                                           fill="green"
                                       >
                                       </circle>
                                       <text
                                           x={(margin_x + 15.0).to_string()}
                                           y={(y + 6).to_string()}
                                           font-size="18"
                                           fill="black"
                                       >
                                           { label }
                                       </text>
                                   </g>
                               }
                           })
                       }
                   </svg>
            </div>
        )
    }
}
