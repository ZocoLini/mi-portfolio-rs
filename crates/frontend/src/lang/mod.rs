use gloo_net::http::Request;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::RwLock;
use web_sys::{window};

pub trait MultiLang {
    fn translate(self) -> Self;
}

static TRANSLATIONS: RwLock<Option<HashMap<String, String>>> = RwLock::new(None);

pub(crate) fn translate(id: &str) -> String {
    if !id.starts_with("%") {
        return id.to_string();
    }

    let translations = TRANSLATIONS.read();

    let translations = if translations.is_err() {
        return format!("{}", id);
    } else {
        translations.unwrap()
    };

    if let Some(trans) = translations
        .as_ref()
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
    
    let new_translations = parse_translations(response);

    let old_translations = TRANSLATIONS.write();
    
    let mut old_translations = if old_translations.is_err() {
        return;
    } else {
        old_translations.unwrap()
    };
    
    old_translations.replace(new_translations);
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
    T: MultiLang,
{
    fn translate(self) -> Self {
        self.into_iter().map(|x| x.translate()).collect()
    }
}

impl<T1, T2> MultiLang for (T1, T2)
where
    T1: MultiLang,
    T2: MultiLang,
{
    fn translate(self) -> Self {
        (self.0.translate(), self.1.translate())
    }
}

impl<T1, T2> MultiLang for HashMap<T1, T2>
where
    T1: MultiLang + Hash + Eq,
    T2: MultiLang,
{
    fn translate(self) -> Self {
        self.into_iter()
            .map(|(k, v)| (k.translate(), v.translate()))
            .collect()
    }
}

impl MultiLang for bool {
    fn translate(self) -> Self {
        self
    }
}

impl<T> MultiLang for Option<T>
where 
    T: MultiLang
{
    fn translate(self) -> Self {
        self.map(|x| x.translate())
    }
    
}
