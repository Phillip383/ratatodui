use crate::types::ActiveWidget;

use super::{Component, app};
use ratatui::{
    Frame, layout::{Constraint, Direction::Horizontal, Layout, Rect}, style::{Color, Style}, widgets::{Block, Padding, Paragraph},
};
use tui_textarea::TextArea;

pub struct VimStatusBar {
    state: String,
    color: Color,
}

impl VimStatusBar {
    pub fn new() -> Self {
        VimStatusBar {
            state: "NORMAL".to_string(),
            color: Color::Red,
        }
    }
}

impl Component for VimStatusBar {
    fn handle_active_state(&mut self, active_widget: &ActiveWidget) {
        match active_widget {
            ActiveWidget::StatusBar => self.color = Color::Blue,
            _ => self.color = Color::Red,
        };
    }

    fn render(&mut self, frame: &mut Frame, area: Rect, app: &app::App) {
        self.handle_active_state(&app.active_widget);

        let container = Block::new().padding(Padding::horizontal(1));
        let container_inner = container.inner(area);

        let lo = Layout::default()
            .direction(Horizontal)
            .constraints([Constraint::Percentage(10), Constraint::Min(0)])
            .split(container_inner);

        self.state = match &app.current_mode {
            crate::state::VimState::Normal(_mode) => "NORMAL".to_string(),
            crate::state::VimState::Command(_mode) => "COMMAND".to_string(),
            crate::state::VimState::Insert(_mode) => "INSERT".to_string(),
            crate::state::VimState::Visual(_mode) => "VISUAL".to_string(),
        };

        let mut command = TextArea::default();
        command.insert_str(&app.command_buffer);
        command.set_cursor_render_mode(tui_textarea::CursorRenderMode::Hidden);
        command.set_style(Style::new().fg(self.color));

        frame.render_widget(container, area);
        frame.render_widget(Paragraph::new(self.state.as_str()), lo[0]);
        frame.render_widget(&command, lo[1]);
    }
}
