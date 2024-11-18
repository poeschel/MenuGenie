use crate::menu::Menu;
use crate::menu_item::MenuItem;
use crate::MenuAction;
use crate::MenuGenie;

/// MenuBuilder provides simple API for creating nested menus.
///
/// We start by creating a menu and give it an id. After that we add menu items to it.
/// Menu items are added to the last added menu so menu must exist before we add any menu
/// items.
/// Menu items have auto-incremented ids starting from 1.
///
/// Special menu items are Back and Quit buttons which have id 0.
///
/// If you need both Back and Quit button add one with "shortcut" method and other with `with_menu_item` method

#[derive(Default)]
pub struct MenuBuilder {
    menus: Vec<Menu>,
    start_menu_id: Option<usize>,
}

impl MenuBuilder {
    /// Creates a new MenuBuilder
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts another menu with the provided id.
    /// Firts time we add a menu its id is set to be starting id.
    pub fn with_menu(mut self, id: usize) -> Self {
        assert!(
            !self.menus.iter().any(|menu| menu.id == id),
            "Menu with id {id} already added"
        );

        self.menus.push(Menu::new(id));

        if self.start_menu_id.is_none() {
            self.start_menu_id = Some(self.menus.len())
        }

        self
    }

    /// Adds a generic menu item with the provided message and action.
    pub fn with_menu_item(mut self, prompt: String, action: MenuAction) -> Self {
        let last_added_menu = self
            .menus
            .last_mut()
            .expect("Menu must be added first before adding menu items");
        last_added_menu.menu_items.push(MenuItem {
            prompt,
            action,
            key: last_added_menu.menu_items.len() + 1,
        });
        self
    }

    /// Adds a menu item with text "Back", action Back and id 0
    pub fn with_back_button(mut self) -> Self {
        let last_added_menu = self
            .menus
            .last_mut()
            .expect("Menu must be added first before adding menu items");
        last_added_menu.menu_items.push(MenuItem {
            prompt: "Back".to_string(),
            action: MenuAction::Back,
            key: 0,
        });
        self
    }

    /// Adds a menu item with text "Quit", action Quit and id 0
    pub fn with_quit_button(mut self) -> Self {
        let last_added_menu = self
            .menus
            .last_mut()
            .expect("Menu must be added first before adding menu items");
        last_added_menu.menu_items.push(MenuItem {
            prompt: "Quit".to_string(),
            action: MenuAction::Quit,
            key: 0,
        });
        self
    }

    /// Changes the starting id so when we build MenuGenie it will start with different menu.
    pub fn with_starting_menu(mut self, starting_menu_id: usize) -> Self {
        self.start_menu_id = Some(starting_menu_id);
        self
    }

    /// Consumes the MenuBulider and creates MenuGenie setting its starting menu to be starting
    /// menu provided in the builder.
    pub fn build(self) -> MenuGenie {
        assert_ne!(self.menus.len(), 0, "No menus added");
        MenuGenie {
            menus: self.menus,
            // UNWRAP its safe to unwrap here bacause if no menu is added we have assert
            // and if there are menus added starting_menu_id will be Some
            start_menu_id: self.start_menu_id.unwrap(),
            callstack: vec![self.start_menu_id.unwrap()],
        }
    }
}
