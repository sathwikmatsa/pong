use super::*;
use piston_window::ellipse::circle;
use piston_window::*;

pub struct GameView {
    config: GameSettings,
}

impl GameView {
    pub fn new(config: GameSettings) -> Self {
        Self { config }
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

            clear(self.config.bg_color, g);
            let left_paddle =
                Rectangle::new_round(self.config.paddle_color, self.config.paddle_round_radius);
            let right_paddle =
                Rectangle::new_round(self.config.paddle_color, self.config.paddle_round_radius);
            let ball = Ellipse::new(self.config.ball_color);

            left_paddle.draw(
                [
                    0.,
                    0.,
                    self.config.paddle_width as f64,
                    self.config.paddle_height as f64,
                ],
                &c.draw_state,
                c.transform
                    .trans(self.config.paddle_margin as f64, state.left_pad_top as f64),
                g,
            );

            right_paddle.draw(
                [
                    0.,
                    0.,
                    self.config.paddle_width as f64,
                    self.config.paddle_height as f64,
                ],
                &c.draw_state,
                c.transform.trans(
                    (self.config.window_width
                        - self.config.paddle_width
                        - self.config.paddle_margin) as f64,
                    state.right_pad_top as f64,
                ),
                g,
            );

            ball.draw(
                circle(
                    state.ball.centre_x_f64(),
                    state.ball.centre_y_f64(),
                    self.config.ball_radius,
                ),
                &c.draw_state,
                c.transform,
                g,
            );

            // left player score
            text::Text::new_color(self.config.score_color, self.config.score_font_size)
                .draw(
                    &state.score_board[0].to_string(),
                    glyphs,
                    &c.draw_state,
                    c.transform
                        .trans(self.config.left_score_xy[0], self.config.left_score_xy[1]),
                    g,
                )
                .unwrap();

            // right player score
            text::Text::new_color(self.config.score_color, self.config.score_font_size)
                .draw(
                    &state.score_board[1].to_string(),
                    glyphs,
                    &c.draw_state,
                    c.transform
                        .trans(self.config.right_score_xy[0], self.config.right_score_xy[1]),
                    g,
                )
                .unwrap();

            // Update glyphs before rendering.
            glyphs.factory.encoder.flush(d);
        });
    }
}
