use std::fmt::Display;
use std::io::Write;

pub struct MenuGenie {
    menus: Vec<Menu>,
    callstack: Vec<usize>,
}
impl MenuGenie {
    pub fn new(menus: Vec<Menu>, starting_menu_id: usize) -> Result<Self, MgError> {
        let s = Self {
            menus,
            callstack: vec![starting_menu_id],
        };

        if !s.does_menu_exist(starting_menu_id) {
            Err(MgError {})
        } else {
            Ok(s)
        }
    }

    fn does_menu_exist(&self, menu_id: usize) -> bool {
        match self.menus.iter().find(|&menu| menu.id == menu_id) {
            Some(_) => true,
            None => false,
        }
    }

    fn get_menu(&self, menu_id: usize) -> &Menu {
        self.menus.iter().find(|&menu| menu.id == menu_id).unwrap()
    }

    fn get_current_menu(&self) -> Option<&Menu> {
        match self.callstack.last() {
            Some(&menu_id) => Some(self.get_menu(menu_id)),
            None => None,
        }
    }

    fn get_current_menu_unchecked(&self) -> &Menu {
        self.get_menu(*self.callstack.last().unwrap())
    }

    pub fn prompt(&mut self) {
        loop {
            // If callstack is empty we should quit the menu
            match self.get_current_menu() {
                Some(current_menu) => {
                    current_menu.prompt();
                    print!("> ");
                    std::io::stdout().flush().unwrap();
                }
                None => return,
            }

            let mut input = String::new();
            match std::io::stdin().read_line(&mut input) {
                Ok(_) => (),
                Err(e) => println!("{}", e.kind().to_string()),
            };

            match input.trim().parse() {
                Ok(value) => self.execute_callback(value),
                Err(_) => {
                    println!("Invalid input.");
                    continue;
                }
            };
        }
    }

    fn execute_callback(&mut self, choosen_key: usize) {
        match self.get_current_menu_unchecked().get_menu_item(choosen_key) {
            Some(menu_item) => match (menu_item.cb)() {
                MenuAction::Back => self.back(),
                MenuAction::Quit => self.quit(),
                MenuAction::Navigate(id) => self.navigate(id),
                _ => (),
            },
            None => {
                println!("Invalid input.");
                ();
            }
        };
    }

    fn back(&mut self) {
        // There should never be the case where callstack is empty
        // In worse case there is only one item
        // and in next iteration of the loop menu would close
        self.callstack.pop().unwrap();
    }

    fn quit(&mut self) {
        self.callstack.clear()
    }

    fn navigate(&mut self, menu_id: usize) {
        self.callstack.push(menu_id)
    }
}

pub struct Menu {
    id: usize,
    menu_items: Vec<MenuItem>,
}

impl Menu {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            menu_items: Vec::new(),
        }
    }

    pub fn prompt(&self) {
        for menu_item in self.menu_items.iter() {
            println!("{}) {}", menu_item.key, menu_item.prompt);
        }
    }

    pub fn get_menu_item(&self, key: usize) -> Option<&MenuItem> {
        self.menu_items
            .iter()
            .find(|&menu_item| menu_item.key == key)
    }

    pub fn insert_menu_item(&mut self, menu_item: MenuItem) -> Result<(), MgError> {
        match self
            .menu_items
            .iter()
            .find(|&item| item.key == menu_item.key)
        {
            Some(_) => Err(MgError {}),
            None => {
                self.menu_items.push(menu_item);
                Ok(())
            }
        }
    }
}

#[derive(Debug)]
pub struct MgError {}

#[derive(Debug)]
pub enum MenuAction {
    Back,
    Quit,
    Navigate(usize),
    Nothing,
}

pub struct MenuItem {
    prompt: String,
    key: usize,
    pub cb: Box<dyn Fn() -> MenuAction>,
}

impl Display for MenuItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}) {}", self.key, self.prompt)
    }
}

#[cfg(test)]
mod basic_tests {
    use crate::{Menu, MenuAction, MenuGenie, MenuItem};

    #[test]
    fn basic() {
        let mut menu = Menu::new(1);
        let mi1 = MenuItem {
            key: 1,
            prompt: "Manage Nodes".into(),
            cb: Box::new(|| MenuAction::Navigate(2)),
        };
        let mi2 = MenuItem {
            key: 2,
            prompt: "Manage Network".into(),
            cb: Box::new(|| MenuAction::Nothing),
        };
        let mi3 = MenuItem {
            key: 3,
            prompt: "Manage Edges".into(),
            cb: Box::new(|| MenuAction::Nothing),
        };

        let mi4 = MenuItem {
            key: 0,
            prompt: "Quit".into(),
            cb: Box::new(|| MenuAction::Back),
        };

        let mut menu2 = Menu::new(2);
        let mi21 = MenuItem {
            key: 1,
            prompt: "Add Node".into(),
            cb: Box::new(|| MenuAction::Nothing),
        };
        let mi22 = MenuItem {
            key: 2,
            prompt: "Delete Node".into(),
            cb: Box::new(|| MenuAction::Nothing),
        };
        let mi23 = MenuItem {
            key: 3,
            prompt: "Edit Node".into(),
            cb: Box::new(|| MenuAction::Nothing),
        };

        let mi24 = MenuItem {
            key: 0,
            prompt: "Back".into(),
            cb: Box::new(|| MenuAction::Back),
        };

        menu.insert_menu_item(mi1).unwrap();
        menu.insert_menu_item(mi2).unwrap();
        menu.insert_menu_item(mi3).unwrap();
        menu.insert_menu_item(mi4).unwrap();
        menu2.insert_menu_item(mi21).unwrap();
        menu2.insert_menu_item(mi22).unwrap();
        menu2.insert_menu_item(mi23).unwrap();
        menu2.insert_menu_item(mi24).unwrap();
        let mut menu_genie = MenuGenie::new(vec![menu, menu2], 1).unwrap();
        menu_genie.prompt()
    }
}
