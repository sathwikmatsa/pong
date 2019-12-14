use super::*;
use piston_window::*;
use std::net::TcpStream;
use std::sync::{mpsc::channel, Arc, Mutex};
use std::thread;

pub struct GameWindow {
    pub title: &'static str,
    pub exit_button: Button,
    pub stream: TcpStream,
    pub player: Player,
}

impl GameWindow {
    pub fn run(self, window: &mut PistonWindow, glyphs: &mut Glyphs) {
        self.stream.set_nodelay(true).expect("set nodelay failed");
        self.stream
            .set_nonblocking(true)
            .expect("non block mode failed");
        window.set_title(self.title.into());

        let model = GameModel::new(self.player);
        let shared_model = Arc::new(Mutex::new(model));
        let shared_model_clone = shared_model.clone();
        let exit_button = self.exit_button;

        let (sender, receiver) = channel();
        thread::spawn(move || {
            Syncer::new(shared_model_clone, self.stream).run(receiver);
        });

        let view = GameView::new();
        let mut controller = GameController::new(shared_model);

        while let Some(e) = window.next() {
            view.render(&controller, window, &e, glyphs);
            controller.handle_event(&e, &sender);

            if Some(exit_button) == e.press_args() {
                break;
            }
        }
    }
}
