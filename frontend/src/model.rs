use std::str::FromStr;

use gloo::storage::{LocalStorage, Storage};
use reqwest::Url;

#[derive(Debug)]
pub struct AppSettings {
    data: std::marker::PhantomData<()>,
}

impl AppSettings {
    pub fn set_dense(&mut self, dense: bool) {
        LocalStorage::set("dense", dense).unwrap();
    }

    pub fn dense(&self) -> bool {
        LocalStorage::get("dense").unwrap()
    }

    pub fn set_theme(&mut self, theme: String) {
        LocalStorage::set("theme", theme).unwrap();
    }

    pub fn theme(&self) -> String {
        LocalStorage::get("theme").unwrap()
    }
}

impl AppSettings {
    pub fn new(default_theme: String) -> Self {
        let mut settings = AppSettings {
            data: std::marker::PhantomData,
        };
        if LocalStorage::get::<bool>("dense").is_err() {
            settings.set_dense(false);
        }
        if LocalStorage::get::<String>("theme").is_err() {
            settings.set_theme(default_theme);
        }
        settings
    }
}

#[derive(Debug)]
pub struct UnloadUrl(pub Url);

impl Default for UnloadUrl {
    fn default() -> Self {
        Self(Url::from_str(&web_sys::window().unwrap().origin()).unwrap())
    }
}
