#[derive(Default, Clone, Copy)]
pub struct GameSettings {
    pub window_width: u32,
    pub window_height: u32,
    pub paddle_width: u32,
    pub paddle_height: u32,
    pub paddle_round_radius: f64,
    pub paddle_margin: u32,
    pub paddle_step: u32,
    pub ball_radius: f64,
    pub ball_speed: u32,
    pub ms_per_update: u128,
    pub max_bounce_angle: f64,
    pub bg_color: [f32; 4],
    pub paddle_color: [f32; 4],
    pub ball_color: [f32; 4],
    pub score_color: [f32; 4],
    pub score_font_size: u32,
    pub left_score_xy: [f64; 2],
    pub right_score_xy: [f64; 2],
}
