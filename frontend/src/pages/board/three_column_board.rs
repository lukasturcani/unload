use dioxus::prelude::*;
use dioxus_sdk::{i18n::*, translate};
use itertools::Itertools;
use shared_models::{BoardName, SavedBoard, TaskStatus};

use crate::{
    commands::FocusTarget,
    components::{
        form::{CancelButton, ConfirmButton},
        icons::{
            BarsIcon, BookmarkIcon, CircledPlusIcon, DoneIcon, EditIcon, InProgressIcon,
            LanguageIcon, SparklesIcon, StackIcon, ToDoIcon, TrashIcon,
        },
        input::TextInput,
        nav::NavBar,
        tooltip::Tooltip,
    },
    model::{BoardLanguage, SavedBoards, Welcome},
    pages::board::{
        components::{
            ChatGpt, DenseTask, FilterBarTagIcon, FilteringUserIcon, NewTaskForm, Task, ThemeButton,
        },
        model::{
            task_filter, Board, ChatGptResponse, Dense, TagFilter, Tags, Tasks, UserFilter, Users,
        },
        requests::{self, BoardSignals},
    },
    route::Route,
    themes::Theme,
    translations::{translations, Translation},
};

#[derive(Clone, Copy, Eq, PartialEq)]
enum Panel {
    None,
    Boards,
    ChatGpt {
        status: TaskStatus,
        adding_task: Signal<bool>,
    },
    LanguagePicker,
}

#[component]
pub fn ThreeColumnBoard(board_name: BoardName) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!("{} {}", theme.text_color, theme.bg_color_1);
    let show_themes = use_signal(|| false);
    let mut panel = use_signal(|| Panel::None);
    let adding_to_do = use_signal(|| false);
    let adding_in_progress = use_signal(|| false);
    let adding_done = use_signal(|| false);
    let mut welcome = use_context::<Signal<Welcome>>();
    if *welcome.read() == Welcome::True {
        welcome.set(Welcome::False);
        panel.set(Panel::ChatGpt {
            status: TaskStatus::ToDo,
            adding_task: adding_to_do,
        });
    }
    rsx! {
        div {
            class: "flex flex-col h-dvh w-screen {style}",
            Header {
                body: rsx!{
                    div {
                        class: "flex flex-row gap-2 w-24",
                        ToggleBoardsPanelButton { panel }
                    }
                    Title {}
                    div {
                        class: "flex flex-row gap-2 w-24",
                        LanguageButton { panel }
                        DenseButton {}
                        ToggleThemesButton { show_themes }
                    }
                }
            }
            div {
                class: "grow flex flex-col gap-2 overflow-y-auto p-4 pb-2",
                div {
                    class: "grow w-full h-full overflow-y-auto",
                    div {
                        class: "w-full h-full grid grid-cols-3 gap-2 overflow-y-auto",
                        Column { adding_task: adding_to_do, status: TaskStatus::ToDo, panel }
                        Column { adding_task: adding_in_progress, status: TaskStatus::InProgress, panel }
                        Column { adding_task: adding_done, status: TaskStatus::Done, panel }
                    },
                }
                if show_themes() {
                    ThemesBar {}
                }
                FilterBar {}
            }
            NavBar { board_name }
        }
        match panel() {
            Panel::None => rsx! {},
            Panel::Boards => rsx! { BoardPopup { panel } },
            Panel::ChatGpt{status, adding_task} => rsx! { ChatGptPopup { status, adding_task, panel } },
            Panel::LanguagePicker => rsx! { LanguagePickerPopup { panel } },
        }
    }
}

#[component]
fn Title() -> Element {
    let editing = use_signal(|| false);
    rsx! {
        if editing() {
            TitleInput { editing }
        } else {
            TitleShow { editing }
        }
    }
}

#[component]
fn TitleInput(editing: Signal<bool>) -> Element {
    let i18 = use_i18();
    let board = use_context::<Signal<Board>>();
    let board_signals = BoardSignals::default();

    let title = use_memo(move || {
        let board = board.read();
        board.title.clone()
    });
    let title = ReadOnlySignal::from(title);
    let input_label = translate!(i18, "task_title_input_label");
    rsx! {
        form {
            aria_label: translate!(i18, "board_title_update_form_label"),
            class: "grow flex flex-row gap-2 items-center justify-center",
            onsubmit: move |event| {
                let title = event.values()[&input_label].as_value();
                spawn_forever(requests::set_board_title(board_signals, title));
                editing.set(false);
            },
            TextInput {
                id: "board-title-input",
                label: input_label.clone(),
                value: title,
            }
            ConfirmButton { label: translate!(i18, "set_board_title_button_label") }
            CancelButton {
                label: translate!(i18, "cancel_board_title_update_button_label"),
                editing,
            }
        }
    }
}

#[component]
fn TitleShow(editing: Signal<bool>) -> Element {
    let board = use_context::<Signal<Board>>();
    let board = board.read();
    rsx! {
        div {
            class: "grow flex flex-col items-center justify-center pb-1",
            div {
                class: "flex flex-row items-center justify-center gap-2",
                h1 {
                    class: "text-2xl font-extrabold",
                    {board.title.clone()}
                }
                EditTitleButton { editing }
            }
            p { "{board.board_name}" }
        }
    }
}

#[component]
fn EditTitleButton(editing: Signal<bool>) -> Element {
    let i18 = use_i18();
    rsx! {
        div {
            class: "group relative",
            button {
                aria_label: translate!(i18, "edit_board_title_tooltip"),
                class: "block size-6",
                onclick: move |_| editing.set(true),
                EditIcon {}
            }
            Tooltip {
                content: translate!(i18, "edit_board_title_tooltip"),
                position: "",
            }
        }
    }
}

#[component]
fn ThemesBar() -> Element {
    let i18 = use_i18();
    let themes = use_context::<Signal<Vec<Theme>>>();
    rsx! {
        section {
            class: "flex flex-row gap-2 items-center",
            h2 {
                class: "text-xl",
                {format!("{}:", translate!(i18, "themes_section_label"))}
            }
            div {
                class: "flex flex-row overflow-x-auto gap-2",
                for theme in themes.read().iter() {
                    ThemeButton { theme: *theme }
                }
            }
        }
    }
}

#[component]
fn FilterBar() -> Element {
    let i18 = use_i18();
    let tags = use_context::<Signal<Tags>>();
    let tags = &tags.read().0;
    let users = use_context::<Signal<Users>>();
    let users = &users.read().0;
    rsx! {
        section {
            aria_label: translate!(i18, "filters_section_label"),
            class: "grid grid-cols-5 gap-1",
            div {
                class: "col-span-4 flex flex-row flex-wrap items-center gap-1",
                for tag_id in tags.keys().sorted_by_key(|tag_id| tags[tag_id].name.to_lowercase())
                {
                    FilterBarTagIcon {
                        tag_id: *tag_id,
                        tag_data: tags[tag_id].clone(),
                    }
                }
            }
            div {
                class: "col-span-1 flex flex-row flex-wrap items-center gap-1",
                for user_id in users.keys().sorted_by_key(|user_id| users[user_id].name.to_lowercase())
                {
                    FilteringUserIcon {
                        user_id: *user_id,
                        user_data: users[user_id].clone(),
                        size: "size-6",
                        dir: "rtl",
                        tooltip_position: "-top-11",
                    }
                }
            }
        }
    }
}

#[component]
fn LanguageButton(panel: Signal<Panel>) -> Element {
    let i18 = use_i18();
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!("border-2 rounded {}", theme.button);
    rsx! {
        div {
            class: "group relative",
            button {
                aria_label: translate!(i18, "pick_language_tooltip"),
                class: "size-9 p-1 {style}",
                aria_pressed: panel() == Panel::LanguagePicker,
                onclick: move |_| panel.set(Panel::LanguagePicker),
                {i18.selected_language.read().language.as_str().to_uppercase()}
            }
            Tooltip {
                content: translate!(i18, "pick_language_tooltip"),
                position: "right-0",
            }
        }
    }
}

#[component]
fn LanguagePickerPopup(panel: Signal<Panel>) -> Element {
    rsx! {
        div {
            class: "
                backdrop-blur-sm backdrop-brightness-50
                size-full absolute inset-0 z-10
                flex flex-row items-center justify-center
            ",
            onclick: move |_| panel.set(Panel::None),
            LanguageList { panel }
        }
    }
}

#[component]
fn DenseButton() -> Element {
    let i18 = use_i18();
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!("border-2 rounded {}", theme.button);
    let mut dense = use_context::<Signal<Dense>>();
    rsx! {
        div {
            class: "group relative",
            button {
                aria_label: translate!(i18, "toggle_dense_view_tooltip"),
                class: "size-9 p-1 {style}",
                aria_pressed: dense.read().0,
                onclick: move |_| {
                    let new_dense = !dense.read().0;
                    dense.set(Dense(new_dense));
                },
                StackIcon {}
            }
            Tooltip {
                content: translate!(i18, "toggle_dense_view_tooltip"),
                position: "right-0",
            }
        }
    }
}

#[component]
fn ToggleThemesButton(show_themes: Signal<bool>) -> Element {
    let i18 = use_i18();
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!("border-2 rounded {}", theme.button);
    rsx! {
        div {
            class: "group relative",
            button {
                aria_label: translate!(i18, "toggle_show_themes_tooltip"),
                class: "size-9 p-1 {style}",
                "aria-pressed": show_themes(),
                onclick: move |_| {
                    show_themes.set(!show_themes());
                },
                SparklesIcon {}
            }
            Tooltip {
                content: translate!(i18, "toggle_show_themes_tooltip"),
                position: "right-0",
            }
        }
    }
}

#[component]
fn Header(body: Element) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!(
        "
        border-b {}
        ",
        theme.border_color
    );
    rsx! {
        header {
            class: "
                flex flex-row items-center
                w-full h-14 shrink-0 grow-0
                pl-5 pr-24
                {style}
            ",
            {body}
        }
    }
}

#[component]
fn ColumnHeading(value: String) -> Element {
    let style = "text-3xl font-extrabold";
    rsx! {
        h2 { class: style, {value} }
    }
}

#[component]
fn Column(adding_task: Signal<bool>, status: TaskStatus, panel: Signal<Panel>) -> Element {
    let i18 = use_i18();
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!("border {}", theme.border_color);
    let dense = use_context::<Signal<Dense>>().read().0;
    let gap = if dense { "" } else { "gap-2" };
    rsx! {
        section {
            class: "flex flex-col overflow-y-auto px-2 pt-2 gap-2 {style}",
            div {
                class: "flex items-center gap-2",
                match status {
                    TaskStatus::ToDo => rsx! {
                        div { class: "size-8 stroke-red-600", ToDoIcon {} }
                        ColumnHeading { value: translate!(i18, "to_do_column_title") }
                    },
                    TaskStatus::InProgress => rsx! {
                        div { class: "size-8 stroke-fuchsia-600", InProgressIcon {} }
                        ColumnHeading { value: translate!(i18, "in_progress_column_title") }
                    },
                    TaskStatus::Done => rsx! {
                        div { class: "size-8 stroke-green-500", DoneIcon {} }
                        ColumnHeading { value: translate!(i18, "done_column_title") }
                    }
                }
            }
            div {
                class: "grow flex flex-col {gap} overflow-y-scroll pt-2",
                if dense {
                    DenseColumnTasks { status }
                } else {
                    ColumnTasks { status }
                }
                if adding_task() {
                    NewTaskForm { status, adding_task }
                }
            }
            AddTaskButton { status, adding_task, panel }
        }
    }
}

#[component]
fn ColumnTasks(status: TaskStatus) -> Element {
    let tasks = use_context::<Signal<Tasks>>();
    let tasks = &tasks.read().0;
    let board = use_context::<Signal<Board>>();
    let board = board.read();
    let user_filter = use_context::<Signal<UserFilter>>();
    let user_filter = &user_filter.read().0;
    let tag_filter = use_context::<Signal<TagFilter>>();
    let tag_filter = &tag_filter.read().0;
    let column_tasks = match status {
        TaskStatus::ToDo => &board.to_do,
        TaskStatus::InProgress => &board.in_progress,
        TaskStatus::Done => &board.done,
    };
    rsx! {
        for (task_id, task) in column_tasks
            .iter()
            .filter(|task_id| {
                task_filter(task_id, tasks, user_filter, tag_filter)
            })
            .map(|task_id| (*task_id, &tasks[task_id]))
        {
            Task {
                key: "{task_id}",
                task_id,
                title: task.title.clone(),
                description: task.description.clone(),
                status,
                assignees: task.assignees.clone(),
                tags: task.tags.clone(),
                due: task.due,
            }
        }
    }
}

#[component]
fn DenseColumnTasks(status: TaskStatus) -> Element {
    let tasks = use_context::<Signal<Tasks>>();
    let tasks = &tasks.read().0;
    let board = use_context::<Signal<Board>>();
    let board = board.read();
    let user_filter = use_context::<Signal<UserFilter>>();
    let user_filter = &user_filter.read().0;
    let tag_filter = use_context::<Signal<TagFilter>>();
    let tag_filter = &tag_filter.read().0;
    let column_tasks = match status {
        TaskStatus::ToDo => &board.to_do,
        TaskStatus::InProgress => &board.in_progress,
        TaskStatus::Done => &board.done,
    };
    rsx! {
        for (task_id, task) in column_tasks
            .iter()
            .filter(|task_id| {
                task_filter(task_id, tasks, user_filter, tag_filter)
            })
            .map(|task_id| (*task_id, &tasks[task_id]))
        {
            DenseTask {
                key: "{task_id}",
                task_id,
                title: task.title.clone(),
                description: task.description.clone(),
                status,
                assignees: task.assignees.clone(),
                tags: task.tags.clone(),
                due: task.due,
            }
        }
    }
}

#[component]
fn ToggleBoardsPanelButton(panel: Signal<Panel>) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!("border-2 rounded {}", theme.button);
    rsx! {
        button {
            class: "size-9 p-1 {style}",
            aria_pressed: panel() == Panel::Boards,
            onclick: move |event| {
                event.stop_propagation();
                if panel() == Panel::Boards {
                    panel.set(Panel::None)
                } else {
                    panel.set(Panel::Boards)
                }
            },
            BarsIcon {}
        }
    }
}

#[component]
fn BoardPopup(panel: Signal<Panel>) -> Element {
    rsx! {
        div {
            class: "
                backdrop-blur-sm backdrop-brightness-50
                size-full absolute inset-0 z-10
                flex flex-row items-center justify-center
            ",
            onclick: move |_| panel.set(Panel::None),
            BoardList { panel }
        }
    }
}

#[component]
fn LanguageList(panel: Signal<Panel>) -> Element {
    let i18 = use_i18();
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!("rounded-lg {} {}", theme.text_color, theme.bg_color_2);
    rsx! {
        section {
            onclick: move |event| event.stop_propagation(),
            class: "max-h-96 overflow-y-auto px-3 py-5 flex flex-col gap-2 w-1/2 {style}",
            h2 {
                class: "
                    px-2
                    font-bold text-xl
                    flex flex-row gap-1 items-center
                ",
                div { class: "size-5", LanguageIcon {} }
                {translate!(i18, "languages_section_title")}
            }
            ul {
                class: "flex flex-col",
                for translation in translations()
                {
                    LanguageListItem { key: "{translation.id}", translation, panel }
                }
            }
        }
    }
}

#[component]
fn LanguageListItem(translation: Translation<&'static str>, panel: Signal<Panel>) -> Element {
    let mut language = use_context::<Signal<BoardLanguage>>();
    let mut i18 = use_i18();
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!("first:border-t border-b {}", theme.border_color);
    rsx! {
        li {
            class: style,
            button {
                class: format!("
                    flex flex-row justify-between items-center
                    px-2
                    size-full rounded-lg
                    group
                    {}
                ", theme.hover_color),
                onclick: move |_| {
                    language.set(BoardLanguage(translation.id.to_string()));
                    i18.set_language(translation.id.parse().unwrap());
                    panel.set(Panel::None);
                },
                {translation.name}
            }
        }
    }
}

#[component]
fn ChatGptPopup(status: TaskStatus, adding_task: Signal<bool>, panel: Signal<Panel>) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = theme.text_color;
    rsx! {
        div {
            class: "
                backdrop-blur-sm backdrop-brightness-50
                size-full absolute inset-0 z-10 p-10
                flex flex-row items-center justify-center
                {style}
            ",
            onclick: move |_| panel.set(Panel::None),
            section {
                aria_label: "chat gpt",
                class: "w-2/3 max-h-full overflow-y-auto",
                onclick: |event| event.stop_propagation(),
                ChatGptContainer { status, adding_task, panel }
            }
        }
    }
}

#[component]
fn ChatGptContainer(
    status: TaskStatus,
    adding_task: Signal<bool>,
    panel: Signal<Panel>,
) -> Element {
    let i18 = use_i18();
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!(
        "rounded-lg border max-h-full overflow-y-auto {} {}",
        theme.bg_color_1, theme.border_color
    );
    let chat_gpt_response = use_signal(|| None);
    if *chat_gpt_response.read() == Some(ChatGptResponse::Resolved) {
        panel.set(Panel::None);
    }
    rsx! {
        div {
            class: "p-5 w-full flex flex-col gap-5 items-center justify-center {style}",
            onclick: |event| event.stop_propagation(),
            ChatGpt { chat_gpt_response }
            if chat_gpt_response.read().is_none() {
                div {
                    class: "inline-flex items-center justify-center",
                    hr { class: "w-64 h-px border-0 bg-gray-700" }
                    span {
                        class: "absolute px-3 font-medium -translate-x-1/2 left-1/2 text-white bg-gray-900",
                        {translate!(i18, "or_label")}
                    }
                }
                CustomTaskButton { status, adding_task, panel }
            }
        }
    }
}

#[component]
fn CustomTaskButton(
    status: TaskStatus,
    adding_task: Signal<bool>,
    panel: Signal<Panel>,
) -> Element {
    let i18 = use_i18();
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!("rounded-lg {}", theme.primary_button);
    let mut focus_target = use_context::<Signal<FocusTarget>>();
    rsx! {
        button {
            class: "
                w-full sm:w-auto
                px-5 py-2.5
                text-sm text-center font-medium
                {style}
            ",
            onclick: move |_| {
                panel.set(Panel::None);
                if adding_task() {
                    focus_target.set(
                        FocusTarget(Some(format!("new-{status:#?}-task-title-input")))
                    );
                } else {
                    adding_task.set(true);
                }
            },
            {translate!(i18, "custom_task_button_label")}
        }
    }
}

#[component]
fn BoardList(panel: Signal<Panel>) -> Element {
    let i18 = use_i18();
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!("rounded-lg {} {}", theme.text_color, theme.bg_color_2);
    let current_board = use_context::<Signal<Board>>();
    let current_board = current_board.read();
    let boards = use_context::<Signal<SavedBoards>>();
    rsx! {
        section {
            onclick: move |event| event.stop_propagation(),
            class: "max-h-96 overflow-y-auto px-3 py-5 flex flex-col gap-2 w-1/2 {style}",
            h2 {
                class: "
                    px-2
                    font-bold text-xl
                    flex flex-row gap-1 items-center
                ",
                div { class: "size-5", BookmarkIcon {} }
                {translate!(i18, "board_list_section_label")}
            }
            ul {
                class: "flex flex-col",
                for board in boards
                    .read()
                    .0
                    .iter()
                    .filter(|board| board.name != current_board.board_name)
                {
                    BoardListItem { boards, board: board.clone() }
                }
            }
            JoinBoard { panel }
        }
    }
}

#[component]
fn JoinBoard(panel: Signal<Panel>) -> Element {
    let editing = use_signal(|| false);
    rsx! {
        if editing() {
            JoinBoardForm { panel, editing }
        } else {
            JoinBoardButton { editing }
        }
    }
}

#[component]
fn JoinBoardButton(editing: Signal<bool>) -> Element {
    let i18 = use_i18();
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!("rounded-lg p-2 {}", theme.primary_button);
    rsx! {
        div {
            class: "flex flex-row items-center justify-center",
            button {
                class: style,
                onclick: move |_| editing.set(true),
                {translate!(i18, "join_board_button_label")}
            }
        }
    }
}

#[component]
fn JoinBoardForm(panel: Signal<Panel>, editing: Signal<bool>) -> Element {
    let i18 = use_i18();
    let nav = use_navigator();
    let input_label = translate!(i18, "join_board_input_label");
    rsx! {
        form {
            aria_label: translate!(i18, "join_board_form_label"),
            class: "flex flex-col gap-1 items-center justify-center",
            onsubmit: move |event| {
                let board_name = event.values()[&input_label].as_value().into();
                panel.set(Panel::None);
                nav.push(Route::Board { board_name });
            },
            TextInput {
                id: "join-board-input",
                label: input_label.clone(),
            }
            div {
                class: "flex flex-row gap-2 items-center justify-center",
                ConfirmButton { label: translate!(i18, "join_board_button_label") }
                CancelButton {
                    label: translate!(i18, "cancel_joining_board_button_label"),
                    editing,
                }
            }
        }
    }
}

#[component]
fn BoardListItem(boards: Signal<SavedBoards>, board: SavedBoard) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!("first:border-t border-b {}", theme.border_color);
    rsx! {
        li {
            class: style,
            div {
                class: format!("
                    flex flex-row justify-between items-center
                    px-2
                    size-full rounded-lg
                    group
                    {}
                ", theme.hover_color),
                a {
                    class: "w-full",
                    href: format!("/boards/{}", board.name),
                    div {
                        class: "w-full flex flex-col",
                        p {
                            class: "font-bold",
                            "{board.title}"
                        }
                        p {
                            "{board.name}"
                        }
                    },
                }
                RemoveBoardButton { boards, board: board.clone() }
            }
        }
    }
}

#[component]
fn RemoveBoardButton(boards: Signal<SavedBoards>, board: SavedBoard) -> Element {
    let i18 = use_i18();
    let style = "stroke-red-600 group-hover:stroke-white";
    rsx! {
        button {
            aria_label: translate!(i18, "remove_board_button_label"),
            class: "size-5 {style}",
            onclick: move |_| {
                boards.write().0.retain(|b| b != &board);
            },
            TrashIcon {}
        }
    }
}

#[component]
fn AddTaskButton(status: TaskStatus, adding_task: Signal<bool>, panel: Signal<Panel>) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!("border-t {}", theme.border_color);
    rsx! {
        button {
            class: "
                h-10 sm:h-12 shrink-0 grow-0
                flex flex-row justify-center items-center
                {style}
            ",
            onclick: move |_| {
                panel.set(Panel::ChatGpt{status, adding_task});
            },
            div {
                class: "size-6",
                CircledPlusIcon {}
            }
        }
    }
}
