use crate::state::ActiveWidget;

use super::{Component, app};
use ratatui::{Frame, layout::Rect, style::Color, widgets::List};

pub struct TodoList {
    items: Vec<String>,
    selected_index: usize,
    color: Color,
}

impl TodoList {
    pub fn new() -> Self {
        TodoList {
            //FIXME: Remove these
            items: vec!["asdf".to_string(), "vsdd".to_string(), "wwefx".to_string()],
            selected_index: 0,
            color: Color::Red,
        }
    }
}

impl Component for TodoList {
    fn handle_active_state(&mut self, active_widget: &crate::state::ActiveWidget) {
        match active_widget {
            ActiveWidget::Todos => self.color = Color::Blue,
            _ => self.color = Color::Red,
        };
    }

    fn render(&mut self, frame: &mut Frame, area: Rect, app: &app::App) {
        self.handle_active_state(&app.state_context.active_widget);

        let list_container =
            super::border_box(self.color, "[T]odos", Some("[C]reate [D]elete [E]dit"));
        let list_inner = list_container.inner(area);

        let list = List::new(&app.lists[app.active_list_item].todos);
        frame.render_widget(list, list_inner);

        frame.render_widget(list_container, area);
    }
}
