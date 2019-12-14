use super::*;
use std::io::{Read, Write};
use std::sync::{mpsc::Receiver, Arc, Mutex};

pub struct Syncer<T: Read + Write> {
    shared_game_state: Arc<Mutex<GameModel>>,
    conn: T,
}

impl<T> Syncer<T>
where
    T: Read + Write,
{
    pub fn new(shared_game_state: Arc<Mutex<GameModel>>, stream: T) -> Self {
        Self {
            shared_game_state,
            conn: stream,
        }
    }
    pub fn run(&mut self, ctrl_conn: Receiver<Message>) {
        loop {
            // send player moves to opponent
            if let Ok(ctrl_message) = ctrl_conn.try_recv() {
                match ctrl_message {
                    Message::PadCollide => {
                        let state = self.shared_game_state.lock().unwrap();
                        let ball_data = (*state).export_ball();
                        self.conn.write_all(&[ctrl_message as u8]).unwrap();
                        self.conn.write_all(&ball_data).unwrap();
                    }
                    _ => self.conn.write_all(&[ctrl_message as u8]).unwrap(),
                }
            }

            // read opponent's movement
            let mut buf = [0; 1];
            if self.conn.read_exact(&mut buf).is_ok() {
                let mut state = self.shared_game_state.lock().unwrap();
                // update opponent movement
                match Message::from_u8(buf[0]) {
                    Message::MoveUp => (*state).move_opponent_up(),
                    Message::MoveDown => (*state).move_opponent_down(),
                    Message::PadCollide => {
                        let mut ball_data = [0; 24];
                        while self.conn.read_exact(&mut ball_data).is_err() {}
                        (*state).reset_ball(ball_data);
                    }
                    _ => (),
                };
            }
        }
    }
}
