use anyhow::Result;
use buttons::ButtonLink;
use cards::Card;
use dioxus::prelude::*;
use dioxus_sdk::{
    i18n::{use_i18, use_init_i18n},
    translate,
};
use nav_bar::NavBar;
use std::fs;
use toggles::Toggle;
use unic_langid_impl::LanguageIdentifier;

#[cfg(feature = "prebuild")]
use shared_models::{IntoEnumIterator, SupportedLanguage};
#[cfg(feature = "prebuild")]
use std::path::PathBuf;
#[cfg(feature = "prebuild")]
use std::str::FromStr;

#[cfg(not(feature = "prebuild"))]
use dioxus_logger::tracing::Level;
#[cfg(not(feature = "prebuild"))]
use dioxus_web::Config;

use website::buttons;
use website::cards;
use website::nav_bar;
use website::toggles;
use website::translations;

fn main() -> Result<()> {
    #[cfg(feature = "prebuild")]
    {
        for language in SupportedLanguage::iter() {
            let mut path = PathBuf::from(format!("./dist/{}", language.id()));
            fs::create_dir(&path)?;
            path.push("index.html");
            fs::write(
                path,
                index_page(LanguageIdentifier::from_str(language.id()).unwrap())?,
            )?;
        }
        fs::write(
            "./dist/index.html",
            index_page("en".parse::<LanguageIdentifier>()?)?,
        )?;
    }
    #[cfg(not(feature = "prebuild"))]
    {
        dioxus_logger::init(Level::INFO).expect("failed to init logger");
        LaunchBuilder::web()
            .with_cfg(Config::new().hydrate(true))
            .launch(|| {
                rsx! {
                    App{ language: "en".parse::<LanguageIdentifier>().unwrap() }
                }
            });
    }
    Ok(())
}

#[allow(dead_code)]
fn index_page(language: LanguageIdentifier) -> Result<String> {
    let mut vdom = VirtualDom::new_with_props(App, AppProps { language });
    vdom.rebuild_in_place();
    Ok(fs::read_to_string("./dist/index.html")?
        .replace("<!-- REPLACE ME -->", &dioxus::ssr::pre_render(&vdom)))
}

#[component]
fn App(language: LanguageIdentifier) -> Element {
    let scroll = eval(
        r#"
            let elementId = await dioxus.recv();
            if (elementId !== "ignore") {
                document.getElementById(elementId).scrollIntoView({behavior: "smooth"});
            }
        "#,
    );
    let mut dense = use_signal(|| false);
    let mut dark = use_signal(|| true);
    let mut mobile = use_signal(|| false);
    use_future(move || async move {
        {
            web_sys::window()
                .and_then(|window| window.inner_width().ok())
                .and_then(|width| width.as_f64())
                .filter(|&width| width < 640.0)
                .map(move |_| mobile.set(true))
        }
    });
    use_init_i18n(language.clone(), language, translations::languages);
    let i18 = use_i18();
    rsx! {
        div {
            class: "font-mono min-h-screen min-w-screen text-text-primary flex flex-col pt-28 sm:pt-32",
            NavBar {}
            div {
                class: "container mx-auto px-3 max-w-5xl",
                div {
                    class: "flex flex-col items-center gap-10 pb-5",
                    h1 {
                        id: "home",
                        class: "text-2xl sm:text-4xl md:text-5xl md:leading-tight lg:text-6xl lg:leading-tight tracking-tight font-extrabold text-center",
                        {translate!(i18, "h1_main")}
                        br {}
                        span {
                            class: "text-transparent bg-clip-text bg-gradient-to-r to-emerald-600 from-sky-400",
                            {translate!(i18, "h1_sub")}
                        }
                    }
                    div {
                        class: "flex flex-col sm:flex-row gap-4 w-full sm:w-auto",
                        ButtonLink {
                            href: translate!(i18, "new_board_link"),
                            size: buttons::Size::Large,
                            color: buttons::Color::Primary,
                            {translate!(i18, "new_board")}

                        }
                        ButtonLink {
                            href: translate!(i18, "app_link"),
                            size: buttons::Size::Large,
                            color: buttons::Color::Secondary,
                            {translate!(i18, "go_to_app")}
                        }
                    }
                    Card {
                        div {
                            class: "flex flex-row gap-12 items-center justify-center mb-4",
                            Toggle {
                                checked: dense(),
                                label: translate!(i18, "dense_button_label"),
                                on_change: move |_| {
                                    dense.set(!dense());
                                    let _ = scroll.send("board-image".into());
                                },
                            }
                            Toggle {
                                checked: dark(),
                                label: translate!(i18, "dark_button_label"),
                                on_change: move |_| {
                                    dark.set(!dark());
                                    let _ = scroll.send("board-image".into());
                                },
                            }
                            Toggle {
                                checked: mobile(),
                                label: translate!(i18, "mobile_button_label"),
                                on_change: move |_| {
                                    mobile.set(!mobile());
                                    let _ = scroll.send("board-image".into());
                                },
                            }
                        }
                        div {
                            class: "flex flex-row items-center justify-center",
                            div {
                                class: if mobile() { "sm:w-1/2" },
                                figure {
                                    class: "rounded-xl overflow-hidden shadow-lg border border-gray-700",
                                    img {
                                        id: "board-image",
                                        class: "object-contain",
                                        alt: "unload board",
                                        src: match (dense(), dark(), mobile()) {
                                            (true, true, true) => "/dense_dark_mobile.png",
                                            (true, true, false) => "/dense_dark_nmobile.png",
                                            (true, false, true) => "/dense_ndark_mobile.png",
                                            (true, false, false) => "/dense_ndark_nmobile.png",
                                            (false, true, true) => "/ndense_dark_mobile.png",
                                            (false, true, false) => "/ndense_dark_nmobile.png",
                                            (false, false, true) => "/ndense_ndark_mobile.png",
                                            (false, false, false) => "/ndense_ndark_nmobile.png",
                                        },
                                    }
                                }
                            }
                        }
                    }
                }
                h2 {
                    id: "features",
                    {translate!(i18, "features_section_label")}
                }
                h2 {
                    id: "pricing",
                    {translate!(i18, "pricing_section_label")}
                }
                h2 {
                    id: "contact",
                    {translate!(i18, "contact_section_label")}
                }
            }
        }
    }
}
