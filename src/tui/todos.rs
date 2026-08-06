use crate::types::ActiveWidget;

use super::{Component, app};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{List, ListState},
};

pub struct TodoList {
    color: Color,
    state: ListState,
}

impl TodoList {
    pub fn new() -> Self {
        TodoList {
            color: Color::Red,
            state: ListState::default().with_selected(Some(0)),
        }
    }
}

impl Component for TodoList {
    fn handle_active_state(&mut self, active_widget: &ActiveWidget) {
        match active_widget {
            ActiveWidget::Todos => self.color = Color::Blue,
            _ => self.color = Color::Red,
        };
    }

    fn render(&mut self, frame: &mut Frame, area: Rect, app: &app::App) {
        self.handle_active_state(&app.active_widget);
        self.state.select(Some(app.active_todo));

        let list_container =
            super::border_box(self.color, "[T]odos", Some("[C]reate [R]emove [c]omplete"));
        let list_inner = list_container.inner(area);

        let Some(active_list) = app.lists.get(app.active_list_item) else {
            frame.render_widget(list_container, area);
            return;
        };

        let list = List::new(&active_list.todos)
            .highlight_style(Style::new().red())
            .highlight_symbol(">> ");
        frame.render_stateful_widget(list, list_inner, &mut self.state);

        frame.render_widget(list_container, area);
    }
}
