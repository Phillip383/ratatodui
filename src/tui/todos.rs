use crate::types::ActiveWidget;

use super::{Component, app};
use ratatui::{Frame, layout::Rect, style::Color, widgets::List};

pub struct TodoList {
    color: Color,
}

impl TodoList {
    pub fn new() -> Self {
        TodoList { color: Color::Red }
    }
}

impl Component for TodoList {
    fn handle_active_state(&mut self, active_widget: &ActiveWidget) {
        match active_widget {
            ActiveWidget::Todos(None) => self.color = Color::Blue,
            _ => self.color = Color::Red,
        };
    }

    fn render(&mut self, frame: &mut Frame, area: Rect, app: &app::App) {
        self.handle_active_state(&app.state_context.active_widget);

        let list_container =
            super::border_box(self.color, "[T]odos", Some("[C]reate [D]elete [E]dit"));
        let list_inner = list_container.inner(area);

        let list = List::new(&app.lists[app.state_context.active_list_item].todos);
        frame.render_widget(list, list_inner);

        frame.render_widget(list_container, area);
    }
}
