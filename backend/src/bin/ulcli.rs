use std::fmt::Display;

use inquire::{Select, Text};
use unload::Color;

enum BoardAction {
    JoinBoard,
    CreateBoard,
}

impl Display for BoardAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoardAction::JoinBoard => write!(f, "Join board"),
            BoardAction::CreateBoard => write!(f, "Create board"),
        }
    }
}

fn main() -> Result<(), anyhow::Error> {
    let board_name = match Select::new(
        "What would you like to do?",
        vec![BoardAction::JoinBoard, BoardAction::CreateBoard],
    )
    .with_vim_mode(true)
    .prompt()?
    {
        BoardAction::CreateBoard => {
            todo!();
        }
        BoardAction::JoinBoard => {
            todo!();
        }
    };
    Ok(())
}
