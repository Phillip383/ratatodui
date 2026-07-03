use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Style, Stylize};
use ratatui::widgets::calendar::{CalendarEventStore, Monthly};
use ratatui::widgets::{
    Block, BorderType, Borders, List, Padding, Paragraph, Scrollbar, ScrollbarOrientation,
    ScrollbarState, Table, Tabs,
};
use ratatui::{DefaultTerminal, Frame};

pub fn run(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(render)?;
        if crossterm::event::read()?.is_key_press() {
            //TODO: Add Input handling in module
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    let main_layout = Layout::horizontal([Constraint::Percentage(25), Constraint::Fill(1)]);
    let left_layout = Layout::vertical([Constraint::Percentage(60), Constraint::Fill(1)]);
    let right_layout = Layout::vertical([Constraint::Percentage(95), Constraint::Fill(1)]);

    frame.render_widget(background(), frame.area());
}

fn background() -> Block<'static> {
    Block::new()
        .title("Ratatodui")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_style(Style::new().red())
        .border_type(ratatui::widgets::BorderType::Rounded)
        .merge_borders(ratatui::symbols::merge::MergeStrategy::Exact)
        .padding(Padding::uniform(2))
}
