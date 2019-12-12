use super::*;
use piston_window::*;
use std::net::TcpStream;
use std::io::*;

#[repr(u8)]
enum Movement {
    Up = 1,
    Down = 2,
    NoOp = 0,
}

pub struct GameController {
    pub state: GameModel
}

impl GameController {
    pub fn new(game_state: GameModel) -> Self {
        Self {
            state: game_state,
        }
    }
    pub fn event<E: GenericEvent>(&mut self, e: &E, conn: &mut TcpStream) {
        let mut movement : u8 = Movement::NoOp as u8;
        // process key presses
        if let Some(button) = e.press_args() {
            if button == Button::Keyboard(Key::Up) {
                self.state.move_up();
                movement = Movement::Up as u8;
            } else if button == Button::Keyboard(Key::Down) {
                self.state.move_down();
                movement = Movement::Down as u8;
            }
        }

        // send movement to opponent
        conn.write(&[movement]).unwrap();

        // read opponent's movements
        let mut buf = [0; 10];
        if conn.peek(&mut buf).expect("peek failed") != 0 {
            let len = conn.read(&mut buf).expect("read failed");
            // update opponent movements
            for i in 0..len {
                if buf[i] == Movement::Up as u8 {
                    self.state.move_opponent_up();
                } else if buf[i] == Movement::Down as u8 {
                    self.state.move_opponent_down();
                }
            }
        }
    }
}
