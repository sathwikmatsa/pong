use super::*;
use std::cmp::min;

#[derive(PartialEq, Copy, Clone)]
pub enum Player {
    Left,
    Right,
}

pub struct GameModel {
    pub ball_pos: [u32; 2],
    pub ball_velocity: [f64; 2],
    pub left_pos: u32,
    pub right_pos: u32,
    pub score: [u8; 2],
    pub player: Player, 
}

impl GameModel {
    pub fn new(player: Player) -> Self {
        Self {
            ball_pos: [WIN_DIM[0] / 2, WIN_DIM[1] / 2],
            ball_velocity: [0., 0.],
            left_pos: WIN_DIM[1] / 2 - PAD_DIM[1] / 2,
            right_pos: WIN_DIM[1] / 2 - PAD_DIM[1] / 2,
            score: [0, 0],
            player,
        }
    }
    pub fn move_up(&mut self) {
        if self.player == Player::Left {
            self.left_pos = self.left_pos.saturating_sub(10);
        } else {
            self.right_pos = self.right_pos.saturating_sub(10);
        }
    }

    pub fn move_down(&mut self) {
        if self.player == Player::Left {
            self.left_pos += 10;
            self.left_pos = min(self.left_pos, WIN_DIM[1] - PAD_DIM[1])
        } else {
            self.right_pos += 10;
            self.right_pos = min(self.right_pos, WIN_DIM[1] - PAD_DIM[1])
        }
    }
    pub fn move_opponent_up(&mut self) {
        if self.player != Player::Left {
            self.left_pos = self.left_pos.saturating_sub(10);
        } else {
            self.right_pos = self.right_pos.saturating_sub(10);
        }
    }

    pub fn move_opponent_down(&mut self) {
        if self.player != Player::Left {
            self.left_pos += 10;
            self.left_pos = min(self.left_pos, WIN_DIM[1] - PAD_DIM[1])
        } else {
            self.right_pos += 10;
            self.right_pos = min(self.right_pos, WIN_DIM[1] - PAD_DIM[1])
        }
    }
}
