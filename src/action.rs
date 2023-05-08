/// MenuAction describes what should be done with the callstack.
#[derive(Debug)]
pub enum MenuAction {
    /// Pops the last menu id from the callstack
    Back,
    /// Removes all but the first element from the callstack
    BackToStart,
    /// Clears the callstack
    Quit,
    /// Pushes menu_id onto the callstack so the next time prompt is called
    /// new menu will be displayed
    Navigate(usize),
    /// Does nothing to the callstack but it returns the tuple containing which menu action
    /// was selected
    Nothing,
}
