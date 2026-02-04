pub mod handler;
pub mod tick;

pub use handler::EventHandler;

use crossterm::event::{KeyEvent, MouseEvent};

#[derive(Debug, Clone)]
pub enum Event {
    Tick,
    Key(KeyEvent),
    #[allow(dead_code)]
    Mouse(MouseEvent),
    #[allow(dead_code)]
    Resize(u16, u16),
}
