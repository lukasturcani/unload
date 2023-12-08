use std::fmt::Display;

use chrono::NaiveTime;
use inquire::{validator::Validation, DateSelect, MultiSelect, Select, Text};
use reqwest::{Client, Url};
use shared_models::{
    BoardName, Color, TaskData, TaskEntry, TaskId, TaskSize, TaskStatus, UserData, UserEntry,
    UserId,
};

struct UserDisplay(UserEntry);

impl Display for UserDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.name)
    }
}

struct TaskDisplay(TaskEntry);

impl Display for TaskDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.title)
    }
}

struct ColorDisplay(Color);

impl Display for ColorDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ColorDisplay(Color::Black) => write!(f, "Black"),
            ColorDisplay(Color::White) => write!(f, "White"),
            ColorDisplay(Color::Gray) => write!(f, "Gray"),
            ColorDisplay(Color::Silver) => write!(f, "Silver"),
            ColorDisplay(Color::Maroon) => write!(f, "Maroon"),
            ColorDisplay(Color::Red) => write!(f, "Red"),
            ColorDisplay(Color::Purple) => write!(f, "Purple"),
            ColorDisplay(Color::Fushsia) => write!(f, "Fushsia"),
            ColorDisplay(Color::Green) => write!(f, "Green"),
            ColorDisplay(Color::Lime) => write!(f, "Lime"),
            ColorDisplay(Color::Olive) => write!(f, "Olive"),
            ColorDisplay(Color::Yellow) => write!(f, "Yellow"),
            ColorDisplay(Color::Navy) => write!(f, "Navy"),
            ColorDisplay(Color::Blue) => write!(f, "Blue"),
            ColorDisplay(Color::Teal) => write!(f, "Teal"),
            ColorDisplay(Color::Aqua) => write!(f, "Aqua"),
        }
    }
}

struct TaskSizeDisplay(TaskSize);

impl Display for TaskSizeDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskSizeDisplay(TaskSize::Small) => write!(f, "Small"),
            TaskSizeDisplay(TaskSize::Medium) => write!(f, "Medium"),
            TaskSizeDisplay(TaskSize::Large) => write!(f, "Large"),
        }
    }
}

struct TaskStatusDisplay(TaskStatus);

impl Display for TaskStatusDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskStatusDisplay(TaskStatus::ToDo) => write!(f, "To Do"),
            TaskStatusDisplay(TaskStatus::InProgress) => write!(f, "In Progress"),
            TaskStatusDisplay(TaskStatus::Done) => write!(f, "Done"),
        }
    }
}

enum BoardChoice {
    JoinBoard,
    CreateBoard,
}

impl Display for BoardChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoardChoice::JoinBoard => write!(f, "Join board"),
            BoardChoice::CreateBoard => write!(f, "Create board"),
        }
    }
}

enum BoardAction {
    GetTask,
    AddTask,
    GetAllTasks,
    DeleteTask,
    GetUser,
    AddUser,
    GetAllUsers,
    DeleteUser,
}

impl Display for BoardAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoardAction::GetTask => write!(f, "Get task"),
            BoardAction::AddTask => write!(f, "Add task"),
            BoardAction::GetAllTasks => write!(f, "Get all tasks"),
            BoardAction::DeleteTask => write!(f, "Delete task"),
            BoardAction::GetUser => write!(f, "Get user"),
            BoardAction::AddUser => write!(f, "Add user"),
            BoardAction::GetAllUsers => write!(f, "Get all users"),
            BoardAction::DeleteUser => write!(f, "Delete user"),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let client = Client::new();
    let url = Text::new("Unload URL:").prompt()?.parse::<Url>()?;
    let board_name = match Select::new(
        "What would you like to do?",
        vec![BoardChoice::JoinBoard, BoardChoice::CreateBoard],
    )
    .with_vim_mode(true)
    .prompt()?
    {
        BoardChoice::CreateBoard => {
            client
                .post(url.join("/api/boards")?)
                .send()
                .await?
                .json::<BoardName>()
                .await?
        }
        BoardChoice::JoinBoard => Text::new("Board Name:").prompt()?.into(),
    };
    println!("Your board is {board_name}!");
    loop {
        let users = client
            .get(url.join(&format!("/api/boards/{board_name}/users"))?)
            .send()
            .await?
            .json::<Vec<UserEntry>>()
            .await?;
        let tasks = client
            .get(url.join(&format!("/api/boards/{board_name}/tasks"))?)
            .send()
            .await?
            .json::<Vec<TaskEntry>>()
            .await?;
        match Select::new(
            "What would you like to do?",
            vec![
                BoardAction::GetTask,
                BoardAction::AddTask,
                BoardAction::GetAllTasks,
                BoardAction::DeleteTask,
                BoardAction::GetUser,
                BoardAction::AddUser,
                BoardAction::GetAllUsers,
                BoardAction::DeleteUser,
            ],
        )
        .with_vim_mode(true)
        .prompt()?
        {
            BoardAction::GetTask => {
                let task_id = Text::new("Task Id:").prompt()?;
                let task = client
                    .get(url.join(&format!("/api/boards/{board_name}/tasks/{task_id}"))?)
                    .send()
                    .await?
                    .json::<TaskEntry>()
                    .await?;
                println!("{task:#?}");
            }
            BoardAction::AddTask => {
                let title = Text::new("Title:").prompt()?;
                let description = Text::new("Description:").prompt()?;
                let due = if let Some(due_date) = DateSelect::new("Due Date:").prompt_skippable()? {
                    let time_fmt = "%I:%M %p";
                    let time = Text::new("Time Due (HH:MM tt):")
                        .with_validator(|time: &str| {
                            match NaiveTime::parse_from_str(time, time_fmt) {
                                Ok(_) => Ok(Validation::Valid),
                                Err(_) => Ok(Validation::Invalid("not a valid time format".into())),
                            }
                        })
                        .prompt()?;
                    Some(
                        due_date
                            .and_time(NaiveTime::parse_from_str(&time, time_fmt)?)
                            .and_utc(),
                    )
                } else {
                    None
                };
                let size = Select::new(
                    "Size:",
                    vec![
                        TaskSizeDisplay(TaskSize::Small),
                        TaskSizeDisplay(TaskSize::Medium),
                        TaskSizeDisplay(TaskSize::Large),
                    ],
                )
                .with_vim_mode(true)
                .prompt()?
                .0;
                let status = Select::new(
                    "Status:",
                    vec![
                        TaskStatusDisplay(TaskStatus::ToDo),
                        TaskStatusDisplay(TaskStatus::InProgress),
                        TaskStatusDisplay(TaskStatus::Done),
                    ],
                )
                .with_vim_mode(true)
                .prompt()?
                .0;
                let assignees = if !users.is_empty() {
                    MultiSelect::new(
                        "Assignees:",
                        users.iter().map(|user| UserDisplay(user.clone())).collect(),
                    )
                    .with_vim_mode(true)
                    .prompt()?
                    .into_iter()
                    .map(|user| user.0.id)
                    .collect()
                } else {
                    Vec::new()
                };
                let blocks = if !tasks.is_empty() {
                    MultiSelect::new(
                        "Blocks:",
                        tasks.iter().map(|task| TaskDisplay(task.clone())).collect(),
                    )
                    .prompt()?
                    .into_iter()
                    .map(|task| task.0.id)
                    .collect()
                } else {
                    Vec::new()
                };
                let blocked_by = if !tasks.is_empty() {
                    MultiSelect::new(
                        "Blocked By:",
                        tasks
                            .iter()
                            .filter(|task| !blocks.contains(&task.id))
                            .map(|task| TaskDisplay(task.clone()))
                            .collect(),
                    )
                    .prompt()?
                    .into_iter()
                    .map(|task| task.0.id)
                    .collect()
                } else {
                    Vec::new()
                };
                let task_id = client
                    .post(url.join(&format!("/api/boards/{board_name}/tasks"))?)
                    .json(&TaskData {
                        title,
                        description,
                        due,
                        size,
                        status,
                        assignees,
                        blocks,
                        blocked_by,
                    })
                    .send()
                    .await?
                    .json::<TaskId>()
                    .await?;
                println!("Created task {task_id}!");
            }
            BoardAction::GetAllTasks => {
                let tasks = client
                    .get(url.join(&format!("/api/boards/{board_name}/tasks"))?)
                    .send()
                    .await?
                    .json::<Vec<TaskEntry>>()
                    .await?;
                println!("{tasks:#?}");
            }
            BoardAction::DeleteTask => {
                let task_id = Text::new("Task Id:").prompt()?;
                client
                    .delete(url.join(&format!("/api/boards/{board_name}/tasks/{task_id}"))?)
                    .send()
                    .await?
                    .json::<()>()
                    .await?;
            }
            BoardAction::GetUser => {
                let user_id = Text::new("User Id:").prompt()?;
                let user = client
                    .get(url.join(&format!("/api/boards/{board_name}/users/{user_id}"))?)
                    .send()
                    .await?
                    .json::<UserEntry>()
                    .await?;
                println!("{user:#?}");
            }
            BoardAction::AddUser => {
                let name = Text::new("User Name:").prompt()?;
                let color = Select::new(
                    "Color:",
                    vec![
                        ColorDisplay(Color::Black),
                        ColorDisplay(Color::White),
                        ColorDisplay(Color::Gray),
                        ColorDisplay(Color::Silver),
                        ColorDisplay(Color::Maroon),
                        ColorDisplay(Color::Red),
                        ColorDisplay(Color::Purple),
                        ColorDisplay(Color::Fushsia),
                        ColorDisplay(Color::Green),
                        ColorDisplay(Color::Lime),
                        ColorDisplay(Color::Olive),
                        ColorDisplay(Color::Yellow),
                        ColorDisplay(Color::Navy),
                        ColorDisplay(Color::Blue),
                        ColorDisplay(Color::Teal),
                        ColorDisplay(Color::Aqua),
                    ],
                )
                .with_vim_mode(true)
                .prompt()?
                .0;
                let user_id = client
                    .post(url.join(&format!("/api/boards/{board_name}/users"))?)
                    .json(&UserData { name, color })
                    .send()
                    .await?
                    .json::<UserId>()
                    .await?;
                println!("Created user {user_id}!")
            }
            BoardAction::GetAllUsers => {
                let users = client
                    .get(url.join(&format!("/api/boards/{board_name}/users"))?)
                    .send()
                    .await?
                    .json::<Vec<UserEntry>>()
                    .await?;
                println!("{users:#?}");
            }
            BoardAction::DeleteUser => {
                let user_id = Text::new("User Id:").prompt()?;
                client
                    .delete(url.join(&format!("/api/boards/{board_name}/users/{user_id}"))?)
                    .send()
                    .await?
                    .json::<()>()
                    .await?;
            }
        }
    }
}
