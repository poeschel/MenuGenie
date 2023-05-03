use std::fmt::Display;

pub struct MenuGenie {}

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

    pub fn prompt(&self) -> std::io::Result<()> {
        loop {
            for menu_item in self.menu_items.iter() {
                println!("{menu_item}");
            }

            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;

            let user_choice: usize = match input.parse() {
                Ok(value) => value,
                Err(_) => continue,
            };

            let selected_menu_item =
                match self.menu_items.iter().find(|&item| item.key == user_choice) {
                    Some(menu_item) => menu_item,
                    None => continue,
                };

            match (selected_menu_item.cb)(self) {
                Ok(_) => break,
                Err(_) => break,
            };
        }

        Ok(())
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

pub struct MenuItem {
    prompt: String,
    key: usize,
    pub cb: Box<dyn Fn(&Menu) -> Result<(), MgError>>,
}

impl Display for MenuItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}) {}", self.key, self.prompt)
    }
}

#[cfg(test)]
mod basic_tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::{Menu, MenuItem};

    #[test]
    fn basic() {
        let menu = Rc::new(RefCell::new(Menu::new(1)));
        let mi1 = MenuItem {
            key: 1,
            prompt: "Manage Nodes".into(),
            cb: Box::new(|weak_menu| Ok(())),
        };
        let mi2 = MenuItem {
            key: 2,
            prompt: "Manage Nodes".into(),
            cb: Box::new(|weak_menu| Ok(())),
        };
        let mi3 = MenuItem {
            key: 3,
            prompt: "Manage Nodes".into(),
            cb: Box::new(|weak_menu| Ok(())),
        };

        menu.borrow_mut().insert_menu_item(mi1).unwrap();
        menu.borrow_mut().insert_menu_item(mi2).unwrap();
        menu.borrow_mut().insert_menu_item(mi3).unwrap();
        menu.borrow().prompt().unwrap();
    }
}
