use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ContactItemProps {
    pub icon_src: String,
    pub alt_text: String,
    pub title: String,
    pub detail: String,
}

#[function_component(ContactItem)]
pub fn contact_item(props: &ContactItemProps) -> Html {
    html! {
        <div class={classes!()}>
            <img class="icon" src={props.icon_src.clone()} alt={props.alt_text.clone()} />
            <div class="vertical-flex">
                <p class="third-text">{ &props.title }</p>
                <p class="second-text">{ &props.detail }</p>
            </div>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct IconLinkProps {
    pub href: String,
    pub icon_src: String,
    pub alt_text: String,
}

#[function_component(IconLink)]
pub fn icon_link(props: &IconLinkProps) -> Html {
    html! {
        <div class="iBox">
            <a href={props.href.clone()} target="_blank">
                <img class="icon" src={props.icon_src.clone()} alt={props.alt_text.clone()} />
            </a>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct IconButtonProps {
    pub icon_src: String,
    pub label: String,
}

#[function_component(IconButton)]
pub fn icon_button(props: &IconButtonProps) -> Html {
    // TODO: Añadir un callback
    html! {
        <a class="pane-button" >
            <img class="icon" src={props.icon_src.clone()} alt={props.label.clone()} />
            <p>{ &props.label }</p>
        </a>
    }
}