use anyhow::Result;
use dioxus::prelude::*;
use dioxus_logger::tracing::Level;
use dioxus_web::Config;
use std::fs;

fn main() -> Result<()> {
    #[cfg(feature = "prebuild")]
    {
        fs::write("./dist/index.html", index_page()?)?;
    }
    #[cfg(not(feature = "prebuild"))]
    {
        dioxus_logger::init(Level::INFO).expect("failed to init logger");
        LaunchBuilder::web()
            .with_cfg(Config::new().hydrate(true))
            .launch(App);
    }
    Ok(())
}

fn index_page() -> Result<String> {
    let mut vdom = VirtualDom::new(App);
    vdom.rebuild_in_place();
    Ok(fs::read_to_string("./dist/index.html")?
        .replace("<!-- REPLACE ME -->", &dioxus::ssr::pre_render(&vdom)))
}

#[component]
fn App() -> Element {
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
    rsx! {
        div {
            class: "font-mono min-h-screen min-w-screen text-white flex flex-col ",
            div {
                class: "container mx-auto px-3",
                nav {
                    class: "flex flex-row py-5 justify-center sm:justify-between items-center",
                    a {
                        class: "text-2xl font-bold",
                        href: "/",
                        "Unload"
                    }
                    div {
                        class: "
                            hidden sm:flex flex-row gap-4 items-center
                        ",
                        a {
                            href: "https://github.com/lukasturcani/unload",
                            target: "_blank",
                            svg {
                                class: "stroke-white fill-white hover:stroke-[#FFFF00] hover:fill-[#FFFF00]",
                                height: "36",
                                width: "37",
                                "viewBox": "0 0 37 36",
                                xmlns: "http://www.w3.org/2000/svg",
                                path {
                                    d: "m18.5 0c-10.24365 0-18.5 8.21021-18.5 18.2766 0 8.4958 5.90771 15.6887 13.875 17.7234v-3.6767c-5.81738-1.7313-7.37109-6.0863-7.44336-6.2648-.0542-.2142.03613-.4641.25293-.589.1626-.1249.41553-.0892.59619.0892.03613 0 3.16164 3.0343 7.02784 2.9629.2891-.7675.7588-1.535 1.4995-2.0883-4.9683-.7496-8.8706-3.6232-8.8706-8.1567 0-1.8562.77686-3.7302 2.1499-5.2473-.43359-1.7313-.61426-3.85528.1626-5.40808 1.9692.16063 3.9023 1.14229 5.3477 2.01686 1.2104-.28558 2.4931-.49976 3.9023-.49976s2.6919.21418 3.9023.49976c1.4454-.87457 3.3785-1.85623 5.3477-2.01686.7769 1.5528.5601 3.67678.1626 5.40808 1.373 1.5171 2.1499 3.3911 2.1499 5.2473 0 4.5335-3.9023 7.4071-8.8706 8.1567 1.5537 1.1959 1.9331 3.3019 1.9331 4.7834v4.7833c7.9673-2.0347 13.875-9.2276 13.875-17.7234 0-10.06639-8.3105-18.2766-18.5-18.2766z",
                                }
                            }
                        }
                        a {
                            class: "
                                px-5 py-2
                                bg-[#ff6dff]
                                rounded-md
                                transition-all ease-in-out
                                shadow-xl hover:shadow-md hover:translate-y-1
                            ",
                            href: "/app",
                            "Go to app"
                        }
                    }
                }
                div {
                    class: "flex flex-col items-center gap-4 pb-5",
                    p {
                        class: "text-2xl sm:text-4xl md:text-5xl lg:text-6xl tracking-tight font-extrabold text-center",
                        "Shared task managment."
                        br {}
                        span {
                            class: "text-[#ff6dff]",
                            "Simple with no sign ups."
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
                            href: "/new-board",
                            "New board"

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
                            href: "/app",
                            "Go to app"
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
                            button {
                                class: "
                                    px-2 py-1 rounded-md font-bold
                                    shadow-md hover:shadow hover:translate-y-1
                                    aria-pressed:shadow aria-pressed:translate-y-1 aria-pressed:bg-[#FFFF00]
                                    bg-white text-[#ff6dff]
                                    transition-all ease-in-out
                                    text-3xl
                                ",
                                "aria-pressed": dense(),
                                onclick: move |_| {
                                    dense.set(!dense());
                                    scroll.send("board-image".into()).unwrap();
                                },
                                "Dense"
                            }
                            button {
                                class: "
                                    px-2 py-1 rounded-md font-bold
                                    shadow-md hover:shadow hover:translate-y-1
                                    aria-pressed:shadow aria-pressed:translate-y-1 aria-pressed:bg-[#FFFF00]
                                    bg-white text-[#ff6dff]
                                    transition-all ease-in-out
                                    text-3xl
                                ",
                                "aria-pressed": dark(),
                                onclick: move |_| {
                                    dark.set(!dark());
                                    scroll.send("board-image".into()).unwrap();
                                },
                                "Dark"
                            }
                            button {
                                class: "
                                    px-2 py-1 rounded-md font-bold
                                    shadow-md hover:shadow hover:translate-y-1
                                    aria-pressed:shadow aria-pressed:translate-y-1 aria-pressed:bg-[#FFFF00]
                                    bg-white text-[#ff6dff]
                                    transition-all ease-in-out
                                    text-3xl
                                ",
                                "aria-pressed": mobile(),
                                onclick: move |_| {
                                    mobile.set(!mobile());
                                    scroll.send("board-image".into()).unwrap();
                                },
                                "Mobile"
                            }
                        }
                        div {
                            class: if mobile() { "sm:w-1/2" },
                            figure {
                                class: "rounded-xl overflow-hidden shadow-lg",
                                img {
                                    id: "board-image",
                                    class: "object-contain",
                                    alt: "unload large board",
                                    src: match (dense(), dark(), mobile()) {
                                        (true, true, true) => "dense_dark_mobile.png",
                                        (true, true, false) => "dense_dark_nmobile.png",
                                        (true, false, true) => "dense_ndark_mobile.png",
                                        (true, false, false) => "dense_ndark_nmobile.png",
                                        (false, true, true) => "ndense_dark_mobile.png",
                                        (false, true, false) => "ndense_dark_nmobile.png",
                                        (false, false, true) => "ndense_ndark_mobile.png",
                                        (false, false, false) => "ndense_ndark_nmobile.png",
                                    },
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
