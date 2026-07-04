mod app;
mod db_service;
mod state;
mod todo_service;
mod tui;

use color_eyre::eyre::{ErrReport, Ok, Result};
use db_service::connection;

fn main() -> Result<(), ErrReport> {
    color_eyre::install()?;
    let _ = connection::connect();
    ratatui::run(app::run)?;
    ratatui::restore();
    Ok(())
}
