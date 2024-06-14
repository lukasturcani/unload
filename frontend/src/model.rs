use std::str::FromStr;

use reqwest::Url;

#[derive(Debug)]
pub struct UnloadUrl(pub Url);

impl Default for UnloadUrl {
    fn default() -> Self {
        Self(Url::from_str(&web_sys::window().unwrap().origin()).unwrap())
    }
}
