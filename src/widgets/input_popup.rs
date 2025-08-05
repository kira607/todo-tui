use ratatui::{
    layout::Constraint,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::utils::center;

pub struct InputPopup {
    title: String,
    input: String,
}

impl Default for InputPopup {
    fn default() -> Self {
        Self {
            title: "Input".to_string(),
            input: String::default(),
        }
    }
}

impl InputPopup {
    pub fn clear(&mut self) -> &mut Self {
        self.input.clear();
        self
    }

    pub fn set_title(&mut self, new_title: &str) -> &mut Self {
        self.title = new_title.to_string();
        self
    }

    pub fn set_input(&mut self, new_input: &str) -> &mut Self {
        self.input = String::from(new_input);
        self
    }

    pub fn get_input(&self) -> &str {
        &self.input
    }

    pub fn push(&mut self, ch: char) {
        self.input.push(ch);
    }

    pub fn pop(&mut self) {
        self.input.pop();
    }
}

impl Widget for &mut InputPopup {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let popup_area = center(area, Constraint::Percentage(60), Constraint::Percentage(40));

        // let layout = Layout::default().direction(Direction::Vertical).margin(1);

        let block = Block::default()
            .title(self.title.clone())
            .borders(Borders::all())
            .style(Style::default().bg(Color::DarkGray));

        let input = Paragraph::new(self.input.clone()).block(block);

        input.render(popup_area, buf);
    }
}
