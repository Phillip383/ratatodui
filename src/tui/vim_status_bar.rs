use crate::state::ActiveWidget;

use super::{Component, app};
use ratatui::{
    Frame,
    layout::{Constraint, Direction::Horizontal, Layout, Rect},
    style::Color,
    widgets::{Block, Padding, Paragraph},
};

pub struct VimStatusBar {
    state: String,
    command: String,
    color: Color,
}

impl VimStatusBar {
    pub fn new() -> Self {
        VimStatusBar {
            state: "NORMAL".to_string(),
            command: "".to_string(),
            color: Color::Red,
        }
    }
}

impl Component for VimStatusBar {
    fn handle_active_state(&mut self, active_widget: &crate::state::ActiveWidget) {
        match active_widget {
            ActiveWidget::StatusBar => self.color = Color::Blue,
            _ => self.color = Color::Red,
        };
    }

    fn render(&mut self, frame: &mut Frame, area: Rect, app: &app::App) {
        self.handle_active_state(&app.state_context.active_widget);

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
