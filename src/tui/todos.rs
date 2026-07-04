use super::{Component, app, app::TodoList};
use color_eyre::Result;
use crossterm::event::Event;
use ratatui::{Frame, layout::Rect};

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
    }
}
