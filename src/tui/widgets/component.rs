use ratatui::{Frame, crossterm::event::Event, layout::Rect};

pub trait Component {
    type Msg;

    fn draw(&mut self, frame: &mut Frame, area: Rect);
    fn handle_event(&mut self, event: Event) -> Option<Self::Msg>;
    fn handle_message(&mut self, _msg: Self::Msg) {()}
}

pub trait Focusable {
    fn set_focused(&mut self, focused: bool);

    fn focus(&mut self) {
        self.set_focused(true);
    }

    fn unfocuse(&mut self) {
        self.set_focused(false);
    }

    fn is_focused(&mut self) -> bool;
}
