use crate::MenuAction;
use std::fmt::Display;

pub struct MenuItem<'a> {
    pub prompt: &'a str,
    pub action: MenuAction,
    pub key: usize,
}

impl<'a> Display for MenuItem<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}) {}", self.key, self.prompt)
    }
}
