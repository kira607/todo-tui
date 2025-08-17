use ratatui::{
    crossterm::event::Event, text::Line, widgets::{Block, Borders, Paragraph, Widget}
};

use crate::{core::task::Task, utils::date_to_string};
use super::component::Component;

pub enum TaskInfoMsg {}

pub struct TaskInfo {
    task: Option<Task>,
}

impl Component for TaskInfo {
    type Msg = TaskInfoMsg;

    fn draw(&mut self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) {
        let lines: Vec<Line<'_>>;

        if let Some(task) = &self.task {
            lines = vec![
                Line::from(task.title.clone()),
                Line::from(format!("id: {}", task.id)),
                Line::from(format!("done: {}", task.done.to_string())),
                Line::from(format!("created: {}", task.created.to_string())),
                Line::from(format!("scheduled: {}", date_to_string(task.scheduled, "-"))),
            ];
        } else {
            lines = vec![Line::from("No task selected...")];
        }

        let block = Block::new().title("Task info").borders(Borders::ALL);

        Paragraph::new(lines).block(block).render(area, frame.buffer_mut());
    }

    fn handle_event(&mut self, _event: Event) -> Option<Self::Msg> {
        None
    }
}

impl TaskInfo {
    pub fn clear(&mut self) {
        self.task = None;
    }

    pub fn set_task(&mut self, task: &Task) {
        self.task = Some(task.clone());
    }
}

impl Default for TaskInfo {
    fn default() -> Self {
        Self { task: None }
    }
}