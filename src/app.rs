use color_eyre::eyre::{ErrReport, Result};
use ratatui::DefaultTerminal;

use crate::state::StateContext;
use crate::tui;

pub fn run(terminal: &mut DefaultTerminal) -> Result<(), ErrReport> {
    let mut state_context = StateContext::new();

    loop {
        terminal.draw(tui::render)?;
        //TODO: This will change, for now it quits if some is returned.
        if state_context.handle_events()?.is_some() {
            break Ok(());
        }
    }
}
