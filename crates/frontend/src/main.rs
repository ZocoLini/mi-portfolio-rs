use crate::ui_structure::home;

mod components;
mod ui_structure;

fn main() {
    yew::Renderer::<home::View>::new().render();
}
