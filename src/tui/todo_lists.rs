use super::{Component, app};
use color_eyre::Result;
use crossterm::event::Event;
use ratatui::{Frame, layout::Rect};

pub struct TodoLists {
    is_active: bool,
    items: Vec<String>,
    selected_index: usize,
}

impl TodoLists {
    pub fn new() -> Self {
        TodoLists {
            is_active: false,
            items: vec!["asdfa".to_string(), "dsfs".to_string(), "werf".to_string()],
            selected_index: 0,
        }
    }
}

impl Component for TodoLists {
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
        frame.render_widget(
            super::border_box("Lists", Some("[C]reate [D]elete [S]elect")),
            area,
        );
    }
}
