use crate::model::{Board, Model, QuickAddTasks, Tags, Tasks, UserFilter, Users};
use crate::route::Route;
use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    use_context_provider(|| Signal::new(Model::default()));
    use_context_provider(|| Signal::new(Board::default()));
    use_context_provider(|| Signal::new(Tasks::default()));
    use_context_provider(|| Signal::new(Users::default()));
    use_context_provider(|| Signal::new(Tags::default()));
    use_context_provider(|| Signal::new(QuickAddTasks::default()));
    use_context_provider(|| Signal::new(UserFilter::default()));
    rsx! {
        Router::<Route>{}
    }
}
