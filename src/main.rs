mod app;
mod state;
mod tui;
mod types;

use color_eyre::eyre::{ErrReport, Result};
use ratatui::DefaultTerminal;

use app::App;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

use crate::app::Config;

#[tokio::main]
async fn main() -> Result<(), ErrReport> {
    color_eyre::install()?;
    
    
    enable_raw_mode()?;
    ratatui::run(|t| run(t))?;
    ratatui::restore();
    disable_raw_mode()?;
     
    Ok(())
}

pub fn run(terminal: &mut DefaultTerminal) -> Result<(), ErrReport> {
    let config: Config= confy::load("Ratatodui", None)?;
    let mut tui = tui::TUI::new();
    let mut app = App::new(config);

   app.init(); 

    loop {

        terminal.draw(|frame| tui::render(frame, &mut tui, &app))?;
        
        if app.b_quit {
            break Ok(());
        }
        
        
        app.handle_events()?;
    }

}
