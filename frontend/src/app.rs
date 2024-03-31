use crate::model::Model;
use crate::route::Route;
use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    use_context_provider(|| Signal::new(Model::default()));
    rsx! {
        Router::<Route>{}
    }
}
