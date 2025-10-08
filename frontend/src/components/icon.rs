use crate::lang::MultiLang;
use crate::resources::get_icon;
use crate::styles::Css;
use crate::{lang, styles};
use serde::Deserialize;
use stylist::yew::styled_component;
use stylist::{StyleSource, css};
use web_sys::MouseEvent;
use yew::{Callback, Component, Context, Html, Properties, function_component, html};

#[derive(Properties, PartialEq, Clone)]
pub struct IconProps {
    pub id: String,
    pub alt: String,
    pub icon_size: u8,
}

#[derive(Deserialize, PartialEq, Clone)]
pub struct Icon {
    id: String,
    #[serde(default)]
    alt: String,
    #[serde(default = "default_icon_size")]
    icon_size: u8,
}

fn default_icon_size() -> u8 {
    30
}

impl Icon {
    pub fn html(&self) -> Html {
        let css = format!("height: {}px; width: {}px;", self.icon_size, self.icon_size).into_css();

        html! {
            <img class={ css } src={ get_icon(&self.id) } alt={self.alt.clone()} />
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
            icon_size: ctx.props().icon_size,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        self.html()
    }
}

#[derive(Properties, PartialEq)]
pub struct IconizedItemProps {
    pub icon_id: String,
    pub alt_text: String,
    pub title: String,
    pub detail: String,
}

#[styled_component(IconizedItem)]
pub fn iconized_item(props: &IconizedItemProps) -> Html {
    let css: StyleSource = css!(
        r#"
        display: flex;
        margin: 10px;
        align-items: center;

        p {
            margin: 0 0 0 10px;
        }

        img {
            height: 30px;
            width: 30px;
        }

        "#
    );

    html! {
        <iconized-item class={css}>
            <Icon id={props.icon_id.clone()} alt={props.alt_text.clone()} icon_size={30} />
            <div style="display: flex; flex-direction: column;">
                <p class={styles::tertiary_text_style()}>{ lang::translate(&props.title) }</p>
                <p class={styles::primary_text_style()}>{ lang::translate(&props.detail) }</p>
            </div>
        </iconized-item>
    }
}

#[derive(Properties, PartialEq)]
pub struct IconLinkProps {
    pub href: String,
    pub icon_id: String,
    pub alt_text: String,
}

#[function_component(IconLink)]
pub fn icon_link(props: &IconLinkProps) -> Html {
    let css = css!(
        r#"
        border-radius: 10px;
        background-color: var(--color-secondary-bkg-pane);
        display: flex;
        justify-content: center;
        align-items: center;
        padding: 5px;
        "#
    );

    html! {
        <a class={ css } href={props.href.clone()} target="_blank">
            <Icon id={props.icon_id.clone()} alt={props.alt_text.clone()} icon_size={30} />
        </a>
    }
}

#[derive(Properties, PartialEq)]
pub struct IconButtonProps {
    pub icon_id: String,
    pub label: String,
    pub onclick: Callback<MouseEvent>,
}

#[function_component(IconButton)]
pub fn icon_button(props: &IconButtonProps) -> Html {
    let css = css!(
        r#"
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        border-radius: 10px;
        background-color: var(--color-secondary-bkg-pane);
        padding: 5px;
        cursor: pointer;
        transition: background-color 0.3s, color 0.3s;
        text-align: center;
        width: 50px;
        height: 50px;
        text-decoration: none;
        overflow: hidden;
        color: var(--color-primary-text);

        p {
          margin: 0;
          text-align: center;
          font-size: 12px;
        }

        :hover {
          background-color: var(--color-button-hover);
        }
        :hover p{
          color: white;
        }
        "#
    );

    let label = lang::translate(&props.label);

    html! {
        <icon-button onclick={props.onclick.clone()} class={css} >
            <Icon id={props.icon_id.clone()} alt={props.label.clone()} icon_size={30} />
            if !label.is_empty() {
                <p>{ label }</p>
            }
        </icon-button>
    }
}
