use super::*;
use std::convert::TryInto;
use std::io::{Read, Write};
use std::sync::{mpsc::Receiver, Arc, RwLock};

pub struct Syncer<T: Read + Write> {
    shared_game_state: Arc<RwLock<GameModel>>,
    conn: T,
}

impl<T> Syncer<T>
where
    T: Read + Write,
{
    pub fn new(shared_game_state: Arc<RwLock<GameModel>>, stream: T) -> Self {
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
                    Message::BallHit => {
                        let state = self.shared_game_state.read().unwrap();
                        let (ball_data, opp_score) = (*state).export_ball_opp_score();
                        let mut ball_data_opp_score = vec![];
                        ball_data_opp_score.extend_from_slice(&ball_data);
                        ball_data_opp_score.extend_from_slice(&[opp_score]);
                        self.conn.write_all(&[ctrl_message as u8]).unwrap();
                        self.conn.write_all(&ball_data_opp_score).unwrap();
                    }
                    _ => self.conn.write_all(&[ctrl_message as u8]).unwrap(),
                }
            }

            // read opponent's movement
            let mut buf = [0; 1];
            if self.conn.read_exact(&mut buf).is_ok() {
                // update opponent movement
                let mut state = self.shared_game_state.write().unwrap();
                match Message::from_u8(buf[0]) {
                    Message::MoveUp => (*state).move_opponent_up(),
                    Message::MoveDown => (*state).move_opponent_down(),
                    Message::BallHit => {
                        let mut data = vec![0; 17];
                        // ball data + score
                        while self.conn.read_exact(&mut data).is_err() {}
                        let score = data.pop().unwrap();
                        (*state).reset_ball_score(data.as_slice().try_into().unwrap(), score);
                    }
                    _ => (),
                };
            }
        }
    }
}
