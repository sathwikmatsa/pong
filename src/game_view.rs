use super::*;
use piston_window::ellipse::circle;
use piston_window::*;

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
            clear(BACKGROUND, g);
            let left_paddle = Rectangle::new_round(color::WHITE, 0.5);
            let right_paddle = Rectangle::new_round(color::WHITE, 0.5);
            let ball = Ellipse::new(color::WHITE);

            left_paddle.draw(
                [0.0, 0.0, PAD_DIM[0] as f64, PAD_DIM[1] as f64],
                &c.draw_state,
                c.transform.trans(0.0, controller.state.left_pos as f64),
                g,
            );

            right_paddle.draw(
                [0.0, 0.0, PAD_DIM[0] as f64, PAD_DIM[1] as f64],
                &c.draw_state,
                c.transform.trans(
                    (WIN_DIM[0] - PAD_DIM[0]) as f64,
                    controller.state.right_pos as f64,
                ),
                g,
            );

            ball.draw(
                circle(
                    controller.state.ball_centre[0] as f64,
                    controller.state.ball_centre[1] as f64,
                    BALL_RADIUS,
                ),
                &c.draw_state,
                c.transform,
                g,
            );

            // left paddle score
            text::Text::new_color([1.0, 1.0, 1.0, 0.5], 12)
                .draw(
                    &controller.state.score[0].to_string(),
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
                    &controller.state.score[1].to_string(),
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
