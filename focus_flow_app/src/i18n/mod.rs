use std::collections::HashMap;

use crate::services::storage;

#[derive(Clone, Debug, PartialEq, Default)]
pub enum Locale {
    #[default]
    En,
    It,
}

impl Locale {
    pub fn code(&self) -> &'static str {
        match self {
            Locale::En => "en",
            Locale::It => "it",
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct I18n {
    pub locale: Locale,
    translations: HashMap<String, String>,
}

impl I18n {
    pub fn new(locale: Locale) -> Self {
        let translations = load_translations(&locale);
        Self { locale, translations }
    }

    /// Returns the translated string for `key`, or `key` itself as fallback.
    pub fn t(&self, key: &str) -> String {
        self.translations
            .get(key)
            .cloned()
            .unwrap_or_else(|| key.to_string())
    }

    /// Returns translated string with positional placeholder substitution.
    /// `{0}`, `{1}`, … in the translation are replaced with the corresponding `args`.
    pub fn tf(&self, key: &str, args: &[&str]) -> String {
        let mut s = self.translations
            .get(key)
            .cloned()
            .unwrap_or_else(|| key.to_string());
        for (i, arg) in args.iter().enumerate() {
            s = s.replace(&format!("{{{i}}}"), arg);
        }
        s
    }
}

fn load_translations(locale: &Locale) -> HashMap<String, String> {
    let json = match locale {
        Locale::En => include_str!("../../assets/locales/en.json"),
        Locale::It => include_str!("../../assets/locales/it.json"),
    };
    serde_json::from_str(json).unwrap_or_default()
}

pub fn use_i18n() -> dioxus::prelude::Signal<I18n> {
    dioxus::prelude::use_context::<dioxus::prelude::Signal<I18n>>()
}

pub fn load_locale() -> Locale {
    match storage::get_item("locale").as_deref() {
        Some("it") => Locale::It,
        _ => Locale::En,
    }
}

pub fn save_locale(locale: &Locale) {
    storage::set_item("locale", locale.code());
}
