use super::*;
use piston_window::ellipse::circle;
use piston_window::*;
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct GameView {}

impl GameView {
    pub fn new() -> Self {
        Self {}
    }
    pub fn render(
        &self,
        controller: &GameController,
        screen: &mut PistonWindow,
        e: &Event,
        glyphs: &mut Glyphs,
    ) {
        screen.draw_2d(e, |c, g, d| {
            let state = controller.state.game_state_copy();

            clear(BACKGROUND, g);
            let left_paddle = Rectangle::new_round(color::WHITE, 0.5);
            let right_paddle = Rectangle::new_round(color::WHITE, 0.5);
            let ball = Ellipse::new(color::WHITE);

            left_paddle.draw(
                [0.0, 0.0, PAD_DIM[0] as f64, PAD_DIM[1] as f64],
                &c.draw_state,
                c.transform.trans(0.0, state.left_pos as f64),
                g,
            );

            right_paddle.draw(
                [0.0, 0.0, PAD_DIM[0] as f64, PAD_DIM[1] as f64],
                &c.draw_state,
                c.transform
                    .trans((WIN_DIM[0] - PAD_DIM[0]) as f64, state.right_pos as f64),
                g,
            );

            ball.draw(
                circle(
                    state.ball_centre[0] as f64,
                    state.ball_centre[1] as f64,
                    BALL_RADIUS,
                ),
                &c.draw_state,
                c.transform,
                g,
            );

            // left paddle score
            text::Text::new_color([1.0, 1.0, 1.0, 0.5], 12)
                .draw(
                    &state.score[0].to_string(),
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
                    &state.score[1].to_string(),
                    glyphs,
                    &c.draw_state,
                    c.transform.trans(
                        (WIN_DIM[0] - 3 * PAD_DIM[0]) as f64,
                        WIN_DIM[1] as f64 - 10.0,
                    ),
                    g,
                )
                .unwrap();

            // Update glyphs before rendering.
            glyphs.factory.encoder.flush(d);
        });
    }
}

pub trait SharedGameModel {
    fn game_state_copy(&self) -> GameModel;
}

impl SharedGameModel for Arc<Mutex<GameModel>> {
    fn game_state_copy(&self) -> GameModel {
        let guard = self.lock().unwrap();
        (*guard).clone()
    }
}
