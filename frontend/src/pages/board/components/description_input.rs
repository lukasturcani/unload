use dioxus::prelude::*;

use crate::{
    components::{
        icons::{BulletsIcon, CheckboxIcon},
        tooltip::Tooltip,
    },
    themes::Theme,
};

#[component]
pub fn DescriptionInput(
    id: String,
    editing: Signal<bool>,
    description: ReadOnlySignal<String>,
) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!(
        "rounded-lg border {} {} {}",
        theme.bg_color_2, theme.border_color, theme.focus_color
    );
    let mut enter_pressed = use_signal(|| false);
    let id = use_signal(|| id);
    rsx! {
        div {
            class: format!(
                "flex flex-col gap-2 border p-2 rounded-lg {}",
                theme.border_color,
            ),
            div {
                class: "flex flex-row justify-center items-center gap-2",
                button {
                    class: "group",
                    prevent_default: "onclick",
                    onclick: move |_| insert_string(id, "\n* "),
                    div {
                        class: "relative",
                        div { class: "size-6", BulletsIcon {} }
                        Tooltip { content: "Bullet Points" }
                    }
                }
                button {
                    class: "group",
                    prevent_default: "onclick",
                    onclick: move |_| insert_string(id, "\n- [ ] "),
                    div {
                        class: "relative",
                        div { class: "size-6", CheckboxIcon {} }
                        Tooltip { content: "Task List" }
                    }
                }
            }
            textarea {
                id,
                onmounted: move |event| async move {
                    let _ = event.set_focus(true).await;
                },
                onkeydown: move |event| enter_pressed.set(event.data().key() == Key::Enter),
                oninput: move |_| {
                    if enter_pressed() {
                        spawn(edit_description(id, enter_pressed));
                    }
                },
                rows: 8.max(description.read().lines().count() as i64),
                class: "p-2.5 {style}",
                name: "Description",
                required: false,
                value: description,
            }
        }
    }
}

async fn insert_string(id: Signal<String>, string: impl AsRef<str>) {
    let mut text_data = eval(&format!(
        r#"
            let element = document.getElementById("{id}");
            dioxus.send([element.value, element.selectionStart, element.selectionEnd]);
        "#,
    ));
    let text_data = &text_data.recv().await.unwrap();
    let [content, start_char_index, end_char_index] = &text_data.as_array().unwrap()[..] else {
        panic!("impossible");
    };
    let start_char_index = start_char_index.as_u64().unwrap() as usize;
    let end_char_index = end_char_index.as_u64().unwrap() as usize;
    let mut content = String::from(content.as_str().unwrap());
    let mut char_indices = content.char_indices();
    let start = char_indices
        .nth(start_char_index)
        .map_or(content.len(), |(i, _)| i);
    if start_char_index == end_char_index {
        content.insert_str(start, string.as_ref());
    } else {
        let end = char_indices
            .nth(end_char_index - start_char_index - 1)
            .map_or(content.len(), |(i, _)| i);
        let start = content[..start].rfind('\n').map_or(0, |i| i + 1);
        let mut block = String::new();
        for line in content[start..end].lines() {
            block.push_str(string.as_ref());
            block.push_str(line);
        }
        content.replace_range(start..end, &block);
    };
    let edit = eval(&format!(
        r#"
            let element = document.getElementById("{id}");
            let selectionStart = element.selectionStart + {};
            let content = await dioxus.recv();
            element.value = content;
            element.selectionStart = selectionStart;
            element.selectionEnd = selectionStart;
            element.focus();
        "#,
        string.as_ref().len(),
    ));
    edit.send(content.into()).unwrap();
}

async fn edit_description(id: Signal<String>, mut enter_pressed: Signal<bool>) {
    let mut text_data = eval(&format!(
        r#"
            let element = document.getElementById("{id}");
            dioxus.send([element.value, element.selectionStart]);
        "#,
    ));
    let text_data = &text_data.recv().await.unwrap();
    let [content, char_index] = &text_data.as_array().unwrap()[..] else {
        panic!("impossible");
    };
    let content = content.as_str().unwrap();
    let char_index = char_index.as_u64().unwrap() as usize;
    let byte_index = content
        .char_indices()
        .nth(char_index)
        .map_or(content.len(), |(i, _)| i);
    let start = content[..byte_index - 1].rfind('\n').map_or(0, |i| i + 1);
    let line = &content[start..byte_index - 1];
    if line.starts_with("- [ ]") || line.starts_with("- [x]") {
        let mut content = String::from(content);
        if line == "- [ ] " {
            let edit = eval(&format!(
                r#"
                    let element = document.getElementById("{id}");
                    let selectionStart = element.selectionStart - 7;
                    let content = await dioxus.recv();
                    element.value = content;
                    element.selectionStart = selectionStart;
                    element.selectionEnd = selectionStart;
                "#,
            ));
            content.drain(start..byte_index);
            edit.send(content.into()).unwrap();
        } else {
            let edit = eval(&format!(
                r#"
                    let element = document.getElementById("{id}");
                    let selectionStart = element.selectionStart + 6;
                    let content = await dioxus.recv();
                    element.value = content;
                    element.selectionStart = selectionStart;
                    element.selectionEnd = selectionStart;
                "#,
            ));
            content.insert_str(byte_index, "- [ ] ");
            edit.send(content.into()).unwrap();
        }
    } else if line.starts_with('*') {
        let mut content = String::from(content);
        if line == "* " {
            let edit = eval(&format!(
                r#"
                    let element = document.getElementById("{id}");
                    let selectionStart = element.selectionStart - 3;
                    let content = await dioxus.recv();
                    element.value = content;
                    element.selectionStart = selectionStart;
                    element.selectionEnd = selectionStart;
                "#,
            ));
            content.drain(start..byte_index);
            edit.send(content.into()).unwrap();
        } else {
            let edit = eval(&format!(
                r#"
                    let element = document.getElementById("{id}");
                    let selectionStart = element.selectionStart + 2;
                    let content = await dioxus.recv();
                    element.value = content;
                    element.selectionStart = selectionStart;
                    element.selectionEnd = selectionStart;
                "#,
            ));
            content.insert_str(byte_index, "* ");
            edit.send(content.into()).unwrap();
        }
    }
    enter_pressed.set(false);
}
