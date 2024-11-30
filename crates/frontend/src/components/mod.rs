pub mod icon;
pub mod loading;

use std::clone::Clone;
use stylist::{css, StyleSource};
use stylist::yew::styled_component;
use yew::prelude::*;
use crate::components::icon::Icon;
use crate::styles::text;

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
                <p class={text::secondary_text_style()}>{ &props.title }</p>
                <p class={text::primary_text_style()}>{ &props.detail }</p>
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
        padding: 10px;
        margin: 5px;
        border-radius: 10px;
        background-color: var(--color-secondary-bkg-pane);
        "#
    );

    html! {
        <icon-link class={css}>
            <a href={props.href.clone()} target="_blank">
                <Icon id={props.icon_id.clone()} alt={props.alt_text.clone()} icon_size={30} />
            </a>
        </icon-link>
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
        border-radius: 10px;
        background-color: var(--color-secondary-bkg-pane);
        padding: 15px;
        margin: 10px;
        cursor: pointer;
        transition: background-color 0.3s, color 0.3s;
        text-align: center;
        width: 50px;
        height: 50px;
        text-decoration: none;
        color: var(--color-primary-text);

        img {
          margin-bottom: 10px;
          filter: brightness(0.2);
        }
        p {
          margin: 0;
          text-align: center;
        }
        /*Al pasar el raaton por encima*/
        :hover {
          background-color: var(--color-button-hover);
        }
        :hover p{
          color: white;
        }
        :hover img{
          filter: brightness(0) invert(1);
        }
        "#
    );

    html! {
        <icon-button onclick={props.onclick.clone()} class={css} >
            <Icon id={props.icon_id.clone()} alt={props.label.clone()} icon_size={30} />
            <p>{ &props.label }</p>
        </icon-button>
    }
}
