use std::io::Write;

mod action;
mod builder;
mod error;
mod menu;
mod menu_item;

pub use action::MenuAction;
pub use builder::MenuBuilder;
pub use error::MgError;

use menu::Menu;

pub struct MenuGenie<'a> {
    menus: Vec<Menu<'a>>,
    callstack: Vec<usize>,
    start_menu_id: usize,
}

impl<'a> MenuGenie<'a> {
    fn get_menu(&self, menu_id: usize) -> Result<&Menu, MgError> {
        self.menus
            .iter()
            .find(|&menu| menu.id == menu_id)
            .ok_or(MgError::MissingMenu(menu_id))
    }

    fn get_current_menu(&self) -> Result<&Menu, MgError> {
        match self.callstack.last() {
            Some(&menu_id) => self.get_menu(menu_id),
            None => Err(MgError::EmptyCallStack),
        }
    }

    fn get_current_menu_unchecked(&self) -> &Menu {
        self.get_menu(*self.callstack.last().unwrap()).unwrap()
    }

    pub fn restart(&mut self) -> Result<Option<(usize, usize)>, MgError> {
        self.callstack.clear();
        self.callstack.push(self.start_menu_id);
        self.prompt()
    }

    pub fn prompt(&mut self) -> Result<Option<(usize, usize)>, MgError> {
        let mut input = String::new();

        loop {
            match self.get_current_menu() {
                Ok(current_menu) => {
                    current_menu.prompt();
                    std::io::stdout().flush().unwrap();
                }
                Err(e) => match e {
                    MgError::EmptyCallStack => {
                        // If the callstack is empty we should quit the menu
                        return Ok(None);
                    }
                    e => return Err(e),
                },
            }

            std::io::stdin().read_line(&mut input)?;

            match input.trim().parse() {
                Ok(value) => match self.check_menu_action(value) {
                    Ok(action_tuple) => match action_tuple {
                        Some(value) => return Ok(Some(value)),
                        None => continue,
                    },
                    // Returning MissingMenuItem Error
                    Err(e) => return Err(e),
                },
                // Returning ParseInputError
                Err(e) => return Err(e.into()),
            };
        }
    }

    fn check_menu_action(&mut self, choosen_key: usize) -> Result<Option<(usize, usize)>, MgError> {
        let current_menu = self.get_current_menu_unchecked();
        match current_menu.get_menu_item(choosen_key) {
            Some(menu_item) => match menu_item.action {
                MenuAction::Back => {
                    self.back();
                    Ok(None)
                }
                MenuAction::Quit => {
                    self.quit();
                    Ok(None)
                }
                MenuAction::BackToStart => {
                    self.back_to_start();
                    Ok(None)
                }
                MenuAction::Navigate(id) => {
                    self.navigate(id);
                    Ok(None)
                }
                _ => Ok(Some((current_menu.id, choosen_key))),
            },

            None => Err(MgError::MissingMenuItem(current_menu.id, choosen_key)),
        }
    }

    fn back(&mut self) {
        // UNWRAP There should never be the case where callstack is empty
        // In worse case there is only one item
        // and in next iteration of the loop menu would close
        self.callstack.pop().unwrap();
    }

    fn back_to_start(&mut self) {
        // UNWRAP Same as in fn back
        let first = *self.callstack.first().unwrap();
        self.callstack.retain(|ele| *ele == first)
    }

    fn quit(&mut self) {
        self.callstack.clear()
    }

    fn navigate(&mut self, menu_id: usize) {
        self.callstack.push(menu_id)
    }
}
