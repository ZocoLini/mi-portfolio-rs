use crate::resources;
use gloo_net::http::Request;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::RwLock;
use web_sys::window;

pub trait MultiLang {
    fn translate(self) -> Self;
}

static TRANSLATIONS: RwLock<Option<HashMap<String, String>>> = RwLock::new(None);

pub(crate) fn translate(id: &str) -> String {
    if !id.starts_with("%") {
        return id.to_string();
    }

    let translations = TRANSLATIONS.read();

    let translations = if let Ok(res) = translations {
        res
    } else {
        #[cfg(debug_assertions)]
        web_sys::console::log_1(&"Error getting the translations data".into());

        return id.to_string();
    };

    let translations = translations
        .as_ref()
        .expect("Translations map should be preloaded");

    if let Some(trans) = translations.get(&id[1..]) {
        trans.clone()
    } else {
        #[cfg(debug_assertions)]
        {
            web_sys::console::log_1(
                &format!(
                    "Translation not found: {}. Translations loaded: {}",
                    id,
                    translations.len()
                )
                .into(),
            );
        }

        id.to_string()
    }
}

#[allow(unreachable_code)]
pub async fn load_local_translations() {
    #[cfg(debug_assertions)]
    {
        load_translations("en_US".to_string()).await;
        return;
    }

    let locale = get_locale();
    load_translations(locale).await;
}

pub async fn load_translations(locale: String) {
    let mut response = get_translations(&locale).await;

    #[cfg(debug_assertions)]
    {
        web_sys::console::log_1(&format!("Translations: {}", response).into());
    }

    if response.contains("<!doctype html>") {
        response = get_translations("en_US").await.to_string()
    }

    let new_translations = parse_translations(response);

    let old_translations = TRANSLATIONS.write();

    let mut old_translations = if let Ok(res) = old_translations {
        res
    } else {
        return;
    };

    old_translations.replace(new_translations);
}

async fn get_translations(locale: &str) -> String {
    let fetched_data = Request::get(&resources::get_translation_file(locale))
        .send()
        .await
        .expect("Failed to fetch translations");

    fetched_data
        .text()
        .await
        .expect("Failed to get translations text")
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

pub fn get_locale() -> String {
    let window = window();

    if let Some(window) = window {
        if let Some(locale) = window.navigator().language() {
            return locale;
        }
    }

    "en_US".to_string()
}

impl MultiLang for f32 {
    fn translate(self) -> Self {
        self
    }
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
    T: MultiLang,
{
    fn translate(self) -> Self {
        self.map(|x| x.translate())
    }
}
