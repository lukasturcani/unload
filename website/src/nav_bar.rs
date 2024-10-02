use dioxus::prelude::*;
use dioxus_html_macro::html;
use dioxus_sdk::{i18n::use_i18, translate};

#[component]
pub fn NavBar() -> Element {
    rsx! {
        nav {
            class: "bg-background fixed w-full z-20 top-0 start-0 border-b border-gray-600",
            div {
                class: "max-w-screen-xl flex flex-wrap items-center justify-between mx-auto p-4",
                Logo {}
                div {
                    class: "flex md:order-2 space-x-3 rtl:space-x-reverse",
                    LanguageSelection {}
                    GitHubLink {}
                    AppLink {}
                    ToggleSectionListButton {}
                }
                SectionList {}
            }
        }
    }
}

#[component]
fn LanguageSelection() -> Element {
    let i18 = use_i18();
    rsx! {
        button {
            "data-dropdown-toggle": "languageDropdown",
            class: "flex items-center justify-between w-full py-2 px-3 rounded md:hover:bg-transparent \
                md:border-0 md:p-0 md:w-auto text-text-primary md:hover:text-hover \
                border-gray-700 hover:bg-gray-700",
            aria_label: "select language",
            {i18.selected_language.read().language.as_str().to_uppercase()}
            svg {
                class:  "w-2.5 h-2.5 ms-2.5",
                "aria-hidden": "true",
                xmlns: "http://www.w3.org/2000/svg",
                fill: "none",
                view_box: "0 0 10 6",
                path {
                    stroke: "currentColor",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    stroke_width: "2",
                    d: "m1 1 4 4 4-4",
                }
            }
            LanguageList {}
        }
    }
}

#[component]
fn LanguageList() -> Element {
    let i18 = use_i18();
    rsx! {
        div {
            id: "languageDropdown",
            class: "z-10 hidden font-normal divide-y rounded-lg shadow w-44 bg-gray-700 divide-gray-600",
            ul {
                class: "py-2 text-sm text-text-primary",
                aria_labelledby: "languageDropdownButton",
                li { LanguageLink { href: "/en", label: "English" } }
                li { LanguageLink { href: "/sk", label: "Slovak" } }
                li { LanguageLink { href: "/ko", label: "Korean" } }
            }
        }
    }
}

#[component]
fn LanguageLink(href: ReadOnlySignal<String>, label: ReadOnlySignal<String>) -> Element {
    rsx! {
        a {
            href,
            class: "block px-4 py-2 hover:bg-gray-600",
            {label}
        }
    }
}

#[component]
fn GitHubLink() -> Element {
    rsx! {
        a {
            href: "https://github.com/lukasturcani/unload",
            target: "_blank",
            class: "text-text-primary hover:text-hover focus:ring-4 focus:outline-none focus:ring-focus font-medium \
                rounded-lg text-sm text-center inline-flex items-center",
            svg {
                "aria-hidden": "true",
                fill: "currentColor",
                height: "36",
                width: "37",
                view_box: "0 0 37 36",
                xmlns: "http://www.w3.org/2000/svg",
                path {
                    d: "m18.5 0c-10.24365 0-18.5 8.21021-18.5 18.2766 0 8.4958 5.90771 15.6887 13.875 17.7234v-3.6767c-5.81738-1.7313-7.37109-6.0863-7.44336-6.2648-.0542-.2142.03613-.4641.25293-.589.1626-.1249.41553-.0892.59619.0892.03613 0 3.16164 3.0343 7.02784 2.9629.2891-.7675.7588-1.535 1.4995-2.0883-4.9683-.7496-8.8706-3.6232-8.8706-8.1567 0-1.8562.77686-3.7302 2.1499-5.2473-.43359-1.7313-.61426-3.85528.1626-5.40808 1.9692.16063 3.9023 1.14229 5.3477 2.01686 1.2104-.28558 2.4931-.49976 3.9023-.49976s2.6919.21418 3.9023.49976c1.4454-.87457 3.3785-1.85623 5.3477-2.01686.7769 1.5528.5601 3.67678.1626 5.40808 1.373 1.5171 2.1499 3.3911 2.1499 5.2473 0 4.5335-3.9023 7.4071-8.8706 8.1567 1.5537 1.1959 1.9331 3.3019 1.9331 4.7834v4.7833c7.9673-2.0347 13.875-9.2276 13.875-17.7234 0-10.06639-8.3105-18.2766-18.5-18.2766z",
                }
            }
            span {
                class: "sr-only",
                "GitHub"
            }
        }
    }
}

#[component]
fn ToggleSectionListButton() -> Element {
    let i18 = use_i18();
    rsx! {
        button {
            "data-collapse-toggle": "navbar-sticky",
            r#type: "button",
            class: "inline-flex items-center p-2 w-10 h-10 justify-center text-sm rounded-lg md:hidden \
                focus:outline-none focus:ring-2 text-gray-400 hover:bg-gray-700 focus:ring-gray-600",
            aria_controls: "navbar-sticky",
            aria_expanded: "false",
            span { class: "sr-only", {translate!(i18, "open_main_menu")} }
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
                "Unload"
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
            class: "cursor-pointer text-primary-contrast-text focus:ring-4 focus:outline-none \
                font-medium rounded-lg text-sm px-4 py-2 text-center bg-primary-main \
                hover:bg-primary-dark focus:ring-focus",
            {translate!(i18, "go_to_app")}
        }
    }
}

#[component]
fn SectionList() -> Element {
    let i18 = use_i18();
    rsx! {
        div {
            class: "items-center justify-between hidden w-full md:flex md:w-auto \
                md:order-1",
            id: "navbar-sticky",
            ul {
                class: "flex flex-col p-4 md:p-0 mt-4 font-medium border rounded-lg md:space-x-8 \
                    rtl:space-x-reverse md:flex-row md:mt-0 md:border-0 bg-background-card md:bg-background border-gray-700",
                li { Link { href: "#home", label: translate!(i18, "home_section_label") } }
                li { Link { href: "#features", label: translate!(i18, "features_section_label") } }
                li { Link { href: "#pricing", label: translate!(i18, "pricing_section_label") } }
                li { Link { href: "#contact", label: translate!(i18, "contact_section_label") } }
            }
        }
    }
}

#[component]
fn Link(href: ReadOnlySignal<String>, label: ReadOnlySignal<String>) -> Element {
    rsx! {
        a {
            href,
            class: "block py-2 px-3 rounded md:hover:bg-transparent md:p-0 md:hover:text-blue-500 text-text-primary hover:bg-gray-700 \
                border-gray-700",
            {label}
        }
    }
}
