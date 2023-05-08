use crate::menu::Menu;
use crate::menu_item::MenuItem;
use crate::MenuAction;
use crate::MenuGenie;

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
            start_menu_id: self.starting_menu_id.unwrap(),
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
