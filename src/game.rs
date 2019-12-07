use super::*;
use piston_window::*;
use std::net::{Shutdown, TcpStream};

pub struct Game {
    pub title: &'static str,
    pub exit_button: Button,
    pub stream: TcpStream,
}

impl Game {
    pub fn run(&mut self, window: &mut PistonWindow, glyphs: &mut Glyphs) {
        let state = GameState::new();
        window.set_title(self.title.into());
        while let Some(e) = window.next() {
            state.render(window, &e, glyphs);
            if let Some(button) = e.press_args() {
                if button == self.exit_button {
                    self.stream
                        .shutdown(Shutdown::Both)
                        .expect("shutdown failed");
                    break;
                }
            }
        }
    }
}
