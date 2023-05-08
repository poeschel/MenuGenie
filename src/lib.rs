//! MenuGenie is a library for making multi-level terminal menus.
//! It provides a MenuBuilder with a simple API for createing menus
//! and managing actions when user makes a choice.
//!
//! # How to use MenuGenie
//!
//! Recomended way to use MenuGenie is to create a MenuBuilder instance,
//! add menus and menu items to it.
//! After that call prompt in a loop, handle errors and user inputs.
//!
//! ### Example
//!
//! Here is an example of how to create a simple menu.
//!
//! ```rust
//! let mut menu = menu_genie::MenuBuilder::new()
//!     .with_menu(1)
//!     .with_menu_item("Create Todo", MenuAction::Nothing)
//!     .with_menu_item("Delete Todo", MenuAction::Nothing)
//!     .with_menu_item("Edit Todo", MenuAction::Navigate(2))
//!     .with_quit_button()
//!     .with_menu(2)
//!     .with_menu_item("Change Name", MenuAction::Nothing)
//!     .with_menu_item("Change Description", MenuAction::Nothing)
//!     .with_back_button()
//!     .build();
//!
//! loop {
//!     match menu.prompt() {
//!         Ok(tuple) => match tuple {
//!             (1, 1) => println!("ACTION: Create Todo"),
//!             (1, 2) => println!("ACTION: Delete Todo"),
//!             (2, 1) => println!("ACTION: Change Name"),
//!             (2, 1) => println!("ACTION: Change Description"),
//!             (0, 0) => break,
//!             _ => ()
//!         },
//!         Err(e) => {
//!             println("{e}");
//!         }
//!     }
//! }
//! ```
//!
//! As you can see prompt returns a Result containing a tuple (menu_id, menu_item_id) or an error.
//! Special case is when user wants to quit the menu, in that case tuple returned is (0, 0).
//!
//! More examples can be found [here](https://github.com/DusanDjordjic/MenuGenie/blob/master/examples).
//!

use std::io::Write;

mod action;
mod builder;
mod error;
mod menu;
mod menu_item;

pub use action::MenuAction;
pub use builder::MenuBuilder;
pub use error::{MgError, MgErrorKind};

use menu::Menu;

/// MenuGenie is a core struct that keeps track of which menus are called and which menu should be displayed.
///
/// When you run build on MenuBuilder it creates MenuGenie and sets it's starting menu.
/// It keeps track of called menus by using callstack and modifies it according to the actions
/// provided. See the MenuAction for details.

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
            .ok_or(MgError::missing_menu(menu_id))
    }

    fn get_current_menu(&self) -> Result<&Menu, MgError> {
        match self.callstack.last() {
            Some(&menu_id) => self.get_menu(menu_id),
            None => Err(MgError::empty_call_stack()),
        }
    }

    fn get_current_menu_unchecked(&self) -> &Menu {
        self.get_menu(*self.callstack.last().unwrap()).unwrap()
    }

    /// Prompts the user with the last menu from the callstack.
    ///
    /// After that it gets user input and executes the actions provided
    /// or in case Nothing action was encountered
    /// it returns the tuple (menu_id, menu_item_id) indicationg which menu item user selected.
    /// When the callstack is empty (0, 0) is returned.
    pub fn prompt(&mut self) -> Result<(usize, usize), MgError> {
        let mut input = String::new();

        loop {
            match self.get_current_menu() {
                Ok(current_menu) => {
                    current_menu.prompt();
                    std::io::stdout().flush().unwrap();
                }
                Err(e) => match e.kind() {
                    MgErrorKind::EmptyCallStack => {
                        // If the callstack is empty we should quit the menu
                        return Ok((0, 0));
                    }
                    _ => return Err(e),
                },
            }

            input.clear();
            std::io::stdin().read_line(&mut input)?;

            match input.trim().parse() {
                Ok(value) => match self.check_menu_action(value) {
                    Ok(action_tuple) => match action_tuple {
                        Some(value) => return Ok(value),
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

    /// Resets the callstack to the starting position and calls prompt
    pub fn restart(&mut self) -> Result<(usize, usize), MgError> {
        self.callstack.clear();
        self.callstack.push(self.start_menu_id);
        self.prompt()
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

            None => Err(MgError::missing_menu_item(current_menu.id, choosen_key)),
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
