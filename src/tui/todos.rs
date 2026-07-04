use super::{Component, app};
use color_eyre::Result;
use crossterm::event::Event;
use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::Style,
    widgets::{Block, Borders, Padding},
};

pub struct TodoList {
    items: Vec<String>,
    selected_index: usize, // UI state lives HERE, not in a global context
    pub is_active: bool,
}

impl TodoList {
    pub fn new() -> Self {
        TodoList {
            //FIXME: Remove these
            items: vec!["asdf".to_string(), "vsdd".to_string(), "wwefx".to_string()],
            selected_index: 0,
            is_active: false,
        }
    }
}

impl Component for TodoList {
    fn handle_event(&mut self, event: &Event) -> Result<Option<()>> {
        // Only react to j/k if this component is active
        if !self.is_active {
            return Ok(None);
        }
        // ... handle vim motions to update self.selected_index ...
        Ok(None)
    }

    fn render(&mut self, frame: &mut Frame, area: Rect, app: &app::App) {
        // Draw the list using self.items and self.selected_index
        frame.render_widget(border_box("Todos"), area);
    }
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
