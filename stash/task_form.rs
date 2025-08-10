use crate::{core::Task, widgets::InputField};
use crate::widgets::Component;


// A form for editing a task
#[derive(Default)]
pub struct TaskForm {
    task: Option<Task>,
    title_input: InputField,
    scheduled_input: InputField,
}

impl Component for TaskForm {
    fn draw(&mut self, frame: &mut ratatui::Frame) {
        todo!()
    }

    fn handle_event(&mut self, event: ratatui::crossterm::event::Event) {
        todo!()
    }

    fn get_focused(&self) -> &impl Component {
        todo!()
    }

    fn set_focused(&mut self, component: impl Component) {
        todo!()
    }
}

impl From<Task> for TaskForm {
    fn from(value: Task) -> Self {
        let scheduled_string = match value.scheduled {
            Some(date) => date.to_string(),
            None => String::new(),
        };
        let input = value.title.clone();
        Self {
            task: Some(value),
            title_input: InputField::from(input),
            scheduled_input: InputField::from(scheduled_string),
        }
    }
}
