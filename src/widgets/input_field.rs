use ratatui::{
    Frame,
    crossterm::event::{Event, KeyCode, KeyEventKind},
    layout::{Position, Rect},
    style::{Color, Stylize},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::widgets::component::Component;

/// Messages produced by InputField
pub enum InputFieldMsg {
    Submited(String),
    Canceled,
}

/// A simple text input field
pub struct InputField {
    /// Input title
    title: String,
    // Text buffer
    buffer: String,
    // Cursor position
    cursor: usize,
}

// Component implementation
impl Component for InputField {
    type Msg = InputFieldMsg;

    fn draw(&mut self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .title(self.title.clone())
            .borders(Borders::all())
            .style(Color::Rgb(80, 80, 80))
            .bg(Color::Rgb(30, 30, 30));

        let input = Paragraph::new(self.buffer.clone()).block(block);

        input.render(area, frame.buffer_mut());

        frame.set_cursor_position(Position::new(
            area.x + 1 + self.cursor_pos() as u16,
            area.y + 1,
        ));
    }

    fn handle_event(&mut self, event: Event) -> Option<InputFieldMsg> {
        let Event::Key(key) = event else {
            return None;
        };
        if key.kind != KeyEventKind::Press {
            return None;
        }

        match key.code {
            KeyCode::Char(c) => self.insert(c),
            KeyCode::Backspace => self.remove_prev_char(),
            KeyCode::Delete => self.remove_next_char(),
            KeyCode::Left => self.cursor_left(),
            KeyCode::Right => self.cursor_right(),
            KeyCode::Enter => Some(InputFieldMsg::Submited(self.buffer.clone())),
            KeyCode::Esc => Some(InputFieldMsg::Canceled),
            _ => None,
        }
    }
}

// Event handlers
impl InputField {
    pub fn insert(&mut self, ch: char) -> Option<InputFieldMsg> {
        self.buffer.insert(self.cursor, ch);
        self.cursor_right();
        None
    }

    pub fn remove_prev_char(&mut self) -> Option<InputFieldMsg> {
        if self.is_empty() {
            return None;
        }

        if self.cursor == 0 {
            return None;
        }

        self.buffer.remove(self.cursor - 1);
        self.cursor_left();
        None
    }

    pub fn remove_next_char(&mut self) -> Option<InputFieldMsg> {
        if self.is_empty() {
            return None;
        }

        if self.cursor >= self.buffer.chars().count() - 1 {
            return None;
        }

        self.buffer.remove(self.cursor + 1);
        None
        // self.cursor_right();
    }

    pub fn cursor_left(&mut self) -> Option<InputFieldMsg> {
        self.cursor = self.cursor.saturating_sub(1);
        None
    }

    pub fn cursor_right(&mut self) -> Option<InputFieldMsg> {
        if self.cursor < self.buffer.chars().count() {
            self.cursor = self.cursor.saturating_add(1);
        }
        None
    }
}

// Manipulation and info
impl InputField {
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    pub fn cursor_pos(&self) -> usize {
        self.cursor
    }

    pub fn clear(&mut self) -> &mut Self {
        self.buffer.clear();
        self.cursor = 0;
        self
    }

    pub fn set_title(&mut self, new_title: &str) -> &mut Self {
        self.title = new_title.to_string();
        self
    }

    pub fn set_input(&mut self, new_input: &str) -> &mut Self {
        self.buffer = String::from(new_input);
        self.cursor = self.buffer.chars().count();
        self
    }

    // pub fn get_input(&self) -> &str {
    //     &self.buffer
    // }
}

// Different built-in traits

impl Default for InputField {
    fn default() -> Self {
        Self {
            title: String::default(),
            buffer: String::default(),
            cursor: usize::default(),
        }
    }
}

impl From<String> for InputField {
    fn from(value: String) -> Self {
        Self {
            title: String::default(),
            buffer: value.clone(),
            cursor: value.chars().count(),
        }
    }
}

impl From<&str> for InputField {
    fn from(value: &str) -> Self {
        Self {
            title: String::default(),
            buffer: String::from(value),
            cursor: value.chars().count(),
        }
    }
}
