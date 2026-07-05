use crate::state::{State, command::CommandMode, normal::NormalMode, visual::VisualMode};

use super::{Component, app};
use color_eyre::Result;
use crossterm::event::Event;
use ratatui::{
    Frame,
    layout::{Constraint, Direction::Horizontal, Layout, Rect},
    widgets::Paragraph,
};

pub struct VimStatusBar {
    state: String,
    command: String,
    is_active: bool,
}

impl VimStatusBar {
    pub fn new() -> Self {
        VimStatusBar {
            state: "NORMAL".to_string(),
            command: "".to_string(),
            is_active: false,
        }
    }
}

impl Component for VimStatusBar {
    fn handle_event(&mut self, event: &Event) -> Result<Option<()>> {
        // Only react to j/k if this component is active
        if !self.is_active {
            return Ok(None);
        }
        // ... handle vim motions to update self.selected_index ...
        Ok(None)
    }

    fn render(&mut self, frame: &mut Frame, area: Rect, app: &app::App) {
        let lo = Layout::default()
            .direction(Horizontal)
            .constraints([Constraint::Length(2), Constraint::Min(0)])
            .split(area);

        //FIXME: This is very bad and verbose, if its bad here, it'll be bad later, changing it now.
        // if let Some(current_state) = &app
        //     .state_context
        //     .current_mode
        //     .as_any()
        //     .downcast_ref::<NormalMode>()
        // {}

        frame.render_widget(Paragraph::new(self.state.as_str()), area);
    }
}
