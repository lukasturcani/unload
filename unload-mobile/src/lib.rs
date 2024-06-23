use anyhow::Result;
use dioxus::prelude::*;
use frontend::app::App;
use reqwest::Url;
use std::str::FromStr;

#[cfg(target_os = "android")]
fn init_logging() {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Trace),
    );
}

#[cfg(not(target_os = "android"))]
fn init_logging() {
    env_logger::init();
}

#[cfg(any(target_os = "android", target_os = "ios"))]
fn stop_unwind<F: FnOnce() -> T, T>(f: F) -> T {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
        Ok(t) => t,
        Err(err) => {
            eprintln!("attempt to unwind out of `rust` with err: {:?}", err);
            std::process::abort()
        }
    }
}

#[no_mangle]
#[inline(never)]
#[cfg(any(target_os = "android", target_os = "ios"))]
pub extern "C" fn start_app() {
    fn _start_app() {
        stop_unwind(|| main().unwrap());
    }

    #[cfg(target_os = "android")]
    {
        tao::android_binding!(
            com_example,
            dioxus_mobile_test,
            WryActivity,
            wry::android_setup, // pass the wry::android_setup function to tao which will invoke when the event loop is created
            _start_app
        );
        wry::android_binding!(com_example, dioxus_mobile_test);
    }
    #[cfg(target_os = "ios")]
    _start_app()
}

pub fn main() -> Result<()> {
    init_logging();
    dioxus_sdk::storage::set_dir!();
    launch(Unload);
    Ok(())
}

#[component]
fn Unload() -> Element {
    let origin = Url::from_str("https://app.unload.life/").unwrap();
    rsx! { App { origin } }
}
