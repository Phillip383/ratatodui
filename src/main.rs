mod app;
mod state;
mod tui;
mod types;
mod client;

use color_eyre::eyre::{ErrReport, Result};
use ratatui::DefaultTerminal;

use app::App;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

#[tokio::main]
async fn main() -> Result<(), ErrReport> {
    color_eyre::install()?;
    
    let client = client::Client::new();
    if client.get_token().is_err() {
        loop {
            // Prompt for login credentials
            println!("\nPlease log in to continue.\n");
            let username = rprompt::prompt_reply("Username: ")?;
            let password = rprompt::prompt_reply("Password: ")?;
            if client.login(&username, &password).await.is_ok() {
                break;
            }
        }
    }
    
    enable_raw_mode()?;
    ratatui::run(|t| run(t, client))?;
    ratatui::restore();
    disable_raw_mode()?; 
    Ok(())
}

pub fn run(terminal: &mut DefaultTerminal, client: client::Client) -> Result<(), ErrReport> {
    let mut tui = tui::TUI::new();
    let mut app = App::new(client);

    loop {
        terminal.draw(|frame| tui::render(frame, &mut tui, &app))?;
        app.handle_events()?;
        if app.b_quit {
            break Ok(());
        }
    }

}
