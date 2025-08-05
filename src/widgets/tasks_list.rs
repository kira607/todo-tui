use ratatui::{
    prelude::Stylize,
    style::{
        Color, Modifier, Style,
        palette::tailwind::{BLUE, GREEN, SLATE},
    },
    symbols,
    text::Line,
    widgets::{
        Block, Borders, HighlightSpacing, List, ListItem, ListState, StatefulWidget, Widget,
    },
};

use crate::core::Task;

const TODO_HEADER_STYLE: Style = Style::new().fg(SLATE.c100).bg(BLUE.c800);
const NORMAL_ROW_BG: Color = SLATE.c950;
const ALT_ROW_BG_COLOR: Color = SLATE.c900;
const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);
const TEXT_FG_COLOR: Color = SLATE.c200;
const COMPLETED_TEXT_FG_COLOR: Color = GREEN.c500;

pub struct TasksList {
    items: Vec<Task>,
    state: ListState,
}

impl TasksList {
    pub fn new(items: Vec<Task>) -> Self {
        Self {
            items: items,
            state: ListState::default(),
        }
    }

    pub fn selected(&self) -> Option<usize> {
        self.state.selected()
    }

    pub fn selected_item(&self) -> Option<&Task> {
        if let Some(i) = self.selected() {
            self.items.get(i)
        } else {
            None
        }
    }

    pub fn selected_item_mut(&mut self) -> Option<&mut Task> {
        if let Some(i) = self.selected() {
            self.items.get_mut(i)
        } else {
            None
        }
    }

    pub fn select_none(&mut self) {
        self.state.select(None);
    }

    pub fn select_next(&mut self) {
        self.state.select_next();
    }

    pub fn select_previous(&mut self) {
        self.state.select_previous();
    }

    pub fn select_first(&mut self) {
        self.state.select_first();
    }

    pub fn select_last(&mut self) {
        self.state.select_last();
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

impl Widget for &mut TasksList {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
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
        StatefulWidget::render(list, area, buf, &mut self.state);
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
