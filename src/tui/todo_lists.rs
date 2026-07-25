use crate::types::ActiveWidget;

use super::{Component, app};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{List, ListState},
};

pub struct TodoLists {
    color: Color,
    state: ListState,
}

impl TodoLists {
    pub fn new() -> Self {
        TodoLists {
            color: Color::Red,
            state: ListState::default().with_selected(Some(0)),
        }
    }
}

impl Component for TodoLists {
    fn handle_active_state(&mut self, active_widget: &ActiveWidget) {
        match active_widget {
            ActiveWidget::Lists(None) => self.color = Color::Blue,
            _ => self.color = Color::Red,
        };
    }

    fn render(&mut self, frame: &mut Frame, area: Rect, app: &app::App) {
        self.handle_active_state(&app.active_widget);
        self.state.select(Some(app.active_list_item));

        let list_container =
            super::border_box(self.color, "[L]ists", Some("[C]reate [D]elete [S]elect"));
        let list_inner = list_container.inner(area);

        let list = List::new(&app.lists)
            .highlight_style(Style::new().red())
            .highlight_symbol(">> ");

        frame.render_stateful_widget(list, list_inner, &mut self.state);

        frame.render_widget(list_container, area);
    }
}
