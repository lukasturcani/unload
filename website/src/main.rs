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
                class: "font-mono bg-emerald-950 h-screen w-screen flex justify-center items-center",
                div {
                    class: "grid grid-cols-1 place-items-center gap-4",
                    h1 {
                        class: "text-center text-pink-600 text-4xl font-bold",
                        "Welcome to Unload.Life"
                    }
                    a {
                        href: "/app",
                        button {
                            class: "font-mono transition-colors ease-in-out duration-200 animate-bounce hover:bg-pink-600 bg-emerald-950 text-white font-bold py-2 px-4 rounded",
                            "Go to app"
                        }
                    }
                }
            }
        }
    }
}
