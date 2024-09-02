use dioxus::prelude::*;
use frontend::{app::App, BoardLanguage};
use log::LevelFilter;
use reqwest::Url;
use std::str::FromStr;

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();
    launch(Unload);
}

#[component]
fn Unload() -> Element {
    let window = web_sys::window().unwrap();
    let origin = Url::from_str(&window.origin()).unwrap();
    let default_language = window.navigator().language().unwrap_or(String::from("en"));
    let default_language = default_language
        .split_once('-')
        .unwrap_or((&default_language, ""))
        .0;
    let default_language = BoardLanguage(default_language.into());
    rsx! { App { origin, default_language } }
}
