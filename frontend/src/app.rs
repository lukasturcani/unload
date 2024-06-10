use crate::commands::{FocusCommand, FocusTarget, ScrollCommand, ScrollTarget};
use crate::model::{
    Board, Model, QuickAddTasks, TagFilter, Tags, Tasks, UnloadUrl, UserFilter, Users,
};
use crate::route::Route;
use crate::themes::THEMES;
use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    use_context_provider(|| Signal::new(Model::default()));
    use_context_provider(|| Signal::new(Board::default()));
    use_context_provider(|| Signal::new(UnloadUrl::default()));
    use_context_provider(|| Signal::new(Tasks::default()));
    use_context_provider(|| Signal::new(Users::default()));
    use_context_provider(|| Signal::new(Tags::default()));
    use_context_provider(|| Signal::new(QuickAddTasks::default()));
    use_context_provider(|| Signal::new(UserFilter::default()));
    use_context_provider(|| Signal::new(TagFilter::default()));
    use_context_provider(|| Signal::new(THEMES[0]));
    use_context_provider(|| Signal::new(ScrollTarget::default()));
    use_context_provider(|| Signal::new(FocusTarget::default()));
    rsx! {
        Router::<Route>{}
        ScrollCommand {}
        FocusCommand {}
    }
}
