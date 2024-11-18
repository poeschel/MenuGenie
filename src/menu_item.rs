use crate::MenuAction;
use std::fmt::Display;

pub struct MenuItem {
    pub prompt: String,
    pub action: MenuAction,
    pub key: usize,
}

impl Display for MenuItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}) {}", self.key, self.prompt)
    }
}
