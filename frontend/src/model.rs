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
}

impl Default for AppSettings {
    fn default() -> Self {
        let mut settings = AppSettings {
            data: std::marker::PhantomData,
        };
        settings.set_dense(false);
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
