use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum MgError {
    EmptyCallStack,
    MissingMenu(usize),
    MissingMenuItem(usize, usize),
    ParseInputError(std::num::ParseIntError),
    IoError(std::io::Error),
}

impl From<std::io::Error> for MgError {
    fn from(err: std::io::Error) -> MgError {
        MgError::IoError(err)
    }
}

impl From<std::num::ParseIntError> for MgError {
    fn from(err: std::num::ParseIntError) -> MgError {
        MgError::ParseInputError(err)
    }
}

impl Display for MgError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Error for MgError {
    fn description(&self) -> &str {
        match self {
            MgError::ParseInputError(e) => e.description(),
            MgError::EmptyCallStack => "Menu callstack is empty",
            MgError::MissingMenu(id) => &format!("Missing menu with id {id}"),
            MgError::MissingMenuItem(menu_id, item_id) => {
                &format!("Missing menu item with id {item_id} in menu {menu_id}")
            }
            MgError::IoError(e) => e.description(),
        }
    }
}
