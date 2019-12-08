use super::*;
use piston_window::*;
use std::io::*;
use std::net::{Shutdown, TcpStream};

pub struct Game {
    pub title: &'static str,
    pub exit_button: Button,
    pub stream: TcpStream,
    pub is_left: bool,
}

impl Game {
    pub fn run(&mut self, window: &mut PistonWindow, glyphs: &mut Glyphs) {
        self.stream.set_nodelay(true).expect("set nodelay failed");
        self.stream
            .set_nonblocking(false)
            .expect("set blocking mode failed");
        let mut movement: [u8; 1] = [0];
        let mut buf = [0; 10];
        let mut state = GameState::new();
        window.set_title(self.title.into());
        while let Some(e) = window.next() {
            state.render(window, &e, glyphs);
            if let Some(button) = e.press_args() {
                if button == self.exit_button {
                    self.stream
                        .shutdown(Shutdown::Both)
                        .expect("shutdown failed");
                    break;
                } else if button == Button::Keyboard(Key::Up) {
                    if self.is_left {
                        state.left_pos -= 5;
                    } else {
                        state.right_pos -= 5;
                    }
                    movement[0] = 1;
                } else if button == Button::Keyboard(Key::Down) {
                    if self.is_left {
                        state.left_pos += 5;
                    } else {
                        state.right_pos += 5;
                    }
                    movement[0] = 2;
                }
            }

            self.stream.write(&movement).unwrap();
            let mut len = self.stream.peek(&mut buf).expect("peek failed");

            if len != 0 {
                len = self.stream.read(&mut buf).expect("read failed");
                if self.is_left {
                    if buf[len - 1] == 1 {
                        state.right_pos -= 5;
                    } else if buf[len - 1] == 2 {
                        state.right_pos += 5;
                    }
                } else {
                    if buf[len - 1] == 1 {
                        state.left_pos -= 5;
                    } else if buf[len - 1] == 2 {
                        state.left_pos += 5;
                    }
                }
            }
            movement[0] = 0;
        }
    }
}
