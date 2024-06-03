use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Page {
    Board,
    Archive,
    Users,
    Tags,
}

#[component]
fn NavBar(on: Page) -> Element {
    rsx! {
        nav {
            class: "
                flex flex-row justify-center
                grow-0 shrink-0 w-full h-16
            ",
        }
    }
}
