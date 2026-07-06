use super::{Component, app};
use color_eyre::Result;
use crossterm::event::Event;
use ratatui::{Frame, layout::Rect, widgets::List};

pub struct TodoList {
    items: Vec<String>,
    selected_index: usize,
    is_active: bool,
}

impl TodoList {
    pub fn new() -> Self {
        TodoList {
            //FIXME: Remove these
            items: vec!["asdf".to_string(), "vsdd".to_string(), "wwefx".to_string()],
            selected_index: 0,
            is_active: false,
        }
    }
}

impl Component for TodoList {
    fn handle_event(&mut self, event: &Event) -> Result<Option<()>> {
        // Only react to j/k if this component is active
        if !self.is_active {
            return Ok(None);
        }
        // ... handle vim motions to update self.selected_index ...
        Ok(None)
    }

    fn render(&mut self, frame: &mut Frame, area: Rect, app: &app::App) {
        let list_container = super::border_box("Todos", Some("[C]reate [D]elete [E]dit"));
        let list_inner = list_container.inner(area);

        let list = List::new(&app.lists[app.active_list_item].todos);
        frame.render_widget(list, list_inner);

        frame.render_widget(
            super::border_box("Todos", Some("[C]reate [D]elete [E]dit")),
            area,
        );
    }
}
