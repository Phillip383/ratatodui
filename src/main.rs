mod app;
mod state;
mod tui;
mod types;
mod cli;

use color_eyre::eyre::{ErrReport, Result};
use ratatui::DefaultTerminal;

use app::App;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

use crate::{app::Config, types::AppStatus};

#[tokio::main]
async fn main() -> Result<(), ErrReport> {
    color_eyre::install()?;

    let args = std::env::args();
    if args.len() > 1 {
        //process command
        let config: Config = confy::load("Ratatodui", None)?;
        cli::run(config).await?;

    } else {    
        enable_raw_mode()?;
        ratatui::run(|t| run(t))?;
        ratatui::restore();
        disable_raw_mode()?;
    }
     
    Ok(())
}

pub fn run(terminal: &mut DefaultTerminal) -> Result<(), ErrReport> {
    let config: Config= confy::load("Ratatodui", None)?;
    let mut tui = tui::TUI::new();
    let mut app = App::new(config);

   app.init(); 

    loop {

        if let Ok(lists) = app.init_rx.try_recv() {
            app.lists = lists;
            app.app_status = AppStatus::Idle;
        }

        if let Ok(save_status) = app.save_rx.try_recv() {
            match save_status {
                AppStatus::Idle => app.app_status = save_status,
                AppStatus::Error(e) => {
                    eprintln!("{e}");
                    app.app_status = AppStatus::Idle;
                },
                _ => ()
            }
        }

        terminal.draw(|frame| tui::render(frame, &mut tui, &app))?;
        
        if app.b_quit {
            break Ok(());
        }
        
        
        app.handle_events()?;
    }

}
