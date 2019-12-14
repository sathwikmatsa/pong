use super::*;
use std::cmp::min;

pub const MAXBOUNCEANGLE: f64 = std::f64::consts::PI * 5. / 12.; // 75 degrees

#[derive(PartialEq, Copy, Clone)]
pub enum Player {
    Left,
    Right,
}

#[derive(Clone)]
pub struct GameModel {
    pub ball_centre: [i32; 2],
    pub ball_velocity: [f64; 2],
    pub left_pos: u32,
    pub right_pos: u32,
    pub score: [u8; 2],
    pub player: Player,
}

impl GameModel {
    pub fn new(player: Player) -> Self {
        Self {
            ball_centre: [WIN_DIM[0] as i32 / 2, WIN_DIM[1] as i32 / 2],
            ball_velocity: [400., 0.],
            left_pos: WIN_DIM[1] / 2 - PAD_DIM[1] / 2,
            right_pos: WIN_DIM[1] / 2 - PAD_DIM[1] / 2,
            score: [0, 0],
            player,
        }
    }
    pub fn move_up(&mut self) {
        if self.player == Player::Left {
            self.left_pos = self.left_pos.saturating_sub(PADDLE_STEP);
        } else {
            self.right_pos = self.right_pos.saturating_sub(PADDLE_STEP);
        }
    }

    pub fn move_down(&mut self) {
        if self.player == Player::Left {
            self.left_pos += PADDLE_STEP;
            self.left_pos = min(self.left_pos, WIN_DIM[1] - PAD_DIM[1])
        } else {
            self.right_pos += PADDLE_STEP;
            self.right_pos = min(self.right_pos, WIN_DIM[1] - PAD_DIM[1])
        }
    }
    pub fn move_opponent_up(&mut self) {
        if self.player != Player::Left {
            self.left_pos = self.left_pos.saturating_sub(PADDLE_STEP);
        } else {
            self.right_pos = self.right_pos.saturating_sub(PADDLE_STEP);
        }
    }

    pub fn move_opponent_down(&mut self) {
        if self.player != Player::Left {
            self.left_pos += PADDLE_STEP;
            self.left_pos = min(self.left_pos, WIN_DIM[1] - PAD_DIM[1])
        } else {
            self.right_pos += PADDLE_STEP;
            self.right_pos = min(self.right_pos, WIN_DIM[1] - PAD_DIM[1])
        }
    }

    pub fn update_ball(&mut self, dt: f64) {
        let ball_vector = Vector::from(self.ball_velocity);
        let ball_speed = ball_vector.magnitude();
        // collision logic
        // left paddle
        if self.ball_centre[0] - BALL_RADIUS as i32 <= PAD_DIM[0] as i32
            && self.ball_centre[1] >= self.left_pos as i32
            && self.ball_centre[1] <= (self.left_pos + PAD_DIM[1]) as i32
        {
            let rel_intr_y = (self.left_pos + PAD_DIM[1] / 2) as i32 - self.ball_centre[1];
            let norm_rel_intr_y: f64 = rel_intr_y as f64 / (PAD_DIM[1] as f64 / 2.);
            let bounce_angle = norm_rel_intr_y * MAXBOUNCEANGLE;
            self.ball_velocity = [
                ball_speed * bounce_angle.cos(),
                ball_speed * -bounce_angle.sin(),
            ];
        // right paddle
        } else if self.ball_centre[0] + BALL_RADIUS as i32 >= (WIN_DIM[0] - PAD_DIM[0]) as i32
            && self.ball_centre[1] >= self.right_pos as i32
            && self.ball_centre[1] <= (self.right_pos + PAD_DIM[1]) as i32
        {
            let rel_intr_y = (self.right_pos + PAD_DIM[1] / 2) as i32 - self.ball_centre[1];
            let norm_rel_intr_y: f64 = rel_intr_y as f64 / (PAD_DIM[1] as f64 / 2.);
            let bounce_angle = norm_rel_intr_y * MAXBOUNCEANGLE;
            self.ball_velocity = [
                -ball_speed * bounce_angle.cos(),
                ball_speed * -bounce_angle.sin(),
            ];
        // right wall
        } else if self.ball_centre[0] + BALL_RADIUS as i32 >= WIN_DIM[0] as i32 {
            self.score[0] += 1;
            self.ball_centre = [(WIN_DIM[0] - 2 * PAD_DIM[0]) as i32, WIN_DIM[1] as i32 / 2];
            self.ball_velocity = [-400.0, 0.];
            return;
        // left wall
        } else if self.ball_centre[0] - BALL_RADIUS as i32 <= 0 {
            self.score[1] += 1;
            self.ball_centre = [2 * PAD_DIM[0] as i32, WIN_DIM[1] as i32 / 2];
            self.ball_velocity = [400.0, 0.];
            return;
        // bottom wall
        } else if self.ball_centre[1] + BALL_RADIUS as i32 >= WIN_DIM[1] as i32 {
            let reflected = ball_vector.reflect([0., -1.].into());
            self.ball_velocity = [reflected.x, reflected.y];
        // top wall
        } else if self.ball_centre[1] - BALL_RADIUS as i32 <= 0 {
            let reflected = ball_vector.reflect([0., 1.].into());
            self.ball_velocity = [reflected.x, reflected.y];
        }
        self.ball_centre[0] += (dt * self.ball_velocity[0]) as i32;
        self.ball_centre[1] += (dt * self.ball_velocity[1]) as i32;
    }
}
