use crate::model::Model;
use crate::route::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[component]
pub fn App(cx: Scope) -> Element {
    use_shared_state_provider(cx, Model::default);
    cx.render(rsx! { Router::<Route>{} })
}
