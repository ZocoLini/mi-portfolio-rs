use crate::ui_structure::home;

mod components;
mod ui_structure;
mod styles;

fn main() {
    yew::Renderer::<home::View>::new().render();
}
