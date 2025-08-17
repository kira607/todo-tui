use crate::{core::task::Task, utils::date_to_string};

use super::{
    component::{Component, Focusable},
    input_field::InputField,
};
use ratatui::crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::{
    prelude::*,
    style::Stylize,
    widgets::{Block, Borders},
};

pub enum TaskFormMsg {
    Submited(Task),
    Canceled,
}

pub enum ActiveField {
    Title,
    Scheduled,
}

impl Default for ActiveField {
    fn default() -> Self {
        Self::Title
    }
}

pub struct TaskForm {
    task: Option<Task>,
    title_input: InputField,
    scheduled_input: InputField,
    active_field: ActiveField,
}

impl TaskForm {
    fn update_focus(&mut self) {
        match self.active_field {
            ActiveField::Title => {
                self.title_input.focus();
                self.scheduled_input.unfocuse();
            }
            ActiveField::Scheduled => {
                self.scheduled_input.focus();
                self.title_input.unfocuse();
            }
        };
    }
}

impl Component for TaskForm {
    type Msg = TaskFormMsg;

    fn draw(&mut self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) {
        self.update_focus();
        let block = Block::default()
            .borders(Borders::ALL)
            .title("Task Form")
            .bg(Color::Rgb(40, 40, 40));

        let inner_area = block.inner(area);
        frame.render_widget(block, area);

        let chunks = ratatui::layout::Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Length(3), Constraint::Length(3)])
            .split(inner_area);

        self.title_input.draw(frame, chunks[0]);
        self.scheduled_input.draw(frame, chunks[1]);
    }

    fn handle_event(&mut self, event: Event) -> Option<Self::Msg> {
        let Event::Key(key) = event else {
            return None;
        };
        if key.kind != KeyEventKind::Press {
            return None;
        }
        match key.code {
            KeyCode::Esc => return Some(Self::Msg::Canceled),
            KeyCode::Enter | KeyCode::Tab => return self.switch_next(),
            KeyCode::BackTab => return self.switch_prev(),
            _ => (),
        };
        match self.active_field {
            ActiveField::Title => self.title_input_update(event),
            ActiveField::Scheduled => self.scheduled_field_update(event),
        }
    }
}

// Event handlers
impl TaskForm {
    fn switch_next(&mut self) -> Option<<TaskForm as Component>::Msg> {
        self.active_field = match self.active_field {
            ActiveField::Title => ActiveField::Scheduled,
            ActiveField::Scheduled => ActiveField::Scheduled,
        };
        self.update_focus();
        None
    }

    fn switch_prev(&mut self) -> Option<<TaskForm as Component>::Msg> {
        self.active_field = match self.active_field {
            ActiveField::Title => ActiveField::Title,
            ActiveField::Scheduled => ActiveField::Title,
        };
        self.update_focus();
        None
    }

    fn title_input_update(&mut self, event: Event) -> Option<<TaskForm as Component>::Msg> {
        self.title_input.handle_event(event);
        None
    }

    fn scheduled_field_update(&mut self, event: Event) -> Option<<TaskForm as Component>::Msg> {
        self.scheduled_input.handle_event(event);
        None
    }
}

impl Default for TaskForm {
    fn default() -> Self {
        Self {
            task: Default::default(),
            title_input: InputField::new("Task title", None),
            scheduled_input: InputField::new("Scheduled date", None),
            active_field: Default::default(),
        }
    }
}

impl From<&Task> for TaskForm {
    fn from(value: &Task) -> Self {
        Self {
            task: Some(value.clone()),
            title_input: InputField::new("Task title", Some(&value.title)),
            scheduled_input: InputField::new(
                "Scheduled date",
                Some(&date_to_string(value.scheduled, "")),
            ),
            active_field: Default::default(),
        }
    }
}
