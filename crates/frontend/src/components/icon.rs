use serde::Deserialize;
use yew::{html, Component, Context, Html, Properties};
use crate::lang::MultiLang;

fn get_icon_src(id: &str) -> String
{
    format!("resources/img/icon/{}", id)
}

#[derive(Properties, PartialEq, Clone)]
pub struct IconProps {
    pub id: String,
    pub alt: String,
    pub icon_size: u8,
}

#[derive(Deserialize)]
pub struct Icon {
    id: String,
    #[serde(default)]
    alt: String,
    #[serde(default = "default_icon_size")]
    icon_size: u8
}

fn default_icon_size() -> u8 {
    30
}

impl Icon {
    pub fn html(&self) -> Html {
        let css = format!("height: {}px; width: {}px;", self.icon_size, self.icon_size);
        
        html! {
            <img style={css} src={ get_icon_src(&self.id) } alt={self.alt.clone()} />
        }
    }
}

impl MultiLang for Icon {
    fn translate(self) -> Self {
        self
    }
}

impl Component for Icon {
    type Message = ();
    type Properties = IconProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            id: ctx.props().id.clone(),
            alt: ctx.props().alt.clone(),
            icon_size: ctx.props().icon_size.clone(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        self.html()
    }
}