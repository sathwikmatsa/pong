pub mod game;
pub use game::*;

pub mod client_option;
pub use client_option::*;

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
