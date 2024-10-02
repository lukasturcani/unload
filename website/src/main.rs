use anyhow::Result;
use dioxus::prelude::*;
use dioxus_logger::tracing::Level;
use dioxus_sdk::{
    i18n::{use_i18, use_init_i18n},
    translate,
};
use dioxus_web::Config;
use nav_bar::NavBar;
use std::{fs, path::PathBuf};
use unic_langid_impl::LanguageIdentifier;

mod nav_bar;
mod translations;

fn main() -> Result<()> {
    #[cfg(feature = "prebuild")]
    {
        let languages = [
            "en".parse::<LanguageIdentifier>()?,
            "sk".parse::<LanguageIdentifier>()?,
            "ko".parse::<LanguageIdentifier>()?,
        ];
        for language in languages {
            let mut path = PathBuf::from(format!("./dist/{}", language));
            fs::create_dir(&path)?;
            path.push("index.html");
            fs::write(path, index_page(language)?)?;
        }
        fs::write(
            "./dist/index.html",
            index_page("en".parse::<LanguageIdentifier>()?)?,
        )?;
    }
    #[cfg(not(feature = "prebuild"))]
    {
        dioxus_logger::init(Level::INFO).expect("failed to init logger");
        let url = web_sys::window().unwrap().location().href().unwrap();
        dioxus_logger::tracing::info!("boo {}", url);
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
    let mut dark = use_signal(|| false);
    let mut mobile = use_signal(|| false);
    use_init_i18n(language.clone(), language, translations::languages);
    let i18 = use_i18();
    rsx! {
        div {
            class: "font-mono min-h-screen min-w-screen text-text-primary flex flex-col scroll-smooth pt-28 sm:pt-20",
            NavBar {}
            div {
                class: "container mx-auto px-3",
                div {
                    class: "flex flex-col items-center gap-4 pb-5",
                    h1 {
                        id: "home",
                        class: "text-2xl sm:text-4xl md:text-5xl lg:text-6xl tracking-tight font-extrabold text-center",
                        {translate!(i18, "h1_main")}
                        br {}
                        span {
                            class: "text-[#ff6dff]",
                            {translate!(i18, "h1_sub")}
                        }
                    }
                    div {
                        class: "flex flex-col sm:flex-row gap-4 w-full sm:w-auto",
                        a {
                            class: "
                                text-xl sm:text-2xl md:text-3xl font-bold text-center
                                w-full sm:w-auto
                                px-5 py-3 sm:py-5
                                text-[#ff6dff]
                                bg-white
                                rounded-md
                                transition-all ease-in-out
                                shadow-xl hover:shadow-md hover:translate-y-1
                            ",
                            href: translate!(i18, "new_board_link"),
                            {translate!(i18, "new_board")}

                        }
                        a {
                            class: "
                                text-xl sm:text-2xl md:text-3xl font-bold text-center
                                w-full sm:w-auto
                                px-5 py-3 sm:py-5
                                bg-[#ff6dff]
                                rounded-md
                                transition-all ease-in-out
                                shadow-xl hover:shadow-md hover:translate-y-1
                            ",
                            href: translate!(i18, "app_link"),
                            {translate!(i18, "go_to_app")}
                        }
                    }
                    div {
                        class: "
                            rounded-xl overflow-hidden mt-9 p-3 max-w-5xl
                            bg-[#ff6dff]
                            flex flex-col items-center justify-center gap-4
                        ",
                        div {
                            class: "flex flex-row gap-2 items-center justify-center",
                            ToggleButton {
                                aria_pressed: dense(),
                                onclick: move |_| {
                                    dense.set(!dense());
                                    let _ = scroll.send("board-image".into());
                                },
                                label: translate!(i18, "dense_button_label")
                            }
                            ToggleButton {
                                aria_pressed: dark(),
                                onclick: move |_| {
                                    dark.set(!dark());
                                    let _ = scroll.send("board-image".into());
                                },
                                label: translate!(i18, "dark_button_label")
                            }
                            ToggleButton {
                                aria_pressed: mobile(),
                                onclick: move |_| {
                                    mobile.set(!mobile());
                                    let _ = scroll.send("board-image".into());
                                },
                                label: translate!(i18, "mobile_button_label")
                            }
                        }
                        div {
                            class: if mobile() { "sm:w-1/2" },
                            figure {
                                class: "rounded-xl overflow-hidden shadow-lg",
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

#[component]
fn ToggleLanguageSelectionButton() -> Element {
    let i18 = use_i18();
    rsx! {
        button {
            class: "text-xl",
            {i18.selected_language.read().language.as_str().to_uppercase()}
        }
    }
}

#[component]
fn ToggleButton(
    onclick: EventHandler<MouseEvent>,
    aria_pressed: bool,
    label: ReadOnlySignal<String>,
) -> Element {
    rsx! {
        button {
            class: "
                px-2 py-1 rounded-md font-bold
                shadow-md sm:hover:shadow sm:hover:translate-y-1
                aria-pressed:shadow aria-pressed:translate-y-1 aria-pressed:bg-[#07fc03]
                bg-white text-[#ff6dff]
                transition-all ease-in-out
                text-3xl
            ",
            aria_pressed,
            onclick: move |event| onclick.call(event),
            {label}
        }
    }
}
