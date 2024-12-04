use gloo_net::http::Request;
use std::collections::HashMap;
use std::hash::Hash;
use web_sys::wasm_bindgen::__rt::once_cell::sync::OnceCell;
use web_sys::window;

pub trait MultiLang {
    fn translate(self) -> Self;
}

static TRANSLATIONS: OnceCell<HashMap<String, String>> = OnceCell::new();

pub(crate) fn translate(id: &str) -> String {
    if !id.starts_with("%") {
        return id.to_string();
    }

    if let Some(trans) = TRANSLATIONS
        .get()
        .expect("Translations map should be preloaded")
        .get(&id[1..])
    {
        trans.clone()
    } else {
        format!("{}", id)
    }
}

pub async fn load_translations() {
    let locale = get_locale();
    let fetched_data = Request::get(&format!(
        "resources/langs/{}.properties",
        locale.replace("-", "_")
    ))
    .send()
    .await
    .expect("Failed to fetch translations");

    let response = fetched_data
        .text()
        .await
        .expect("Failed to parse translations");
    let _map = parse_translations(response);

    TRANSLATIONS.set(_map).expect("Should set the translations map");
}

fn parse_translations(data: String) -> HashMap<String, String> {
    let mut translations = HashMap::new();

    for line in data.lines() {
        let mut parts = line.splitn(2, '=');

        if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
            translations.insert(key.to_string(), value.to_string());
        }
    }

    translations
}

fn get_locale() -> String {
    let window = window();

    if let Some(window) = window {
        if let Some(locale) = window.navigator().language() {
            return locale;
        }
    }

    "en_US".to_string()
}

impl MultiLang for String {
    fn translate(self) -> Self {
        translate(&self)
    }
}

impl<T> MultiLang for Vec<T> 
where 
    T: MultiLang    
{
    fn translate(self) -> Self {
        self.into_iter().map(|x| x.translate()).collect()
    }
}

impl<T1, T2> MultiLang for (T1, T2) 
where 
    T1: MultiLang,
    T2: MultiLang
{
    fn translate(self) -> Self {
        (self.0.translate(), self.1.translate())
    }
}

impl<T1, T2> MultiLang for HashMap<T1, T2>
where 
    T1: MultiLang + Hash + Eq,
    T2: MultiLang
{
    fn translate(self) -> Self {
        self.into_iter().map(|(k, v)| (k.translate(), v.translate())).collect()
    }
}

impl MultiLang for bool {
    fn translate(self) -> Self {
        self
    }
}