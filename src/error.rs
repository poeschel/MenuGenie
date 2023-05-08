use std::error::Error;
use std::fmt::Display;

/// Custom Error type
#[derive(Debug)]
pub struct MgError {
    kind: MgErrorKind,
    description: String,
}

impl MgError {
    pub fn kind(&self) -> MgErrorKind {
        self.kind
    }

    /// Creates an EmptyCallStack error
    pub fn empty_call_stack() -> Self {
        Self {
            kind: MgErrorKind::EmptyCallStack,
            description: String::from("Callstack is empty"),
        }
    }

    /// Creates a MissingMenu error
    pub fn missing_menu(id: usize) -> Self {
        Self {
            kind: MgErrorKind::MissingMenu(id),
            description: format!("Missing menu with id {id}"),
        }
    }

    /// Creates a MissingMenuItem error
    pub fn missing_menu_item(menu_id: usize, item_id: usize) -> Self {
        Self {
            kind: MgErrorKind::MissingMenuItem(menu_id, item_id),
            description: format!("Missing menu item with id {item_id} in menu with id {menu_id}"),
        }
    }
}

/// Error kinds for MgError
#[derive(Debug, Clone, Copy)]
pub enum MgErrorKind {
    EmptyCallStack,
    MissingMenu(usize),
    MissingMenuItem(usize, usize),
    ParseInputError,
    IoError,
}

impl From<std::io::Error> for MgError {
    fn from(err: std::io::Error) -> MgError {
        MgError {
            kind: MgErrorKind::IoError,
            description: err.to_string(),
        }
    }
}

impl From<std::num::ParseIntError> for MgError {
    fn from(err: std::num::ParseIntError) -> MgError {
        MgError {
            kind: MgErrorKind::ParseInputError,
            description: err.to_string(),
        }
    }
}

impl Display for MgError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl Error for MgError {
    fn description(&self) -> &str {
        &self.description
    }
}
