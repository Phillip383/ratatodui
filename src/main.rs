mod app;
mod state;
mod tui;
mod types;

use color_eyre::eyre::{ErrReport, Result};
use ratatui::DefaultTerminal;

use app::App;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

fn main() -> Result<(), ErrReport> {
    enable_raw_mode()?;
    color_eyre::install()?;
    ratatui::run(run)?;
    ratatui::restore();
    disable_raw_mode()?;

    Ok(())
}

pub fn run(terminal: &mut DefaultTerminal) -> Result<(), ErrReport> {
    let mut tui = tui::TUI::new();
    let mut app = App::new();

    loop {
        terminal.draw(|frame| tui::render(frame, &mut tui, &app))?;
        app.handle_events()?;
        if app.b_quit {
            break Ok(());
        }
    }
}
