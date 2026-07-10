use ratatui::{
    Frame,
    layout::{Constraint, Direction::Vertical, Layout, Rect},
    style::Color,
    widgets::{Block, BorderType, Borders, Padding},
};
use tui_textarea::TextArea;

use crate::types::ActiveWidget;
use crate::{
    app,
    types::ActiveWidget::{EditorTodoDesc, EditorTodoName},
};

use super::Component;

pub struct Editor {
    title_color: Color,
    desc_color: Color,
}

impl Editor {
    pub fn new() -> Self {
        Editor {
            title_color: Color::Red,
            desc_color: Color::Red,
        }
    }
}

impl Component for Editor {
    fn handle_active_state(&mut self, active_widget: &ActiveWidget) {
        match active_widget {
            EditorTodoName => {
                self.title_color = Color::Blue;
                self.desc_color = Color::Red;
            }
            EditorTodoDesc => {
                self.desc_color = Color::Blue;
                self.title_color = Color::Red;
            }
            _ => {
                self.title_color = Color::Red;
                self.desc_color = Color::Red;
            }
        }
    }

    fn render(&mut self, frame: &mut Frame, area: Rect, app: &app::App) {
        self.handle_active_state(&app.active_widget);

        let editor_block = super::border_box(Color::Red, "Ratatodui", Some("[S]ave [C]ancel"));
        let editor_inner = editor_block.inner(area);

        let editor_layout = Layout::default()
            .direction(Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(editor_inner);

        let active_todo = &app.lists[app.active_list_item].todos[app.active_todo];
        let todo_title = active_todo.title.as_str();
        let todo_desc = &active_todo.description;

        let mut title = TextArea::default();
        title.set_placeholder_text("Name");
        title.set_block(
            Block::default()
                .borders(Borders::ALL)
                .style(self.title_color)
                .border_type(BorderType::Rounded)
                .title_top("[N]ame")
                .title_style(Color::LightYellow),
        );

        let mut desc = TextArea::default();
        desc.set_placeholder_text("description...");
        desc.set_block(
            Block::default()
                .borders(Borders::ALL)
                .style(self.desc_color)
                .padding(Padding::uniform(1))
                .border_type(BorderType::Rounded)
                .title_top("[D]escription")
                .title_style(Color::LightYellow),
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
