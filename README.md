# MenuGenie

Rust library for managing multi-level terminal menus. It provides simple builder API for creating nested menus.

## Usage

A simple example application [simple.rs](./examples/simple.rs)

### Menu actions

There are several menu actions that can execute when user selects a menu item

- Back - Goes back to the previous menu
- BackToStart - Goes back to the firts menu
- Quit - Quits the menu
- Navigate - Navigates to the menu with provided id
- Nothing - Used when we actualy want to execute some other code.

When the menu encounters the Nothing action it will return with `Ok(Some((menu_id, menu_item_id)))` so we can match against the returned touple. See the [simple.rs](./examples/simple.rs) example.

### Builder

Builder starts of with empty vector of menus. 

When we call `with_menu(id)` we insert another menu with the provided id. 

`with_menu_item(prompt, action)` adds a generic menu item with the provided message and action. Each menu item has an id that is auto-generated when menu item is added to the menu. That menu item id is used when user makes a selection. *Menu items are added to the last added menu so menu must exist before we add any menu items.* 

`with_back_button()` adds a menu item with text "Back", action Back and id 0.

`with_quit_button()` adds a menu item with text "Quit", action Quit and id 0.

> **_Note_** If you want a back and a quit button at the same time you can add one with provided "shortcut" function and other with `with_menu_item` function.

Starting menu id is set when first menu is added but it can be modified with `with_starting_menu` function.

Finally to get a `MenuGenie` instance simply call `build`.

### MenuGenie struct

MenuGenie struct holds the menus and the starting menu id. It works by saving a callstack of menu ids. 

It has two functions for running the menu:

- `run` Function reads the last menu id from the callstack and prompts the user. Maybe we started a menu and an error happend, we can continue from the last menu that was displayed by just calling `run` again.

- `restart` Function first clears the callstack and puts starting menu id back on it, then calls `run`. It's useful if we want to start the menu from the beggining again.

> **_Note_** when we call `build` on `MenuBuilder` it places starting menu id onto the callstack so we can just call run.

## Future

It the future a plan on adding more features and customizations to the library. If you have some ideas fell free to let me know.
These are some ideas that came to my mind: 

- [ ] Configuration for setting the default quit and back button texts, Callbacks for automatically handling input and parsing errors, etc.
- [ ] Dynamic menu item that is controlled by some condition. For example we want to display some menu item if user is logged in or not.

## Contributions

Contributions and suggestions are always welcome. Fell free to participate in this project.
