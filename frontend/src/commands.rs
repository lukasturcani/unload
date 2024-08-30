use dioxus::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct ScrollTarget(pub Option<String>);

#[component]
pub fn ScrollCommand() -> Element {
    let scroll_target = use_context::<Signal<ScrollTarget>>();
    let scroll_target = &scroll_target.read().0;
    let scroll = eval(
        r#"
            let elementId = await dioxus.recv();
            if (elementId !== "ignore") {
                document.getElementById(elementId).scrollIntoView({behavior: "smooth"});
            }
        "#,
    );
    if let Some(scroll_target) = scroll_target {
        let _ = scroll.send(scroll_target.clone().into());
    } else {
        let _ = scroll.send("ignore".into());
    }
    rsx! {}
}

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct FocusTarget(pub Option<String>);

#[component]
pub fn FocusCommand() -> Element {
    let focus_target = use_context::<Signal<FocusTarget>>();
    let focus_target = &focus_target.read().0;
    let focus = eval(
        r#"
            let elementId = await dioxus.recv();
            if (elementId !== "ignore") {
                document.getElementById(elementId).focus();
            }
        "#,
    );
    if let Some(focus_target) = focus_target {
        let _ = focus.send(focus_target.clone().into());
    } else {
        let _ = focus.send("ignore".into());
    }
    rsx! {}
}
