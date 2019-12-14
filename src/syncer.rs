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
                self.conn.write_all(&[ctrl_message as u8]).unwrap();
            }

            // read opponent's movements
            let mut buf = [0; 10];
            if let Ok(len) = self.conn.read(&mut buf) {
                let mut state = self.shared_game_state.lock().unwrap();
                // update opponent movements
                buf.iter()
                    .take(len)
                    .for_each(|x| match Message::from_u8(*x) {
                        Message::MoveUp => (*state).move_opponent_up(),
                        Message::MoveDown => (*state).move_opponent_down(),
                        _ => (),
                    });
            }
        }
    }
}
