use super::{Component, app};
use color_eyre::Result;
use crossterm::event::Event;
use ratatui::{
    Frame,
    layout::{Constraint, Direction::Horizontal, Layout, Rect},
    widgets::{Block, Padding, Paragraph},
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
        let container = Block::new().padding(Padding::horizontal(1));
        let container_inner = container.inner(area);

        let lo = Layout::default()
            .direction(Horizontal)
            .constraints([Constraint::Percentage(10), Constraint::Min(0)])
            .split(container_inner);

        self.state = match &app.state_context.current_mode {
            crate::state::VimState::Normal(mode) => "NORMAL".to_string(),
            crate::state::VimState::Command(mode) => "COMMAND".to_string(),
            crate::state::VimState::Insert(mode) => "INSERT".to_string(),
            crate::state::VimState::Visual(mode) => "VISUAL".to_string(),
        };

        frame.render_widget(container, area);
        frame.render_widget(Paragraph::new(self.state.as_str()), lo[0]);
        frame.render_widget(Paragraph::new("..."), lo[1]);
    }
}
