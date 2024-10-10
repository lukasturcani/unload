use dioxus::prelude::*;

#[component]
pub fn Card(children: Element) -> Element {
    rsx! {
        div {
            class: "p-6 border rounded-lg shadow bg-background-card border-gray-700",
            {children}
        }
    }
}

#[component]
pub fn ImageCard(children: Element) -> Element {
    rsx! {
        div {
            class: "max-w-sm border rounded-lg shadow \
                bg-background-card border-gray-700",
            a {
                href: "#",
                img {
                    class: "rounded-t-lg",
                    src: "https://flowbite.com/docs/images/blog/image-1.jpg",
                    alt: "",
                }
            }
            div {
                class: "p-5",
                a {
                    href: "#",
                    h5 {
                        class: "mb-2 text-2xl font-bold tracking-tight text-text-primary",
                        "Noteworthy technology acquisitions 2021"
                    }
                }
                p {
                    class: "mb-3 font-normal text-text-secondary",
                    "Here are the biggest enterprise technology acquisitions of 2021 so far, in reverse chronological order."
                }
            }
        }
    }
}
