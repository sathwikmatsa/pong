use super::*;
use piston_window::*;
use std::net::TcpListener;

const PORT: &str = "9731";

pub struct ServerOption {
    title: &'static str,
    exit_button: Button,
    ip_addr: String,
    listener: TcpListener,
}

impl ServerOption {
    pub fn new(title: &'static str, exit_button: Button) -> Self {
        Self {
            title: title.into(),
            exit_button,
            ip_addr: String::new(),
            listener: TcpListener::bind("127.0.0.1:".to_owned() + PORT)
                .expect("TcpListener error."),
        }
    }
    pub fn run(&mut self, window: &mut PistonWindow, glyphs: &mut Glyphs) {
        window.set_title(self.title.into());
        let mut wait = LAPSE;

        while let Some(e) = window.next() {
            set_local_ip_address(&mut self.ip_addr);
            window.draw_2d(&e, |c, g, device| {
                clear([0.054, 0.062, 0.098, 1.0], g);

                // red box highlight on PONG word
                rectangle(
                    [1.0, 0.0, 0.0, 1.0],
                    [420.0, 40.0, 115.0, 100.0],
                    c.transform,
                    g,
                );

                // welcome message
                text::Text::new_color([1.0, 1.0, 1.0, 0.9], 22)
                    .draw(
                        "Welcome to PONG",
                        glyphs,
                        &c.draw_state,
                        c.transform.trans(100.0, 100.0),
                        g,
                    )
                    .unwrap();

                // instruction text
                text::Text::new_color([1.0, 1.0, 1.0, 0.9], 8)
                    .draw(
                        "Inform opponent to connect to this addr",
                        glyphs,
                        &c.draw_state,
                        c.transform.trans(100.0, 200.0),
                        g,
                    )
                    .unwrap();

                // info text
                text::Text::new_color([1.0, 1.0, 1.0, 0.9], 8)
                    .draw(
                        "-listening on the following ip/port-",
                        glyphs,
                        &c.draw_state,
                        c.transform.trans(100.0, 220.0),
                        g,
                    )
                    .unwrap();

                // local ip addr
                text::Text::new_color([1.0, 1.0, 1.0, 1.0], 15)
                    .draw(
                        format!("{}/{}", self.ip_addr, PORT).as_str(),
                        glyphs,
                        &c.draw_state,
                        c.transform.trans(100.0, 300.0),
                        g,
                    )
                    .unwrap();

                // status text
                text::Text::new_color([1.0, 1.0, 1.0, 1.0], 8)
                    .draw(
                        "status: waiting for peer..",
                        glyphs,
                        &c.draw_state,
                        c.transform.trans(100.0, 400.0),
                        g,
                    )
                    .unwrap();

                // Update glyphs before rendering.
                glyphs.factory.encoder.flush(device);
            });

            let key_press = e.press_args();
            if let Some(button) = key_press {
                if button == self.exit_button {
                    break;
                }
            }

            wait -= 1;
            // listen for incoming connections
            if wait <= 0 {
                for stream in self.listener.incoming() {
                    if stream.is_ok() {
                        Game {
                            title: "Pong multiplayer (press X to exit game)",
                            exit_button: Button::Keyboard(Key::X),
                            stream: stream.unwrap(),
                        }
                        .run(window);
                        window.set_title(self.title.into());
                        wait = LAPSE;
                        break;
                    }
                }
            }
        }
    }
}

fn set_local_ip_address(ip: &mut String) {
    use get_if_addrs::*;

    ip.clear();
    for iface in get_if_addrs().unwrap() {
        if iface.ip().is_ipv4() {
            let ipaddr = iface.ip().to_string();
            if ipaddr != "127.0.0.1" {
                ip.push_str(&ipaddr);
                return;
            }
        }
    }
    ip.push_str("127.0.0.1");
}
