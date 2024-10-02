use dioxus::prelude::*;
use dioxus_sdk::{i18n::use_i18, translate};

#[component]
pub fn NavBar() -> Element {
    rsx! {
        nav {
            class: "bg-background fixed w-full z-20 top-0 start-0 border-b border-gray-600",
            div {
                class: "max-w-screen-xl flex flex-wrap items-center \
                    justify-between mx-auto p-4",
                Logo {}
                div {
                    class: "flex md:order-2 space-x-3 md:space-x-0 \
                        rtl:space-x-reverse",
                    AppLink {}
                    ToggleSectionListButton {}
                }
                SectionList {}
            }
        }
    }
}

#[component]
fn ToggleSectionListButton() -> Element {
    rsx! {
        button {
            "data-collapse-toggle": "navbar-sticky",
            r#type: "button",
            class: "inline-flex items-center p-2 w-10 h-10 justify-center text-sm rounded-lg md:hidden \
                focus:outline-none focus:ring-2 text-gray-400 hover:bg-gray-700 focus:ring-gray-600",
            aria_controls: "navbar-sticky",
            aria_expanded: "false",
            span { class: "sr-only", "Open main menu" }
            svg {
                class: "w-5 h-5",
                "aria-hidden": "true",
                xmlns: "http://www.w3.org/2000/svg",
                fill: "none",
                view_box: "0 0 17 14",
                path {
                    stroke: "currentColor",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    stroke_width: "2",
                    d: "M1 1h15M1 7h15M1 13h15"
                }
            }
        }
    }
}

#[component]
fn Logo() -> Element {
    rsx! {
        a {
            href: "/",
            class: "flex items-center space-x-3 rtl:space-x-reverse",
            img {
                src: "https://flowbite.com/docs/images/logo.svg",
                class: "h-8",
                alt: "Unload Logo"
            }
            span {
                class: "self-center text-2xl font-semibold whitespace-nowrap text-text-primary",
                "Flowbite"
            }
        }
    }
}

#[component]
fn AppLink() -> Element {
    let i18 = use_i18();
    rsx! {
        a {
            href: translate!(i18, "app_link"),
            class: "cursor-pointer text-white focus:ring-4 focus:outline-none \
                font-medium rounded-lg text-sm px-4 py-2 text-center bg-primary-main \
                hover:bg-primary-dark focus:ring-focus",
            {translate!(i18, "go_to_app")}
        }
    }
}

#[component]
fn SectionList() -> Element {
    rsx! {
        div {
            class: "items-center justify-between hidden w-full md:flex md:w-auto \
                md:order-1",
            id: "navbar-sticky",
            ul {
                class: "flex flex-col p-4 md:p-0 mt-4 font-medium border rounded-lg md:space-x-8 \
                    rtl:space-x-reverse md:flex-row md:mt-0 md:border-0 bg-background-card md:bg-background border-gray-700",
                li { Link { href: "#", label: "Home" } }
                li { Link { href: "#", label: "Feautres" } }
                li { Link { href: "#", label: "Pricing" } }
                li { Link { href: "#", label: "Contact" } }
            }
        }
    }
}

#[component]
fn Link(href: ReadOnlySignal<String>, label: ReadOnlySignal<String>) -> Element {
    rsx! {
        a {
            href,
            class: "block py-2 px-3 text-text-primary rounded hover:bg-grey-700 \
                md:hover:bg-transparent md:hover:text-primary-main \
                hover:text-text-primary md:p-0 border-grey-700",
            {label}
        }
    }
}
