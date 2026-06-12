use crossterm::event::{KeyEvent, MouseEvent};
use ratatui::{Frame, layout::Rect};

pub trait Component {
    type Message;

    fn render(&mut self, frame: &mut Frame, area: Rect);

    fn handle_key_event(&mut self, _event: KeyEvent) -> Option<Self::Message> {
        None
    }

    fn handle_mouse_event(&mut self, _event: MouseEvent) -> Option<Self::Message> {
        None
    }

    fn tick(&mut self) -> Option<Self::Message> {
        None
    }
}
