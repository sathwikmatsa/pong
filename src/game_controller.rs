use super::*;
use piston_window::*;
use std::sync::{mpsc::Sender, Arc, Mutex};

#[repr(u8)]
pub enum Message {
    MoveUp = 1,
    MoveDown = 2,
    PadCollide = 3,
    Invalid = 4,
}

impl Message {
    pub fn from_u8(i: u8) -> Self {
        match i {
            1 => Message::MoveUp,
            2 => Message::MoveDown,
            3 => Message::PadCollide,
            _ => Message::Invalid,
        }
    }
}

pub struct GameController {
    pub state: Arc<Mutex<GameModel>>,
}

impl GameController {
    pub fn new(shared_game_state: Arc<Mutex<GameModel>>) -> Self {
        Self {
            state: shared_game_state,
        }
    }
    pub fn handle_event<E: GenericEvent>(&mut self, e: &E, syncer_conn: &Sender<Message>) {
        // process key presses
        if let Some(button) = e.press_args() {
            let mut state = self.state.lock().unwrap();
            if button == Button::Keyboard(Key::Up) {
                (*state).move_up();
                syncer_conn.send(Message::MoveUp).unwrap();
            } else if button == Button::Keyboard(Key::Down) {
                (*state).move_down();
                syncer_conn.send(Message::MoveDown).unwrap();
            }
        }

        // update ball
        if let Some(args) = e.update_args() {
            let mut state = self.state.lock().unwrap();
            let player_collides_ball = (*state).update_ball(args.dt);
            if player_collides_ball {
                syncer_conn.send(Message::PadCollide).unwrap();
            }
        }
    }
}
