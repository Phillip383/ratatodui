pub mod editor;
pub mod todo_lists;
pub mod todos;
pub mod vim_status_bar;

use ratatui::Frame;
use ratatui::layout::Direction::{Horizontal, Vertical};
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::Color;
use ratatui::widgets::{Block, Borders, Padding};

use crate::app;
use crate::state::ActiveWidget::{self};

pub struct TUI {
    pub editor: editor::Editor,
    pub todos: todos::TodoList,
    pub lists: todo_lists::TodoLists,
    pub status_bar: vim_status_bar::VimStatusBar,
}

impl TUI {
    pub fn new() -> Self {
        TUI {
            editor: editor::Editor::new(),
            todos: todos::TodoList::new(),
            lists: todo_lists::TodoLists::new(),
            status_bar: vim_status_bar::VimStatusBar::new(),
        }
    }
}

pub trait Component {
    fn handle_active_state(&mut self, active_widget: &ActiveWidget);

    fn render(&mut self, frame: &mut Frame, area: Rect, app: &app::App);
}

pub fn render(frame: &mut Frame, tui: &mut TUI, app: &app::App) {
    let vim_command_layout = Layout::default()
        .direction(Vertical)
        .constraints(vec![Constraint::Min(0), Constraint::Length(1)])
        .split(frame.area());

    let bg_layout = Layout::default()
        .direction(Horizontal)
        .constraints(vec![Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(vim_command_layout[0]);

    let lh_side_layout = Layout::default()
        .direction(Vertical)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(bg_layout[0]);

    tui.editor.render(frame, bg_layout[1], app);
    tui.todos.render(frame, lh_side_layout[0], app);
    tui.lists.render(frame, lh_side_layout[1], app);
    tui.status_bar.render(frame, vim_command_layout[1], app);
}

fn border_box(
    color: Color,
    title: &'static str,
    bottom_title: Option<&'static str>,
) -> Block<'static> {
    let bottom_title = bottom_title.unwrap_or_default();

    Block::new()
        .title_top(title)
        .title_alignment(Alignment::Left)
        .title_bottom(bottom_title)
        .title_style(Color::LightYellow)
        .borders(Borders::ALL)
        .border_style(color)
        .border_type(ratatui::widgets::BorderType::Rounded)
        .merge_borders(ratatui::symbols::merge::MergeStrategy::Exact)
        .padding(Padding::uniform(1))
}
