use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEventKind},
    prelude::Stylize,
    style::{
        Color, Modifier, Style,
        palette::tailwind::{BLUE, GREEN, SLATE},
    },
    symbols,
    text::Line,
    widgets::{Block, Borders, HighlightSpacing, List, ListItem, ListState, StatefulWidget},
};

use crate::{core::Task, widgets::component::Component};

const TODO_HEADER_STYLE: Style = Style::new().fg(SLATE.c100).bg(BLUE.c800);
const NORMAL_ROW_BG: Color = SLATE.c950;
const ALT_ROW_BG_COLOR: Color = SLATE.c900;
const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);
const TEXT_FG_COLOR: Color = SLATE.c200;
const COMPLETED_TEXT_FG_COLOR: Color = GREEN.c500;

pub enum TasksListMsg {
    AddTask,
    EditTask,
    ToggleStatus,
    DeleteTask,
    SelectedNone,
    SelectedNext,
    SelectedPrevious,
    SelectedFirst,
    SelectedLast,
}

/// Tasks list widget
pub struct TasksList {
    items: Vec<Task>,
    state: ListState,
}

impl Component for TasksList {
    type Msg = TasksListMsg;

    fn draw(&mut self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) {
        let block = Block::new()
            .title(Line::raw("TODO List").centered())
            .borders(Borders::TOP)
            .border_set(symbols::border::EMPTY)
            .border_style(TODO_HEADER_STYLE)
            .bg(NORMAL_ROW_BG);

        // Iterate through all elements in the `items` and stylize them.
        let items: Vec<ListItem> = self
            .items
            .iter()
            .enumerate()
            .map(|(i, todo_item)| {
                let color = alternate_colors(i);
                ListItem::from(todo_item).bg(color)
            })
            .collect();

        // Create a List from all list items and highlight the currently selected one
        let list = List::new(items)
            .block(block)
            .highlight_style(SELECTED_STYLE)
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);

        // We need to disambiguate this trait method as both `Widget` and `StatefulWidget` share the
        // same method name `render`.
        StatefulWidget::render(list, area, frame.buffer_mut(), &mut self.state);
    }

    fn handle_event(&mut self, event: Event) -> Option<Self::Msg> {
        let Event::Key(key) = event else {
            return None;
        };
        if key.kind != KeyEventKind::Press {
            return None;
        }
        match key.code {
            KeyCode::Char('a') => Some(Self::Msg::AddTask),
            KeyCode::Char('e') => Some(Self::Msg::EditTask),
            KeyCode::Char('l') | KeyCode::Right | KeyCode::Enter | KeyCode::Char(' ') => {
                Some(Self::Msg::ToggleStatus)
            }
            KeyCode::Char('d') => Some(Self::Msg::DeleteTask),
            KeyCode::Char('h') | KeyCode::Left => Some(Self::Msg::SelectedNone),
            KeyCode::Char('j') | KeyCode::Down => Some(Self::Msg::SelectedNext),
            KeyCode::Char('k') | KeyCode::Up => Some(Self::Msg::SelectedPrevious),
            KeyCode::Char('g') | KeyCode::Home => Some(Self::Msg::SelectedFirst),
            KeyCode::Char('G') | KeyCode::End => Some(Self::Msg::SelectedLast),
            _ => None,
        }
    }
}

// Event handlers
impl TasksList {
    pub fn select_none(&mut self) {
        self.state.select(None);
    }

    pub fn select_next(&mut self) {
        match self.state.selected() {
            None => self.state.select(Some(0)),
            Some(i) => {
                if i + 1 < self.items.len() {
                    self.state.select(Some(i + 1));
                }
            }
        }
    }

    pub fn select_previous(&mut self) {
        match self.state.selected() {
            None => self.state.select(Some(self.items.len().saturating_sub(1))),
            Some(_) => {
                self.state.select_previous();
            }
        }
    }

    pub fn select_first(&mut self) {
        self.state.select_first();
    }

    pub fn select_last(&mut self) {
        self.state.select(Some(self.items.len() - 1));
    }

    pub fn toggle_status(&mut self) -> Option<&Task> {
        if let Some(i) = self.state.selected() {
            self.items[i].done = !self.items[i].done;
            return Some(&self.items[i]);
        }
        None
    }

    pub fn add_task(&mut self, task: &Task) {
        self.items.push(task.clone());
    }
}

impl TasksList {
    pub fn new(items: Vec<Task>) -> Self {
        Self {
            items: items,
            state: ListState::default(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn selected(&self) -> Option<usize> {
        self.state.selected()
    }

    pub fn selected_item(&self) -> Option<&Task> {
        let i = self.state.selected()?;
        self.items.get(i)
    }

    pub fn selected_item_mut(&mut self) -> Option<&mut Task> {
        let i = self.state.selected()?;
        self.items.get_mut(i)
    }

    pub fn remove_selected(&mut self) -> Option<Task> {
        // First remove task (might exit here)
        let i = self.selected()?;
        let task = self.items.get(i)?.clone();
        self.items.remove(i);

        // Then change selected (now we know the remove was ok)
        let new_selected = Some(self.state.selected()?.saturating_sub(1));
        self.state.select(new_selected);

        return Some(task);
    }

    pub fn update_selected(&mut self, new_title: &str) {
        if let Some(task) = self.selected_item_mut() {
            task.title = String::from(new_title);
        }
    }
}

impl From<&Task> for ListItem<'_> {
    fn from(value: &Task) -> Self {
        let line = match value.done {
            true => Line::styled(format!(" ✓ {}", value.title), COMPLETED_TEXT_FG_COLOR),
            false => Line::styled(format!(" ☐ {}", value.title), TEXT_FG_COLOR),
        };
        ListItem::new(line)
    }
}

const fn alternate_colors(i: usize) -> Color {
    if i % 2 == 0 {
        NORMAL_ROW_BG
    } else {
        ALT_ROW_BG_COLOR
    }
}
