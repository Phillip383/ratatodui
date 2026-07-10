mod app;
mod state;
mod tui;
mod types;

use color_eyre::eyre::{ErrReport, Result};
use ratatui::DefaultTerminal;

use app::App;

fn main() -> Result<(), ErrReport> {
    color_eyre::install()?;
    ratatui::run(run)?;
    ratatui::restore();

    Ok(())
}

pub fn run(terminal: &mut DefaultTerminal) -> Result<(), ErrReport> {
    let mut tui = tui::TUI::new();
    let state_context = state::StateContext::new();
    let mut app = App::new(state_context);

    loop {
        terminal.draw(|frame| tui::render(frame, &mut tui, &app))?;
        app.state_context.handle_events()?;
        if app.state_context.b_quit {
            break Ok(());
        }
    }
}
