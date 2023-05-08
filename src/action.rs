#[derive(Debug)]
pub enum MenuAction {
    Back,
    BackToStart,
    Quit,
    Navigate(usize),
    Nothing,
}
