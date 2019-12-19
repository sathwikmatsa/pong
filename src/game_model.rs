use super::*;
use std::cmp::min;
use std::sync::{Arc, Mutex};

#[derive(PartialEq, Copy, Clone)]
pub enum Pad {
    Left,
    Right,
}

pub struct GameStateInstance {
    pub left_pad_top: f64,
    pub right_pad_top: f64,
    pub ball_centre_x: f64,
    pub ball_centre_y: f64,
    pub left_player_score: String,
    pub right_player_score: String,
}

pub struct GameModel {
    pub ball: Ball,
    pub left_pad_top: u32,
    pub right_pad_top: u32,
    pub score_board: [u8; 2],
    pub player_pad: Pad,
    pub config: GameSettings,
    sfx: Sfx,
}

impl GameModel {
    pub fn new(player_pad: Pad, config: GameSettings) -> Self {
        Self {
            ball: Ball::new(
                [
                    config.window_width as i32 / 2,
                    config.window_height as i32 / 2,
                ],
                [config.ball_speed as f32, 0.],
                config.ball_radius,
            ),
            left_pad_top: (config.window_height - config.paddle_height) / 2,
            right_pad_top: (config.window_height - config.paddle_height) / 2,
            score_board: [0, 0],
            player_pad,
            config,
            sfx: Sfx::new(),
        }
    }
    pub fn move_up(&mut self) {
        if self.player_pad == Pad::Left {
            self.left_pad_top = self.left_pad_top.saturating_sub(self.config.paddle_step);
        } else {
            self.right_pad_top = self.right_pad_top.saturating_sub(self.config.paddle_step);
        }
    }
    pub fn move_down(&mut self) {
        if self.player_pad == Pad::Left {
            self.left_pad_top += self.config.paddle_step;
            // restricting paddle movement below bottom wall
            self.left_pad_top = min(
                self.left_pad_top,
                self.config.window_height - self.config.paddle_height,
            );
        } else {
            self.right_pad_top += self.config.paddle_step;
            self.right_pad_top = min(
                self.right_pad_top,
                self.config.window_height - self.config.paddle_height,
            );
        }
    }
    pub fn move_opponent_up(&mut self) {
        if self.player_pad != Pad::Left {
            self.left_pad_top = self.left_pad_top.saturating_sub(self.config.paddle_step);
        } else {
            self.right_pad_top = self.right_pad_top.saturating_sub(self.config.paddle_step);
        }
    }
    pub fn move_opponent_down(&mut self) {
        if self.player_pad != Pad::Left {
            self.left_pad_top += self.config.paddle_step;
            // restricting paddle movement below bottom wall
            self.left_pad_top = min(
                self.left_pad_top,
                self.config.window_height - self.config.paddle_height,
            );
        } else {
            self.right_pad_top += self.config.paddle_step;
            self.right_pad_top = min(
                self.right_pad_top,
                self.config.window_height - self.config.paddle_height,
            );
        }
    }
    fn get_bounce_angle(&self, pad: Pad) -> f64 {
        // https://gamedev.stackexchange.com/a/4255
        let relative_intersect_y = if pad == Pad::Left {
            (self.left_pad_top + self.config.paddle_height / 2) as i32 - self.ball.centre_y()
        } else {
            (self.right_pad_top + self.config.paddle_height / 2) as i32 - self.ball.centre_y()
        };
        let normalized_riy: f64 =
            relative_intersect_y as f64 / (self.config.paddle_height as f64 / 2.);

        normalized_riy * self.config.max_bounce_angle
    }
    fn respawn_ball_from(&mut self, pad: Pad) {
        if pad == Pad::Left {
            self.ball.centre = [
                (2 * self.config.paddle_width + self.config.paddle_margin) as i32,
                self.config.window_height as i32 / 2,
            ];
            self.ball.velocity = [self.config.ball_speed as f32, 0.].into();
        } else {
            self.ball.centre = [
                (self.config.window_width
                    - 2 * self.config.paddle_width
                    - self.config.paddle_margin) as i32,
                self.config.window_height as i32 / 2,
            ];
            self.ball.velocity = [-(self.config.ball_speed as f32), 0.].into();
        }
    }
    pub fn update_ball(&mut self) -> bool {
        let left_paddle_topx = self.config.paddle_margin + self.config.paddle_width;
        let right_paddle_topx =
            self.config.window_width - self.config.paddle_margin - self.config.paddle_width;
        let ball_speed = self.config.ball_speed;
        let mut ball_hit = false;

        // collision logic
        // left paddle
        if self.ball.collides_left_vseg(
            self.left_pad_top,
            self.config.paddle_height,
            left_paddle_topx,
        ) {
            self.ball
                .reflect_from_left(self.get_bounce_angle(Pad::Left), ball_speed);
            self.sfx.play(Sound::PaddleHit);
            if self.player_pad == Pad::Left {
                ball_hit = true;
            }
        // right paddle
        } else if self.ball.collides_right_vseg(
            self.right_pad_top,
            self.config.paddle_height,
            right_paddle_topx,
        ) {
            self.ball
                .reflect_from_right(self.get_bounce_angle(Pad::Right), ball_speed);
            self.sfx.play(Sound::PaddleHit);
            if self.player_pad == Pad::Right {
                ball_hit = true;
            }
        // right wall
        } else if self.ball.centre_x() + self.config.ball_radius as i32
            >= self.config.window_width as i32
        {
            self.score_board[0] += 1;
            self.respawn_ball_from(Pad::Right);
            self.sfx.play(Sound::Goal);
            return self.player_pad == Pad::Right;
        // left wall
        } else if self.ball.centre_x() - self.config.ball_radius as i32 <= 0 {
            self.score_board[1] += 1;
            self.respawn_ball_from(Pad::Left);
            self.sfx.play(Sound::Goal);
            return self.player_pad == Pad::Left;
        // bottom wall
        } else if self.ball.centre_y() + self.config.ball_radius as i32
            >= self.config.window_height as i32
        {
            self.sfx.play(Sound::WallHit);
            self.ball.reflect_with_normal([0., -1.]);
        // top wall
        } else if self.ball.centre_y() - self.config.ball_radius as i32 <= 0 {
            self.sfx.play(Sound::WallHit);
            self.ball.reflect_with_normal([0., 1.]);
        }

        self.ball.centre[0] += self.ball.velocity.i as i32;
        self.ball.centre[1] += self.ball.velocity.j as i32;
        ball_hit
    }
    pub fn export_ball_opp_score(&self) -> (Vec<u8>, u8) {
        let ball_data = self.ball.export();
        let player_score = if self.player_pad == Pad::Left {
            self.score_board[1]
        } else {
            self.score_board[0]
        };
        (ball_data, player_score)
    }
    pub fn reset_ball_score(&mut self, serialized: [u8; 16], score: u8) {
        self.ball.reset(serialized);
        if self.player_pad == Pad::Left {
            self.score_board[0] = score;
        } else {
            self.score_board[1] = score;
        }
    }
}

pub trait SharedGameModel {
    fn capture_game_state(&self) -> GameStateInstance;
}

impl SharedGameModel for Arc<Mutex<GameModel>> {
    fn capture_game_state(&self) -> GameStateInstance {
        let state = self.lock().unwrap();
        GameStateInstance {
            left_pad_top: (*state).left_pad_top as f64,
            right_pad_top: (*state).right_pad_top as f64,
            ball_centre_x: (*state).ball.centre_x_f64(),
            ball_centre_y: (*state).ball.centre_y_f64(),
            left_player_score: (*state).score_board[0].to_string(),
            right_player_score: (*state).score_board[1].to_string(),
        }
    }
}
