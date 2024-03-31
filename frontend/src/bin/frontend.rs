use dioxus::prelude::*;
use frontend::app::App;
use log::LevelFilter;

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();
    launch(App);
}
