use crate::menu_item::MenuItem;

pub struct Menu<'a> {
    pub id: usize,
    pub menu_items: Vec<MenuItem<'a>>,
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

        print!("> ");
    }
}
