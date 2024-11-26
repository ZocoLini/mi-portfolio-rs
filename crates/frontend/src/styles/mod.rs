use std::str::FromStr;
use stylist::{StyleSource};
use stylist::ast::Sheet;

pub mod pane;
pub mod text;

pub trait Css
{
    fn into_css(self) -> StyleSource;
}

impl Css for String {
    fn into_css(self) -> StyleSource {
        StyleSource::from(Sheet::from_str(self.as_str()).unwrap())
    }
}