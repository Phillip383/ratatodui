use ratatui::{
    Frame,
    layout::{Constraint, Direction::Vertical, Layout, Rect},
    style::Color,
    widgets::{Block, BorderType, Borders, Padding},
};
use tui_textarea::TextArea;

use crate::app;
use crate::types::ActiveWidget;

use super::Component;

pub struct Editor {
    color: Color,
}

impl Editor {
    pub fn new() -> Self {
        Editor { color: Color::Red }
    }
}

impl Component for Editor {
    fn handle_active_state(&mut self, active_widget: &ActiveWidget) {
        match active_widget {
            ActiveWidget::Editor(None) => self.color = Color::Blue,
            _ => self.color = Color::Red,
        };
    }

    fn render(&mut self, frame: &mut Frame, area: Rect, app: &app::App) {
        self.handle_active_state(&app.state_context.active_widget);

        let editor_block = super::border_box(self.color, "Ratatodui", Some("[S]ave [C]ancel"));
        let editor_inner = editor_block.inner(area);

        let editor_layout = Layout::default()
            .direction(Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(editor_inner);

        let active_todo =
            &app.lists[app.state_context.active_list_item].todos[app.state_context.active_todo];
        let todo_title = active_todo.title.as_str();
        let todo_desc = active_todo.description.as_deref().unwrap_or("");

        let mut title = TextArea::default();
        title.set_placeholder_text("Name");
        title.set_block(
            Block::default()
                .borders(Borders::ALL)
                .style(Color::Red)
                .border_type(BorderType::Rounded),
        );

        let mut desc = TextArea::default();
        desc.set_placeholder_text("description...");
        desc.set_block(
            Block::default()
                .borders(Borders::ALL)
                .style(Color::Red)
                .padding(Padding::uniform(1))
                .border_type(BorderType::Rounded),
        );

        let text_title: Vec<String> = todo_title.split("\n").map(String::from).collect();
        title.set_lines(text_title, (0, 0));

        let text_desc: Vec<String> = todo_desc.split("\n").map(String::from).collect();
        desc.set_lines(text_desc, (0, 0));

        frame.render_widget(&title, editor_layout[0]);

        frame.render_widget(&desc, editor_layout[1]);
        frame.render_widget(editor_block, area);
    }
}
