use gloo_net::http::Request;
use std::collections::HashMap;
use web_sys::wasm_bindgen::__rt::once_cell::unsync::OnceCell;
use web_sys::window;

pub trait MultiLang {
    fn translate(self) -> Self;
}

impl MultiLang for String {
    fn translate(self) -> Self {
        get_string(self)
    }
}

static mut TRANSLATIONS: OnceCell<HashMap<String, String>> = OnceCell::new();

fn get_string(id: String) -> String {
    if !id.starts_with("%") {
        return id;
    }

    unsafe {
        if let Some(trans) = TRANSLATIONS
            .get()
            .expect("Translations map should be preloaded")
            .get(&id[1..])
        {
            trans.clone()
        } else {
            format!("{} traslation not found", id)
        }
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

    unsafe {
        TRANSLATIONS.set(_map).expect("Should set the translations map");
    }
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
