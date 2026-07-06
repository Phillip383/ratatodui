use ratatui::{
    Frame,
    layout::{Constraint, Direction::Vertical, Layout, Rect},
    style::Color,
    widgets::Paragraph,
};

use crate::{app, state::ActiveWidget};

use super::Component;

pub struct Editor {
    title: String,
    date: String,
    description: String,
    color: Color,
}

impl Editor {
    pub fn new() -> Self {
        Editor {
            title: "Title".to_string(),
            date: "01/19/1993".to_string(),
            description: "Todo description text goes here.".to_string(),
            color: Color::Red,
        }
    }
}

impl Component for Editor {
    fn handle_active_state(&mut self, active_widget: &crate::state::ActiveWidget) {
        match active_widget {
            ActiveWidget::Editor => self.color = Color::Blue,
            _ => self.color = Color::Red,
        };
    }

    fn render(&mut self, frame: &mut Frame, area: Rect, app: &app::App) {
        self.handle_active_state(&app.state_context.active_widget);

        let editor_block = super::border_box(self.color, "Ratatodui", Some("[S]ave [C]ancel"));
        let editor_inner = editor_block.inner(area);

        let editor_layout = Layout::default()
            .direction(Vertical)
            .constraints([Constraint::Length(2), Constraint::Min(0)])
            .split(editor_inner);

        let active_todo = &app.lists[app.active_list_item].todos[app.active_todo];

        let todo_title = active_todo.title.as_str();
        let todo_desc = active_todo.description.as_deref().unwrap_or("");

        frame.render_widget(Paragraph::new(todo_title), editor_layout[0]);
        frame.render_widget(Paragraph::new(todo_desc), editor_layout[1]);

        frame.render_widget(editor_block, area);
    }
}
