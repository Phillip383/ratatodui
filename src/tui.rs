use ratatui::layout::Direction::{Horizontal, Vertical};
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
    let bg_layout = Layout::default()
        .direction(Horizontal)
        .constraints(vec![
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ])
        .split(frame.area());

    let lh_side_layout = Layout::default()
        .direction(Vertical)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(bg_layout[0]);

    let rh_side_layout = Layout::default()
        .direction(Vertical)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(bg_layout[2]);

    frame.render_widget(border_box(("Ratatodui")), bg_layout[1]);

    frame.render_widget(border_box("Todos"), lh_side_layout[0]);
    frame.render_widget(border_box("Lists"), lh_side_layout[1]);

    frame.render_widget(border_box("Calender"), rh_side_layout[0]);
    frame.render_widget(border_box("Events"), rh_side_layout[1]);
}

fn border_box(title: &'static str) -> Block<'static> {
    Block::new()
        .title(title)
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_style(Style::new().red())
        .border_type(ratatui::widgets::BorderType::Rounded)
        .merge_borders(ratatui::symbols::merge::MergeStrategy::Exact)
        .padding(Padding::uniform(2))
}
