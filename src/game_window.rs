use super::*;
use piston_window::*;
use std::net::{Shutdown, TcpStream};

pub struct GameWindow {
    pub title: &'static str,
    pub exit_button: Button,
    pub stream: TcpStream,
    pub player: Player,
}

impl GameWindow {
    pub fn run(&mut self, window: &mut PistonWindow, glyphs: &mut Glyphs) {
        self.stream.set_nodelay(true).expect("set nodelay failed");
        self.stream
            .set_nonblocking(false)
            .expect("set blocking mode failed");
        window.set_title(self.title.into());

        let model = GameModel::new(self.player);
        let view = GameView::new();
        let mut controller = GameController::new(model);

        while let Some(e) = window.next() {
            view.render(&controller, window, &e, glyphs);
            controller.event(&e, &mut self.stream);
            if Some(self.exit_button) == e.press_args() {
                self.stream
                    .shutdown(Shutdown::Both)
                    .expect("shutdown failed");
                break;
            }
        }
    }
}
