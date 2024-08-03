use dioxus::prelude::*;
use itertools::Itertools;
use shared_models::{BoardName, SavedBoard, TaskStatus};

use crate::{
    components::{
        form::{CancelButton, ConfirmButton},
        icons::{
            BarsIcon, BookmarkIcon, CircledPlusIcon, DoneIcon, EditIcon, InProgressIcon,
            SparklesIcon, StackIcon, ToDoIcon, TrashIcon,
        },
        input::TextInput,
        nav::NavBar,
        tooltip::Tooltip,
    },
    model::SavedBoards,
    pages::board::{
        components::{
            ChatGpt, DenseTask, FilterBarTagIcon, NewTaskForm, Task, ThemeButton, UserIcon,
        },
        model::{task_filter, Board, Dense, TagFilter, Tags, Tasks, UserFilter, Users},
        requests::{self, BoardSignals},
    },
    route::Route,
    themes::Theme,
};

#[derive(Clone, Copy, Eq, PartialEq)]
enum Panel {
    None,
    Boards,
    ChatGpt,
}

#[component]
pub fn ThreeColumnBoard(board_name: BoardName) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!("{} {}", theme.text_color, theme.bg_color_1);
    let show_themes = use_signal(|| false);
    let panel = use_signal(|| Panel::None);
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
                        Column { status: TaskStatus::ToDo, panel }
                        Column { status: TaskStatus::InProgress, panel }
                        Column { status: TaskStatus::Done, panel }
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
            Panel::ChatGpt => rsx! { ChatGptPopup { panel } },
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
    let board = use_context::<Signal<Board>>();
    let board = board.read();
    let board_signals = BoardSignals::default();
    rsx! {
        form {
            "aria-label": "update board title",
            class: "grow flex flex-row gap-2 items-center justify-center",
            onsubmit: move |event| {
                let title = event.values()["Title"].as_value();
                spawn_forever(requests::set_board_title(board_signals, title));
                editing.set(false);
            },
            TextInput {
                id: "board-title-input",
                label: "Title",
                value: board.title.clone(),
            }
            ConfirmButton { label: "set title" }
            CancelButton { label: "cancel title update", editing }
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
    rsx! {
        div {
            class: "group relative",
            button {
                "aria-label": "edit title",
                class: "block size-6",
                onclick: move |_| editing.set(true),
                EditIcon {}
            }
            Tooltip { content: "Edit Title", position: "" }
        }
    }
}

#[component]
fn ThemesBar() -> Element {
    let themes = use_context::<Signal<Vec<Theme>>>();
    rsx! {
        section {
            class: "flex flex-row gap-2 items-center",
            h2 { class: "text-xl", "Themes:" }
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
    let tags = use_context::<Signal<Tags>>();
    let tags = &tags.read().0;
    let users = use_context::<Signal<Users>>();
    let users = &users.read().0;
    rsx! {
        section {
            "aria-label": "filters",
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
                    UserIcon {
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
fn DenseButton() -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!("border-2 rounded {}", theme.button);
    let mut dense = use_context::<Signal<Dense>>();
    rsx! {
        div {
            class: "group relative",
            button {
                "aria-label": "toggle dense view",
                class: "size-9 p-1 {style}",
                "aria-pressed": dense.read().0,
                onclick: move |_| {
                    let new_dense = !dense.read().0;
                    dense.set(Dense(new_dense));
                },
                StackIcon {}
            }
            Tooltip { content: "Toggle Dense View", position: "-left-10" }
        }
    }
}

#[component]
fn ToggleThemesButton(show_themes: Signal<bool>) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!("border-2 rounded {}", theme.button);
    rsx! {
        div {
            class: "group relative",
            button {
                "aria-label": "toggle show themes",
                class: "size-9 p-1 {style}",
                "aria-pressed": show_themes(),
                onclick: move |_| {
                    show_themes.set(!show_themes());
                },
                SparklesIcon {}
            }
            Tooltip { content: "Toggle Show Themes", position: "-left-14" }
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
fn Column(status: TaskStatus, panel: Signal<Panel>) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!("border {}", theme.border_color);
    let dense = use_context::<Signal<Dense>>().read().0;
    let gap = if dense { "" } else { "gap-2" };
    let adding_task = use_signal(|| false);
    rsx! {
        section {
            class: "flex flex-col overflow-y-auto px-2 pt-2 gap-2 {style}",
            div {
                class: "flex items-center gap-2",
                match status {
                    TaskStatus::ToDo => rsx! {
                        div { class: "size-8 stroke-red-600", ToDoIcon {} }
                        ColumnHeading { value: "To Do" }
                    },
                    TaskStatus::InProgress => rsx! {
                        div { class: "size-8 stroke-fuchsia-600", InProgressIcon {} }
                        ColumnHeading { value: "In Progress" }
                    },
                    TaskStatus::Done => rsx! {
                        div { class: "size-8 stroke-green-500", DoneIcon {} }
                        ColumnHeading { value: "Done" }
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
    let tasks = tasks.read();
    let board = use_context::<Signal<Board>>();
    let board = board.read();
    let user_filter = use_context::<Signal<UserFilter>>();
    let user_filter = user_filter.read();
    let tag_filter = use_context::<Signal<TagFilter>>();
    let tag_filter = tag_filter.read();
    let column_tasks = match status {
        TaskStatus::ToDo => &board.to_do,
        TaskStatus::InProgress => &board.in_progress,
        TaskStatus::Done => &board.done,
    };
    rsx! {
        for task_id in column_tasks
            .iter()
            .filter(|task_id| {
                task_filter(task_id, &tasks.0, &user_filter.0, &tag_filter.0)
            })
        {
            Task {
                key: "{task_id}",
                task_id: *task_id,
                task: tasks.0[task_id].clone(),
                status,
            }
        }
    }
}

#[component]
fn DenseColumnTasks(status: TaskStatus) -> Element {
    let tasks = use_context::<Signal<Tasks>>();
    let tasks = tasks.read();
    let board = use_context::<Signal<Board>>();
    let board = board.read();
    let user_filter = use_context::<Signal<UserFilter>>();
    let user_filter = user_filter.read();
    let tag_filter = use_context::<Signal<TagFilter>>();
    let tag_filter = tag_filter.read();
    let column_tasks = match status {
        TaskStatus::ToDo => &board.to_do,
        TaskStatus::InProgress => &board.in_progress,
        TaskStatus::Done => &board.done,
    };
    rsx! {
        for task_id in column_tasks
            .iter()
            .filter(|task_id| {
                task_filter(task_id, &tasks.0, &user_filter.0, &tag_filter.0)
            })
        {
            DenseTask {
                key: "{task_id}",
                task_id: *task_id,
                task: tasks.0[task_id].clone(),
                status,
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
            "aria-pressed": panel() == Panel::Boards,
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
fn ChatGptPopup(panel: Signal<Panel>) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = theme.text_color;
    rsx! {
        div {
            class: "
                backdrop-blur-sm backdrop-brightness-50
                size-full absolute inset-0 z-10
                flex flex-row items-center justify-center
                {style}
            ",
            onclick: move |_| panel.set(Panel::None),
            div {
                class: "w-2/3",
                onclick: |event| event.stop_propagation(),
                ChatGpt {}
            }
        }
    }
}

#[component]
fn BoardList(panel: Signal<Panel>) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!("rounded-lg {} {}", theme.text_color, theme.bg_color_2);
    let current_board = use_context::<Signal<Board>>();
    let current_board = current_board.read();
    let boards = use_context::<Signal<SavedBoards>>();
    rsx! {
        section {
            onclick: move |event| event.stop_propagation(),
            class: "px-3 py-5 flex flex-col gap-2 w-1/2 {style}",
            h2 {
                class: "
                    px-2
                    font-bold text-xl
                    flex flex-row gap-1 items-center
                ",
                div { class: "size-5", BookmarkIcon {} }
                "Boards"
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
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!("rounded-lg p-2 {}", theme.primary_button);
    rsx! {
        div {
            class: "flex flex-row items-center justify-center",
            button {
                class: style,
                onclick: move |_| editing.set(true),
                "Join Board"
            }
        }
    }
}

#[component]
fn JoinBoardForm(panel: Signal<Panel>, editing: Signal<bool>) -> Element {
    let nav = use_navigator();
    rsx! {
        form {
            "aria-label": "join board",
            class: "flex flex-col gap-1 items-center justify-center",
            onsubmit: move |event| {
                let board_name = event.values()["Board Name"].as_value().into();
                panel.set(Panel::None);
                nav.push(Route::Board { board_name });
            },
            TextInput {
                id: "join-board-input",
                label: "Board Name"
            }
            div {
                class: "flex flex-row gap-2 items-center justify-center",
                ConfirmButton { label: "join board" }
                CancelButton {
                    label: "cancel join board",
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
    let style = "stroke-red-600 group-hover:stroke-white";
    rsx! {
        button {
            "aria-label": "remove board",
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
                panel.set(Panel::ChatGpt);
            },
            div {
                class: "size-6",
                CircledPlusIcon {}
            }
        }
    }
}
