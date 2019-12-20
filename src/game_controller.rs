use super::*;
use piston_window::*;
use std::sync::{mpsc::Sender, Arc, RwLock};
use std::time::Instant;

#[repr(u8)]
pub enum Message {
    MoveUp = 1,
    MoveDown = 2,
    BallHit = 3,
    Invalid = 4,
}

impl Message {
    pub fn from_u8(i: u8) -> Self {
        match i {
            1 => Message::MoveUp,
            2 => Message::MoveDown,
            3 => Message::BallHit,
            _ => Message::Invalid,
        }
    }
}

pub struct GameController {
    pub state: Arc<RwLock<GameModel>>,
    lag: u128,
    timer: Instant,
}

impl GameController {
    pub fn new(shared_game_state: Arc<RwLock<GameModel>>) -> Self {
        Self {
            state: shared_game_state,
            lag: 0,
            timer: Instant::now(),
        }
    }
    pub fn handle_event<E: GenericEvent>(&mut self, e: &E, syncer_conn: &Sender<Message>) {
        // process key presses
        let mut state = self.state.write().unwrap();
        if let Some(button) = e.press_args() {
            if button == Button::Keyboard(Key::Up) {
                (*state).move_up();
                syncer_conn.send(Message::MoveUp).unwrap();
            } else if button == Button::Keyboard(Key::Down) {
                (*state).move_down();
                syncer_conn.send(Message::MoveDown).unwrap();
            }
        }

        //https://gameprogrammingpatterns.com/game-loop.html#play-catch-up
        self.lag += self.timer.elapsed().as_millis();
        self.timer = Instant::now();
        let ms_per_update = (*state).config.ms_per_update;
        while self.lag >= ms_per_update {
            let ball_hit = (*state).update_ball();
            if ball_hit {
                syncer_conn.send(Message::BallHit).unwrap();
            }
            self.lag -= ms_per_update;
        }
    }
}
