use menu_genie::{MenuAction, MenuBuilder};

fn main() {
    let mut menu = MenuBuilder::new()
        .with_menu(1)
        .with_menu_item("Add Todo", MenuAction::Nothing)
        .with_menu_item("Delete Todo", MenuAction::Nothing)
        .with_menu_item("Edit Todo", MenuAction::Navigate(2))
        .with_back_button()
        .with_menu(2)
        .with_menu_item("Change todo's name", MenuAction::Nothing)
        .with_menu_item("Change todo's description", MenuAction::Nothing)
        .with_menu_item("Change todo's priority", MenuAction::Navigate(3))
        .with_back_button()
        .with_menu(3)
        .with_menu_item("Set priority to Low", MenuAction::Nothing)
        .with_menu_item("Set priority to Medium", MenuAction::Nothing)
        .with_menu_item("Set priority to High", MenuAction::Nothing)
        .with_menu_item("Back to start", MenuAction::BackToStart)
        .with_back_button()
        .build();

    while let Some(tuple) = menu.prompt() {
        match tuple {
            (1, 1) => println!("ACTION: Add Todo"),
            (1, 2) => println!("ACTION: Delete Todo"),
            (2, 1) => println!("ACTION: Change todo's name"),
            (2, 2) => println!("ACTION: Change todo's description"),
            (3, 1) => println!("ACTION: Set priority to Low"),
            (3, 2) => println!("ACTION: Set priority to Medium"),
            (3, 3) => println!("ACTION: Set priority to High"),
            _ => (),
        }
    }

    println!("Bye!")
}
