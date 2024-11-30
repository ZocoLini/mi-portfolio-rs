use crate::ui_structure::home;

mod components;
mod dyn_data_gen;
mod lang;
mod styles;
mod ui_structure;

fn main() {
    yew::Renderer::<home::View>::new().render();
}
