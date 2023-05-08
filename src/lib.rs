use std::fmt::Display;
use std::io::Write;

pub struct MenuBuilder<'a> {
    menus: Vec<Menu<'a>>,
    starting_menu_id: Option<usize>,
}

impl<'a> MenuBuilder<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_menu(mut self, key: usize) -> Self {
        assert!(
            self.menus.iter().find(|menu| menu.id == key).is_none(),
            "Menu with key {key} already added"
        );

        self.menus.push(Menu::new(key));
        if let None = self.starting_menu_id {
            self.starting_menu_id = Some(self.menus.len())
        }
        self
    }

    pub fn with_menu_item(mut self, prompt: &'a str, action: MenuAction) -> Self {
        let last_added_menu = self.menus.last_mut().expect("Menu must be added first");
        last_added_menu.menu_items.push(MenuItem {
            prompt,
            action,
            key: last_added_menu.menu_items.len() + 1,
        });
        self
    }

    pub fn with_back_button(mut self) -> Self {
        let last_added_menu = self.menus.last_mut().expect("Menu must be added first");
        last_added_menu.menu_items.push(MenuItem {
            prompt: "Back",
            action: MenuAction::Back,
            key: 0,
        });
        self
    }

    pub fn starting_menu(mut self, starting_menu_id: usize) -> Self {
        self.starting_menu_id = Some(starting_menu_id);
        self
    }

    pub fn build(self) -> MenuGenie<'a> {
        assert_ne!(self.menus.len(), 0, "No menus added.");
        MenuGenie {
            menus: self.menus,
            // UNWRAP its safe to unwrap here bacause if no menu is added we have assert
            // and if there are menus added starting_menu_id will be Some
            callstack: vec![self.starting_menu_id.unwrap()],
        }
    }
}

impl<'a> Default for MenuBuilder<'a> {
    fn default() -> Self {
        MenuBuilder {
            menus: Vec::new(),
            starting_menu_id: None,
        }
    }
}

pub struct MenuGenie<'a> {
    menus: Vec<Menu<'a>>,
    callstack: Vec<usize>,
}

impl<'a> MenuGenie<'a> {
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

    pub fn prompt(&mut self) -> Option<(usize, usize)> {
        loop {
            // If callstack is empty we should quit the menu
            match self.get_current_menu() {
                Some(current_menu) => {
                    current_menu.prompt();
                    print!("> ");
                    std::io::stdout().flush().unwrap();
                }
                None => return None,
            }

            let mut input = String::new();
            match std::io::stdin().read_line(&mut input) {
                Ok(_) => (),
                Err(e) => println!("{}", e.kind().to_string()),
            };

            match input.trim().parse() {
                Ok(value) => match self.execute_callback(value) {
                    Some(tuple) => return Some(tuple),
                    None => continue,
                },
                Err(_) => {
                    println!("Invalid input.");
                    continue;
                }
            };
        }
    }

    fn execute_callback(&mut self, choosen_key: usize) -> Option<(usize, usize)> {
        let current_menu = self.get_current_menu_unchecked();
        match current_menu.get_menu_item(choosen_key) {
            Some(menu_item) => match menu_item.action {
                MenuAction::Back => {
                    self.back();
                    None
                }
                MenuAction::Quit => {
                    self.quit();
                    None
                }
                MenuAction::BackToStart => {
                    self.back_to_start();
                    None
                }
                MenuAction::Navigate(id) => {
                    self.navigate(id);
                    None
                }
                _ => Some((current_menu.id, choosen_key)),
            },
            None => {
                println!("Invalid input.");
                None
            }
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

pub struct Menu<'a> {
    id: usize,
    menu_items: Vec<MenuItem<'a>>,
}

impl<'a> Menu<'a> {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            menu_items: Vec::new(),
        }
    }

    pub fn get_menu_item(&self, key: usize) -> Option<&MenuItem> {
        self.menu_items
            .iter()
            .find(|&menu_item| menu_item.key == key)
    }

    pub fn prompt(&self) {
        for item in self.menu_items.iter() {
            println!("{item}")
        }
    }
}

#[derive(Debug)]
pub struct MgError {}

#[derive(Debug)]
pub enum MenuAction {
    Back,
    BackToStart,
    Quit,
    Navigate(usize),
    Nothing,
}

pub struct MenuItem<'a> {
    prompt: &'a str,
    action: MenuAction,
    key: usize,
    // pub cb: Box<dyn Fn() -> ()>,
}

impl<'a> Display for MenuItem<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}) {}", self.key, self.prompt)
    }
}

#[cfg(test)]
mod tests {
    use crate::{MenuAction, MenuBuilder};

    #[test]
    fn t1() {
        let mut menu = MenuBuilder::new()
            .with_menu(1)
            .with_menu_item("Manage Nodes", MenuAction::Navigate(2))
            .with_menu_item("Manage Edges", MenuAction::Navigate(3))
            .with_back_button()
            .with_menu(2)
            .with_menu_item("Add Node", MenuAction::Nothing)
            .with_menu_item("Delete Node", MenuAction::Nothing)
            .with_menu_item("Edit Node", MenuAction::Nothing)
            .with_back_button()
            .with_menu(3)
            .with_menu_item("Add Edge", MenuAction::Nothing)
            .with_menu_item("Delete Edge", MenuAction::Nothing)
            .with_menu_item("Edit Edge", MenuAction::Navigate(4))
            .with_back_button()
            .with_menu(4)
            .with_menu_item("Change Items", MenuAction::Nothing)
            .with_menu_item("Change Name", MenuAction::Nothing)
            .with_menu_item("Change Start Node", MenuAction::Nothing)
            .with_menu_item("Change End Node", MenuAction::Nothing)
            .with_menu_item("Back to starting menu", MenuAction::BackToStart)
            .with_back_button()
            .build();

        while let Some(tuple) = menu.prompt() {
            match tuple {
                (2, 1) => println!("Adding node"),
                (2, 2) => println!("Deleting node"),
                (2, 3) => println!("Editing node"),
                (3, 1) => println!("Adding edge"),
                (3, 2) => println!("Deleting edge"),
                (3, 3) => println!("Editing edge"),
                (4, 1) => println!("Changing edge items"),
                (4, 2) => println!("Changing edge name"),
                (4, 3) => println!("Changing edge start node"),
                (4, 4) => println!("Changing edge end node"),
                _ => (),
            }
        }
    }
}
