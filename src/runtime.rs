use crossterm::event::{self, Event};
use tokio::sync::mpsc;

pub fn spawn_event_reader() -> mpsc::UnboundedReceiver<Event> {
    let (sender, receiver) = mpsc::unbounded_channel();

    std::thread::spawn(move || {
        loop {
            match event::read() {
                Ok(event) => {
                    if sender.send(event).is_err() {
                        break;
                    }
                }
                Err(_) => break,
            }
        }
    });

    receiver
}
