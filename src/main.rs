mod app;
mod db_service;
mod state;
mod todo_service;
mod tui;

use color_eyre::eyre::{ErrReport, Result};
use db_service::connection;
use ratatui::DefaultTerminal;

use app::App;
use state::StateContext;

fn main() -> Result<(), ErrReport> {
    color_eyre::install()?;
    let _ = connection::connect();
    ratatui::run(run)?;
    ratatui::restore();

    Ok(())
}

pub fn run(terminal: &mut DefaultTerminal) -> Result<(), ErrReport> {
    let mut state_context = StateContext::new();

    let app = App::new();

    loop {
        terminal.draw(|frame| tui::render(frame, &app))?;
        //TODO: This will change, for now it quits if some is returned.
        if state_context.handle_events()?.is_some() {
            break Ok(());
        }
    }
}
