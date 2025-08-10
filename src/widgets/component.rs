use ratatui::{Frame, crossterm::event::Event, layout::Rect};

pub trait Component {
    type Msg;

    fn draw(&mut self, frame: &mut Frame, area: Rect);
    fn handle_event(&mut self, event: Event) -> Option<Self::Msg>;
}
