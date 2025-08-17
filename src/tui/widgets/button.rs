use super::component::Component;

enum ButtonMsg {
    Pressed,
}

struct Button {
    text: String,
}

impl Component for Button {
    type Msg = ButtonMsg;

    fn draw(&mut self, frame: &mut ratatui::Frame, area: ratatui::prelude::Rect) {
        todo!()
    }

    fn handle_event(&mut self, event: ratatui::crossterm::event::Event) -> Option<Self::Msg> {
        todo!()
    }
}