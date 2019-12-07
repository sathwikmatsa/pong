use super::*;
use piston_window::ellipse::circle;
use piston_window::*;

pub struct GameState {
    ball_pos: [u32; 2],
    ball_velocity: [i8; 2],
    left_pos: u32,
    right_pos: u32,
    score: [u8; 2],
}

impl GameState {
    pub fn new() -> Self {
        Self {
            ball_pos: [WIN_DIM[0] / 2, WIN_DIM[1] / 2],
            ball_velocity: [0, 0],
            left_pos: WIN_DIM[1] / 2 - PAD_DIM[1] / 2,
            right_pos: WIN_DIM[1] / 2 - PAD_DIM[1] / 2,
            score: [0, 0],
        }
    }

    pub fn render(&self, screen: &mut PistonWindow, e: &Event, glyphs: &mut Glyphs) {
        screen.draw_2d(e, |c, g, d| {
            clear(BACKGROUND, g);
            let left_paddle = Rectangle::new_round(color::WHITE, 0.5);
            let right_paddle = Rectangle::new_round(color::WHITE, 0.5);
            let ball = Ellipse::new(color::WHITE);

            left_paddle.draw(
                [0.0, 0.0, PAD_DIM[0] as f64, PAD_DIM[1] as f64],
                &c.draw_state,
                c.transform.trans(0.0, self.left_pos as f64),
                g,
            );

            right_paddle.draw(
                [0.0, 0.0, PAD_DIM[0] as f64, PAD_DIM[1] as f64],
                &c.draw_state,
                c.transform
                    .trans((WIN_DIM[0] - PAD_DIM[0]) as f64, self.right_pos as f64),
                g,
            );

            ball.draw(
                circle(
                    self.ball_pos[0] as f64,
                    self.ball_pos[1] as f64,
                    BALL_RADIUS,
                ),
                &c.draw_state,
                c.transform,
                g,
            );

            // left paddle score
            text::Text::new_color([1.0, 1.0, 1.0, 0.5], 12)
                .draw(
                    &self.score[0].to_string(),
                    glyphs,
                    &c.draw_state,
                    c.transform
                        .trans(PAD_DIM[0] as f64, WIN_DIM[1] as f64 - 10.0),
                    g,
                )
                .unwrap();

            // right paddle score
            text::Text::new_color([1.0, 1.0, 1.0, 0.5], 12)
                .draw(
                    &self.score[1].to_string(),
                    glyphs,
                    &c.draw_state,
                    c.transform
                        .trans((WIN_DIM[0] - 3*PAD_DIM[0]) as f64, WIN_DIM[1] as f64 - 10.0),
                    g,
                )
                .unwrap();


            // Update glyphs before rendering.
            glyphs.factory.encoder.flush(d);
        });
    }
}
