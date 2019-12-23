pub const DEFAULT_CONFIG: GameSettings = GameSettings {
    window_width: 640,
    window_height: 480,
    paddle_width: 10,
    paddle_height: 80,
    paddle_round_radius: 0.1,
    paddle_margin: 5,
    paddle_step: 14,
    ball_radius: 5.,
    ball_speed: 11,
    ms_per_update: 25,
    max_bounce_angle: std::f64::consts::PI * 1. / 3.,
    bg_color: [0.054, 0.062, 0.098, 1.0],
    paddle_color: [1.; 4],
    ball_color: [1.; 4],
    score_color: [1.; 4],
    score_font_size: 12,
    left_score_xy: [150., 20.],
    right_score_xy: [470., 20.],
};

pub mod client_window;
pub use client_window::*;

pub mod server_window;
pub use server_window::*;

pub mod game_window;
pub use game_window::*;

pub mod game_model;
pub use game_model::*;

pub mod game_view;
pub use game_view::*;

pub mod game_controller;
pub use game_controller::*;

pub mod game_settings;
pub use game_settings::*;

pub mod syncer;
pub use syncer::*;

pub mod ball;
pub use ball::*;

pub mod sfx;
pub use sfx::*;

pub fn is_valid_port(port: &str) -> bool {
    if let Ok(x) = port.parse::<u32>() {
        // max port value
        if x <= 65532 {
            return true;
        }
    }
    false
}

pub fn is_valid_ip(ip: &str) -> bool {
    let octects_validity = ip
        .split('.')
        .map(|octect| octect.parse::<u8>().is_ok())
        .collect::<Vec<_>>();
    if octects_validity.len() == 4 && octects_validity.iter().all(|x| x == &true) {
        return true;
    }
    false
}
