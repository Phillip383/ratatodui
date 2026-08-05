mod app;
mod state;
mod tui;
mod types;

use color_eyre::eyre::{ErrReport, Result};
use ratatui::DefaultTerminal;

use app::App;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

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
    let mut tui = tui::TUI::new();
    let mut app = App::new();

   app.init(); 

    loop {

        
        if let Ok(result) = app.save_rx.try_recv() {
            match result {
                Ok(_) => app.save_status = types::SaveStatus::Success,
                Err(msg) => app.save_status = types::SaveStatus::Error(msg)
            }
        }

        terminal.draw(|frame| tui::render(frame, &mut tui, &app))?;
        
        if app.b_quit {
            break Ok(());
        }
        
        
        app.handle_events()?;
    }

}
