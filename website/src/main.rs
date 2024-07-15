use dioxus::prelude::*;
use fs_extra::dir::CopyOptions;
use std::{error::Error, fs, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    if Path::new("./dist").exists() {
        fs::remove_dir_all("./dist")?;
    }
    let copy_options = CopyOptions::new().copy_inside(true);
    fs_extra::copy_items(&["./assets"], "./dist", &copy_options)?;
    fs::write("./dist/index.html", index_page())?;
    Ok(())
}

fn index_page() -> String {
    format!(
        "<!DOCTYPE html><html>{}<!html>",
        dioxus_ssr::render_element(App())
    )
}

#[component]
fn App() -> Element {
    rsx! {
        head {
            title { "Unload.Life | Shared Clarity" }
            meta {
                content: "text/html;charset=utf-8",
                "http-equiv": "Content-Type",
            }
            meta {
                name: "viewport",
                content: "width=device-width, initial-scale=1",
            }
            meta {
                charset: "UTF-8",
            }
            link {
                rel: "stylesheet",
                href: "/tailwind.css",
            }
        }
        body {
            div {
                class: "font-mono bg-indigo-600 h-screen w-screen text-white flex flex-col ",
                div {
                    class: "container mx-auto",
                    nav {
                        class: "flex flex-row py-5 justify-between items-center",
                        a {
                            class: "text-2xl font-bold",
                            href: "/",
                            "Unload"
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
                    div {
                        class: "grid grid-cols-1 place-items-center gap-4",
                        p {
                            class: "text-6xl tracking-tight font-extrabold text-center",
                            "Shared task managment."
                            br {}
                            span {
                                class: "text-[#ff6dff]",
                                "Simple with no sign ups."
                            }
                        }
                        a {
                            class: "
                                text-3xl
                                p-5
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
            }
        }
    }
}
