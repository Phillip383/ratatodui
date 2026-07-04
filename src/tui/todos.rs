use super::Component;
use color_eyre::Result;
use crossterm::event::Event;
use ratatui::{Frame, layout::Rect};

pub struct TodoList {
    items: Vec<String>,
    selected_index: usize, // UI state lives HERE, not in a global context
    is_active: bool,
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

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        // Draw the list using self.items and self.selected_index
    }
}
