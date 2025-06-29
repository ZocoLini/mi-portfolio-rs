use crate::styles::Css;
use stylist::StyleSource;

pub fn primary_text_style() -> StyleSource {
    r#"
    color: var(--color-primary-text);
    "#
    .to_string()
    .into_css()
}

pub fn primary_text_style_as_string() -> String {
    r#"
    color: var(--color-primary-text);
    "#
    .to_string()
}

pub fn secondary_text_style() -> StyleSource {
    r#"
    color: var(--color-secondary-text);
    "#
    .to_string()
    .into_css()
}

pub fn secondary_text_style_as_string() -> String {
    r#"
    color: var(--color-secondary-text);
    "#
    .to_string()
}

#[allow(dead_code)]
pub fn tertiary_text_style() -> StyleSource {
    r#"
    color: var(--color-tertiary-text);
    "#
    .to_string()
    .into_css()
}

#[allow(dead_code)]
pub fn tertiary_text_style_as_string() -> String {
    r#"
    color: var(--color-tertiary-text);
    "#
    .to_string()
}
