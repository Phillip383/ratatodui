use super::Component;
use crate::state::ActiveWidget;
use ratatui::{
    style::Color,
    widgets::{Block, Padding},
};
use tui_textarea::TextArea;

pub struct EditText<'a> {
    area: TextArea<'a>,
    widget: ActiveWidget,
    border_color: Color,
}

impl<'a> EditText<'a> {
    pub fn new(widget_type: ActiveWidget) -> Self {
        EditText {
            area: TextArea::default(),
            widget: widget_type,
            border_color: Color::Red,
        }
    }

    pub fn set_lines(&mut self, lines: Vec<String>) {
        self.area.set_lines(lines, (0, 0));
    }
}

impl<'a> Component for EditText<'a> {
    fn handle_active_state(&mut self, active_widget: &crate::state::ActiveWidget) {
        if *active_widget == self.widget {
            self.border_color = Color::Blue;
        } else {
            //Print Red
            self.border_color = Color::Red;
        }
    }

    fn render(
        &mut self,
        frame: &mut ratatui::prelude::Frame,
        area: ratatui::prelude::Rect,
        app: &crate::app::App,
    ) {
        let block = Block::bordered()
            .border_style(self.border_color)
            .padding(Padding::uniform(1))
            .border_type(ratatui::widgets::BorderType::Rounded);
        self.area.set_block(block);

        frame.render_widget(&self.area, area);
    }
}
