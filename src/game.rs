use piston_window::*;
use std::net::{TcpStream, Shutdown};

/// Stores game state of event loop.
pub struct Game {
    pub title: &'static str,
    pub exit_button: Button,
    pub stream: TcpStream,
}

impl Game {
    pub fn run(&mut self, window: &mut PistonWindow) {
        window.set_title(self.title.into());
        while let Some(e) = window.next() {
            window.draw_2d(&e, |_c, g, _| {
                clear([0.0, 0.0, 0.0, 1.0], g);
            });
            if let Some(button) = e.press_args() {
                if button == self.exit_button {
                    self.stream.shutdown(Shutdown::Both).expect("shutdown failed");
                    break;
                }
            }
        }
    }
}
