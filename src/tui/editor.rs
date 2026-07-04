use color_eyre::Result;
use crossterm::event::Event;
use ratatui::{
    Frame,
    layout::{Constraint, Direction::Vertical, Layout, Rect},
    widgets::Paragraph,
};

use crate::app;

use super::Component;

pub struct Editor {
    title: String,
    date: String,
    description: String,
    is_active: bool,
}

impl Editor {
    pub fn new() -> Self {
        Editor {
            title: "Title".to_string(),
            date: "01/19/1993".to_string(),
            description: "Todo description text goes here.".to_string(),
            is_active: false,
        }
    }
}

impl Component for Editor {
    fn handle_event(&mut self, event: &Event) -> Result<Option<()>> {
        // Only react to j/k if this component is active
        if !self.is_active {
            return Ok(None);
        }
        // ... handle vim motions to update self.selected_index ...
        Ok(None)
    }

    fn render(&mut self, frame: &mut Frame, area: Rect, app: &app::App) {
        let editor_block = super::border_box("Ratatodui");
        let editor_inner = editor_block.inner(area);

        let editor_layout = Layout::default()
            .direction(Vertical)
            .constraints(vec![
                Constraint::Length(2),
                Constraint::Min(0),
                Constraint::Length(2),
            ])
            .split(editor_inner);

        frame.render_widget(editor_block, area);
        frame.render_widget(Paragraph::new(self.title.as_str()), editor_layout[0]);
        frame.render_widget(Paragraph::new(self.description.as_str()), editor_layout[1]);
        frame.render_widget(Paragraph::new("[S]ave [C]ancel"), editor_layout[2]);

        frame.render_widget(super::border_box("Ratatodui"), area);
    }
}
