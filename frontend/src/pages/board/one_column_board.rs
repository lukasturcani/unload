use dioxus::prelude::*;
use dioxus_sdk::{i18n::use_i18, translate};
use itertools::Itertools;
use shared_models::{BoardName, SavedBoard, TaskStatus};

use crate::{
    commands::FocusTarget,
    components::{
        form::{CancelButton, ConfirmButton},
        icons::{
            BarsIcon, BookmarkIcon, CancelIcon, CircledPlusIcon, DoneIcon, EditIcon,
            ElipsisHorizontalIcon, FilterIcon, InProgressIcon, SparklesIcon, StackIcon, ToDoIcon,
            TrashIcon,
        },
        input::TextInput,
        nav::NavBar,
    },
    model::SavedBoards,
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
};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Panel {
    None,
    Actions,
    Navigation,
    Status,
    ChatGpt {
        status: TaskStatus,
        adding_task: Signal<bool>,
    },
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum ExtraBar {
    None,
    Filter,
    Themes,
}

#[component]
pub fn OneColumnBoard(board_name: BoardName) -> Element {
    let i18 = use_i18();
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!("{} {}", theme.text_color, theme.bg_color_1);
    let status = use_signal(|| TaskStatus::ToDo);
    let status_ = status();
    let mut panel = use_signal(|| Panel::None);
    let aria_label = match status_ {
        TaskStatus::ToDo => translate!(i18, "to_do_column_title"),
        TaskStatus::InProgress => translate!(i18, "in_progress_column_title"),
        TaskStatus::Done => translate!(i18, "done_column_title"),
    };
    let extra_bar = use_signal(|| ExtraBar::None);
    let adding_task = use_signal(|| false);
    rsx! {
        div {
            onclick: move |_| panel.set(Panel::None),
            class: "flex flex-col h-dvh w-screen {style}",
            Header { panel, status, extra_bar }
            section {
                class: "grow flex flex-col overflow-y-auto gap-1",
                aria_label,
                ColumnSwitcher { status, panel }
                Column { status: status_, adding_task }
            }
            AddTaskButton { status: status_, adding_task, panel }
            match extra_bar() {
                ExtraBar::Filter => rsx! { FilterBar { extra_bar } },
                ExtraBar::Themes => rsx! { ThemesBar { extra_bar } },
                ExtraBar::None => rsx! {},
            }
            NavBar { board_name }
        }
        match panel() {
            Panel::Actions => rsx! { ActionsSheet { panel, extra_bar } },
            Panel::Navigation => rsx! { NavigationSheet { panel } },
            Panel::ChatGpt{status, adding_task} => rsx! { ChatGptSheet { status, adding_task, panel } },
            Panel::None | Panel::Status => rsx! {},
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
            onclick: move |event| {
                event.stop_propagation();
                panel.set(Panel::ChatGpt{status, adding_task});
            },
            div {
                class: "size-6",
                CircledPlusIcon {}
            }
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
    let input_label = translate!(i18, "board_title_input_label");
    rsx! {
        form {
            aria_label: translate!(i18, "board_title_update_form_label"),
            class: "grow flex flex-col gap-1 items-center justify-center",
            onsubmit: move |event| {
                let title = event.values()[&input_label].as_value();
                spawn_forever(requests::set_board_title(board_signals, title));
                editing.set(false);
            },
            div {
                class: "flex flex-row gap-1 items-center justify-center",
                TextInput {
                    id: "board-title-input",
                    label: input_label.clone(),
                    value: title,
                }
            }
            div {
                class: "flex flex-row gap-2 items-center justify-center",
                ConfirmButton { label: translate!(i18, "set_board_title_button_label") }
                CancelButton {
                    label: translate!(i18, "cancel_board_title_update_button_label"),
                    editing,
                }
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
            class: "flex flex-col truncate",
            div {
                class: "flex flex-row items-center justify-center gap-2 truncate",
                h1 {
                    class: "font-extrabold truncate",
                    {board.title.clone()}
                }
                div {
                    class: "shrink-0",
                    EditTitleButton { editing }
                }
            }
            p { class: "text-center", "{board.board_name}" }
        }
    }
}

#[component]
fn EditTitleButton(editing: Signal<bool>) -> Element {
    let i18 = use_i18();
    rsx! {
        button {
            aria_label: translate!(i18, "edit_board_title_tooltip"),
            class: "block size-4",
            onclick: move |_| editing.set(true),
            EditIcon {}
        }
    }
}

#[component]
fn BottomSheet(panel: Signal<Panel>, body: Element) -> Element {
    rsx! {
        div {
            class: "
                size-full absolute inset-0 z-10
                flex flex-col
            ",
            div {
                class: "grow backdrop-blur-sm",
                onclick: move |_| panel.set(Panel::None),
            }
            {body}
        }
    }
}

#[component]
fn ChatGptSheet(status: TaskStatus, panel: Signal<Panel>, adding_task: Signal<bool>) -> Element {
    let i18 = use_i18();
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!(
        "
                rounded-t-2xl text-lg border-t overflow-hidden
                {} {} {}
            ",
        theme.bg_color_1, theme.text_color, theme.border_color
    );
    let chat_gpt_response = use_signal(|| None);
    if *chat_gpt_response.read() == Some(ChatGptResponse::Resolved) {
        panel.set(Panel::None);
    }
    rsx! {
        BottomSheet {
            panel,
            body: rsx! {
                section {
                    aria_label: "chat gpt",
                    class: "flex flex-col gap-5 h-5/6 {style}",
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
fn ActionsSheet(panel: Signal<Panel>, extra_bar: Signal<ExtraBar>) -> Element {
    let i18 = use_i18();
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!(
        "
                rounded-t-2xl text-lg border-t
                {} {} {}
            ",
        theme.bg_color_1, theme.text_color, theme.border_color
    );
    let mut dense = use_context::<Signal<Dense>>();
    rsx! {
        BottomSheet {
            panel
            body: rsx! {
                section {
                    aria_label: translate!(i18, "task_actions_section_label"),
                    class: "flex flex-col gap-2 pt-2 pb-20 {style}",
                    button {
                        class: "flex flex-row gap-2 items-center justify-left px-1",
                        onclick: move |_| {
                            let new_dense = !dense.read().0;
                            dense.set(Dense(new_dense));
                            panel.set(Panel::None);
                        },
                        div { class: "size-5", StackIcon {} }
                        {translate!(i18, "toggle_dense_view_tooltip")}
                    }
                    button {
                        class: "flex flex-row gap-2 items-center justify-left px-1",
                        onclick: move |_| {
                            extra_bar.set(ExtraBar::Themes);
                            panel.set(Panel::None);
                        },
                        div { class: "size-5", SparklesIcon {} }
                        {translate!(i18, "toggle_show_themes_tooltip")}
                    }
                }
            },
        }
    }
}

#[component]
fn SideSheet(panel: Signal<Panel>, body: Element) -> Element {
    rsx! {
        div {
            class: "
                size-full absolute inset-0 z-10
                flex flex-row
            ",
            {body}
            div {
                class: "grow backdrop-blur-sm",
                onclick: move |_| panel.set(Panel::None),
            }
        }
    }
}

#[component]
fn NavigationSheet(panel: Signal<Panel>) -> Element {
    let i18 = use_i18();
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!(
        "text-lg {} {} {}",
        theme.bg_color_1, theme.text_color, theme.border_color
    );
    rsx! {
        SideSheet {
            panel,
            body: rsx! {
                section {
                    class: "w-10/12 {style}",
                    aria_label: translate!(i18, "navigation_section_label"),
                    BoardList { panel }
                }
            }
        }
    }
}

#[component]
fn BoardList(panel: Signal<Panel>) -> Element {
    let i18 = use_i18();
    let current_board = use_context::<Signal<Board>>();
    let current_board = current_board.read();
    let boards = use_context::<Signal<SavedBoards>>();
    rsx! {
        section {
            class: "px-2 flex flex-col gap-2",
            h2 {
                class: "font-bold flex flex-row gap-1 items-center",
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
                class: "text-sm {style}",
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
            class: "
                flex flex-row justify-between items-center
                text-sm {style}
            ",
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

#[component]
fn RemoveBoardButton(boards: Signal<SavedBoards>, board: SavedBoard) -> Element {
    let i18 = use_i18();
    let style = "stroke-red-600";
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
fn Header(
    panel: Signal<Panel>,
    status: Signal<TaskStatus>,
    extra_bar: Signal<ExtraBar>,
) -> Element {
    let editing_title = use_signal(|| false);
    let height = if editing_title() {
        ""
    } else {
        "h-14 shrink-0 grow-0"
    };
    rsx! {
        header {
            class: "
                flex flex-row items-center justify-between
                w-full {height} py-1 px-2 gap-1
            ",
            if editing_title() {
                TitleInput { editing: editing_title }
            } else {
                div {
                    class: "shrink-0",
                    ToggleNavDrawerButton { panel }
                }
                TitleShow { editing: editing_title }
                div {
                    class: "shrink-0 flex flex-row gap-1 items-center justify-end",
                    ToggleFiltersButton { extra_bar }
                    ToggleActionsDrawerButton { panel }
                }
            }
        }
    }
}

#[component]
fn Column(status: TaskStatus, adding_task: Signal<bool>) -> Element {
    let dense = use_context::<Signal<Dense>>().read().0;
    rsx! {
        div {
            class: "grow flex flex-col overflow-y-auto",
            if dense {
                DenseColumnTasks { status }
            } else {
                ColumnTasks { status }
            }
            if adding_task() {
                NewTaskForm { status, adding_task }
            }
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
fn ToggleNavDrawerButton(panel: Signal<Panel>) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!("border rounded {}", theme.button);
    rsx! {
        button {
            class: "size-7 p-1 {style}",
            aria_pressed: panel() == Panel::Navigation,
            onclick: move |event| {
                event.stop_propagation();
                if panel() == Panel::Navigation {
                    panel.set(Panel::None)
                } else {
                    panel.set(Panel::Navigation)
                }
            },
            BarsIcon {}
        }
    }
}

#[component]
fn ToggleActionsDrawerButton(panel: Signal<Panel>) -> Element {
    let i18 = use_i18();
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!("border rounded {}", theme.button);
    rsx! {
        button {
            aria_label: translate!(i18, "toggle_actions_drawer_button_label"),
            class: "size-7 p-1 {style}",
            aria_pressed: panel() == Panel::Actions,
            onclick: move |event| {
                event.stop_propagation();
                if panel() == Panel::Actions {
                    panel.set(Panel::None)
                } else {
                    panel.set(Panel::Actions)
                }
            },
            ElipsisHorizontalIcon {}
        }
    }
}

#[component]
fn ToggleFiltersButton(extra_bar: Signal<ExtraBar>) -> Element {
    let i18 = use_i18();
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!("border rounded {}", theme.button);
    rsx! {
        button {
            aria_label: translate!(i18, "toggle_show_filters_button_label"),
            class: "size-7 p-1 {style}",
            aria_pressed: extra_bar() == ExtraBar::Filter,
            onclick: move |event| {
                event.stop_propagation();
                if extra_bar() == ExtraBar::Filter {
                    extra_bar.set(ExtraBar::None)
                } else {
                    extra_bar.set(ExtraBar::Filter)
                }
            },
            FilterIcon {}
        }
    }
}

#[component]
fn ColumnSwitcher(status: Signal<TaskStatus>, panel: Signal<Panel>) -> Element {
    let i18 = use_i18();
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let status_style = format!("border rounded {}", theme.button);
    let dropdown_style = format!(
        "
            border divide-y
            rounded-lg shadow-sm
            {} {} {}
        ",
        theme.border_color, theme.divide_color, theme.bg_color_2,
    );
    rsx! {
        div {
            class: "group relative px-2",
            button {
                class: "
                    py-0.5 px-1
                    flex flex-row gap-1 items-center
                    text-sm
                    {status_style}
                ",
                aria_pressed: panel() == Panel::Status,
                onclick: move |event| {
                    if panel() == Panel::Status {
                        panel.set(Panel::None);
                    } else {
                        panel.set(Panel::Status);
                    }
                    event.stop_propagation();
                },
                match status() {
                    TaskStatus::ToDo => rsx! {
                        div { class: "size-3", ToDoIcon {} }
                        "To Do"
                    },
                    TaskStatus::InProgress => rsx! {
                        div { class: "size-3", InProgressIcon {} }
                        "In Progress"
                    },
                    TaskStatus::Done => rsx! {
                        div { class: "size-3", DoneIcon {} }
                        "Done"
                    }
                }
            }
            if panel() == Panel::Status {
                div {
                    class: "
                        absolute -bottom-24
                        z-10
                        flex flex-col
                        text-lg
                        {dropdown_style}
                    ",
                    button {
                        class: "flex flex-row gap-1 items-center text-nowrap px-1",
                        onclick: move |event| {
                            status.set(TaskStatus::ToDo);
                            panel.set(Panel::None);
                            event.stop_propagation();
                        },
                        div { class: "size-5", ToDoIcon {} }
                        {translate!(i18, "to_do_column_title")}
                    }
                    button {
                        class: "flex flex-row gap-1 items-center text-nowrap px-1",
                        onclick: move |event| {
                            status.set(TaskStatus::InProgress);
                            panel.set(Panel::None);
                            event.stop_propagation();
                        },
                        div { class: "size-5", InProgressIcon {} }
                        {translate!(i18, "in_progress_column_title")}
                    }
                    button {
                        class: "flex flex-row gap-1 items-center text-nowrap px-1",
                        onclick: move |event| {
                            status.set(TaskStatus::Done);
                            panel.set(Panel::None);
                            event.stop_propagation();
                        },
                        div { class: "size-5", DoneIcon {} }
                        {translate!(i18, "done_column_title")}
                    }
                }
            }
        }
    }
}

#[component]
fn ThemesBar(extra_bar: Signal<ExtraBar>) -> Element {
    let themes = use_context::<Signal<Vec<Theme>>>();
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!(
        "border-t divide-x-2 {} {}",
        theme.border_color, theme.divide_color
    );
    rsx! {
        section {
            class: "
                flex flex-row gap-2 items-center justify-between
                p-2
                {style}
            ",
            "aria-label": "themes",
            div {
                class: "flex flex-row overflow-x-auto gap-2",
                for theme in themes.read().iter() {
                    ThemeButton { theme: *theme }
                }
            }
            button {
                "aria-label": "close theme selector",
                class: "size-6 shrink-0",
                onclick: move |_| extra_bar.set(ExtraBar::None),
                CancelIcon {}
            }
        }
    }
}

#[component]
fn FilterBar(extra_bar: Signal<ExtraBar>) -> Element {
    let theme = use_context::<Signal<Theme>>();
    let theme = theme.read();
    let style = format!("border-t {}", theme.border_color);
    let tags = use_context::<Signal<Tags>>();
    let tags = &tags.read().0;
    let users = use_context::<Signal<Users>>();
    let users = &users.read().0;
    rsx! {
        section {
            "aria-label": "filters",
            class: "flex flex-col gap-1 px-2 py-1 {style}",
            div {
                class: "flex flex-row px-1 justify-end",
                button {
                    "aria-label": "close filters",
                    class: "size-6",
                    onclick: move |_| extra_bar.set(ExtraBar::None),
                    CancelIcon {}
                }
            }
            div {
                class: "flex flex-row gap-1 flex-wrap items-center justify-center",
                for tag_id in tags.keys().sorted_by_key(|tag_id| tags[tag_id].name.to_lowercase())
                {
                    FilterBarTagIcon {
                        tag_id: *tag_id,
                        tag_data: tags[tag_id].clone(),
                    }
                }
            }
            div {
                class: "flex flex-row gap-1 flex-wrap items-center justify-center",
                for user_id in users.keys().sorted_by_key(|user_id| users[user_id].name.to_lowercase())
                {
                    FilteringUserIcon {
                        user_id: *user_id,
                        user_data: users[user_id].clone(),
                        size: "size-6",
                        tooltip_position: "-left-3 -top-10",
                    }
                }
            }

        }
    }
}
