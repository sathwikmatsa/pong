pub const LAPSE: u32 = 100;
pub const BACKGROUND: [f32; 4] = [0.054, 0.062, 0.098, 1.0];
pub const WIN_DIM: [u32; 2] = [640, 480];
pub const PAD_DIM: [u32; 2] = [10, 80];
pub const BALL_RADIUS: f64 = 10.0;

pub mod game;
pub use game::*;

pub mod client_option;
pub use client_option::*;

pub mod server_option;
pub use server_option::*;

pub mod game_state;
pub use game_state::*;

pub fn is_valid_port(port: &str) -> bool {
    if let Ok(x) = port.parse::<u32>() {
        // max port value
        if x <= 65532 {
            return true;
        }
    }
    return false;
}

pub fn is_valid_ip(ip: &str) -> bool {
    let octects_validity = ip
        .split('.')
        .map(|octect| {
            if let Ok(_) = octect.parse::<u8>() {
                true
            } else {
                false
            }
        })
        .collect::<Vec<_>>();
    if octects_validity.len() == 4 && octects_validity.iter().all(|x| x == &true) {
        return true;
    }
    return false;
}
