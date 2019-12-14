use super::*;
use piston_window::*;
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

pub struct ClientWindow {
    title: &'static str,
    exit_button: Button,
    peer_ip: String,
    status: String,
    conn_status: ConnMode,
}

impl ClientWindow {
    pub fn new(title: &'static str, exit_button: Button) -> Self {
        Self {
            title,
            exit_button,
            peer_ip: String::new(),
            status: String::new(),
            conn_status: ConnMode::InvalidAddress,
        }
    }
    pub fn run(&mut self, window: &mut PistonWindow, glyphs: &mut Glyphs) {
        window.set_title(self.title.into());

        while let Some(e) = window.next() {
            window.draw_2d(&e, |c, g, device| {
                clear(BACKGROUND, g);

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
                        "type in the IP address of other player",
                        glyphs,
                        &c.draw_state,
                        c.transform.trans(100.0, 200.0),
                        g,
                    )
                    .unwrap();

                // instruction example text
                text::Text::new_color([1.0, 1.0, 1.0, 0.9], 8)
                    .draw(
                        "ip/port - for example: 192.168.2.1/8000",
                        glyphs,
                        &c.draw_state,
                        c.transform.trans(100.0, 220.0),
                        g,
                    )
                    .unwrap();

                // user input text
                text::Text::new_color([1.0, 1.0, 1.0, 1.0], 15)
                    .draw(
                        &self.peer_ip,
                        glyphs,
                        &c.draw_state,
                        c.transform.trans(100.0, 300.0),
                        g,
                    )
                    .unwrap();

                // status text
                text::Text::new_color([1.0, 1.0, 1.0, 1.0], 8)
                    .draw(
                        &self.status,
                        glyphs,
                        &c.draw_state,
                        c.transform.trans(100.0, 400.0),
                        g,
                    )
                    .unwrap();

                // Update glyphs before rendering.
                glyphs.factory.encoder.flush(device);
            });

            if self.conn_status == ConnMode::ReqForConnect {
                let peer_ip_port: SocketAddr = self.peer_ip.replace("/", ":").parse().unwrap();
                let conn = TcpStream::connect_timeout(&peer_ip_port, Duration::from_secs(5));
                if let Ok(stream) = conn {
                    self.conn_status = ConnMode::Connected;
                    GameWindow {
                        title: "Pong: Client (press X to exit game)",
                        exit_button: Button::Keyboard(Key::X),
                        stream,
                        player: Player::Left,
                    }
                    .run(window, glyphs);
                    window.set_title(self.title.into());
                    self.conn_status = ConnMode::ValidAddress;
                } else {
                    self.conn_status = ConnMode::FailedToConnect;
                }
            }

            let key_press = e.press_args();
            if let Some(button) = key_press {
                if button == self.exit_button {
                    break;
                }
            }
            // process key presses
            if let Some(Button::Keyboard(key)) = key_press {
                if self.conn_status == ConnMode::FailedToConnect
                    || (self.conn_status == ConnMode::Connected && key != Key::Return)
                {
                    self.conn_status = ConnMode::InvalidAddress;
                }
                match key {
                    Key::D0 | Key::NumPad0 => self.peer_ip.push('0'),
                    Key::D1 | Key::NumPad1 => self.peer_ip.push('1'),
                    Key::D2 | Key::NumPad2 => self.peer_ip.push('2'),
                    Key::D3 | Key::NumPad3 => self.peer_ip.push('3'),
                    Key::D4 | Key::NumPad4 => self.peer_ip.push('4'),
                    Key::D5 | Key::NumPad5 => self.peer_ip.push('5'),
                    Key::D6 | Key::NumPad6 => self.peer_ip.push('6'),
                    Key::D7 | Key::NumPad7 => self.peer_ip.push('7'),
                    Key::D8 | Key::NumPad8 => self.peer_ip.push('8'),
                    Key::D9 | Key::NumPad9 => self.peer_ip.push('9'),
                    Key::Period | Key::NumPadPeriod => self.peer_ip.push('.'),
                    Key::Backspace => {
                        self.peer_ip.pop();
                    }
                    Key::Slash => self.peer_ip.push('/'),
                    Key::Return => {
                        if self.conn_status == ConnMode::ValidAddress {
                            self.conn_status = ConnMode::ReqForConnect;
                        }
                    }
                    _ => (),
                };
            }
            // validate user input and set status
            set_status(&mut self.status, &self.peer_ip, &mut self.conn_status);
        }
    }
}

#[derive(PartialEq)]
enum ConnMode {
    InvalidAddress,
    ValidAddress,
    ReqForConnect,
    Connected,
    FailedToConnect,
}

fn set_status(status: &mut String, ip: &str, conn_mode: &mut ConnMode) {
    status.clear();
    if *conn_mode == ConnMode::ReqForConnect {
        status.push_str("status: connecting with peer..");
    } else if *conn_mode == ConnMode::FailedToConnect {
        status.push_str("status: failed to connect with peer.");
    } else if *conn_mode == ConnMode::Connected {
        status.push_str("status: SUCCESS! Press \"ENTER\".");
    } else if ip.is_empty() {
        status.push_str("status: waiting for ip/port input.");
        *conn_mode = ConnMode::InvalidAddress;
    } else {
        // initialized to InvalidAddress
        *conn_mode = ConnMode::InvalidAddress;
        let ip_port = ip.split('/').collect::<Vec<_>>();
        if ip_port.len() == 2 {
            let ip = ip_port[0];
            let port = ip_port[1];

            if !is_valid_ip(ip) {
                status.push_str("status: invalid ip input.");
            } else if !is_valid_port(port) {
                status.push_str("status: invalid port input.");
            } else {
                status.push_str("status: valid ip/port. press \"ENTER\".");
                *conn_mode = ConnMode::ValidAddress;
            }
            return;
        }
        status.push_str("status: invalid ip/port input.");
    }
}
