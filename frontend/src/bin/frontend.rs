use dioxus::prelude::*;
use frontend::app::App;
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
    let origin = Url::from_str(&web_sys::window().unwrap().origin()).unwrap();
    rsx! { App { origin } }
}
