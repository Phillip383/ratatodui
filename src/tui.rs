mod calendar;
mod calendar_events;
mod editor;
mod todo_lists;
mod todos;
mod vim_status_bar;

use color_eyre::Result;
use crossterm::event::Event;
use ratatui::Frame;
use ratatui::layout::Direction::{Horizontal, Vertical};
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::Style;
use ratatui::widgets::{Block, Borders, Padding, Paragraph};

use crate::app;

pub trait Component {
    fn handle_event(&mut self, event: &Event) -> Result<Option<()>>;

    fn render(&mut self, frame: &mut Frame, area: Rect, app: &app::App);
}

pub fn render(frame: &mut Frame, app: &app::App) {
    let vim_command_layout = Layout::default()
        .direction(Vertical)
        .constraints(vec![Constraint::Min(0), Constraint::Length(1)])
        .split(frame.area());

    let bg_layout = Layout::default()
        .direction(Horizontal)
        .constraints(vec![
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ])
        .split(vim_command_layout[0]);

    let lh_side_layout = Layout::default()
        .direction(Vertical)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(bg_layout[0]);

    let rh_side_layout = Layout::default()
        .direction(Vertical)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(bg_layout[2]);

    let mut editor = editor::Editor::new();
    let mut todos = todos::TodoList::new();
    let mut lists = todo_lists::TodoLists::new();

    editor.render(frame, bg_layout[1], app);
    todos.render(frame, lh_side_layout[0], app);
    lists.render(frame, lh_side_layout[1], app);

    frame.render_widget(border_box("Calender"), rh_side_layout[0]);
    frame.render_widget(border_box("Events"), rh_side_layout[1]);

    frame.render_widget(Paragraph::new("Vim Status Bar"), vim_command_layout[1]);
}

fn border_box(title: &'static str) -> Block<'static> {
    Block::new()
        .title(title)
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_style(Style::new().red())
        .border_type(ratatui::widgets::BorderType::Rounded)
        .merge_borders(ratatui::symbols::merge::MergeStrategy::Exact)
        .padding(Padding::uniform(1))
}
