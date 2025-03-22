use crate::app::{App, AppResult};
use crossterm::event::KeyEvent;

/// Handles the key events and updates the state of [`App`].
pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    match key_event.code {
        // TODO: call the proper method on app instance for each event you want to handle
        // eq: arrow keys pressed, enter key, scroll...
        // TODO: define actions for quitting the app
        crossterm::event::KeyCode::Char('q') => app.running = false,
        _ => {}
    }
    Ok(())
}
