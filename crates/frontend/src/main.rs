use crate::ui_structure::home;

mod components;
mod ui_structure;
mod styles;
mod dyn_data_gen;
mod lang;

fn main() {
    yew::Renderer::<home::View>::new().render();
}
