use piston_window::*;
use pong::*;
use std::net::{Shutdown, SocketAddr, TcpStream};
use std::time::Duration;

#[derive(PartialEq)]
enum ConnMode {
    InvalidAddress,
    ValidAddress,
    ReqForConnect,
    Connected,
    FailedToConnect,
}

fn set_status(status: &mut String, ip: &String, conn_mode: &mut ConnMode) {
    status.clear();
    if *conn_mode == ConnMode::ReqForConnect {
        status.push_str("status: connecting with peer..");
    } else if *conn_mode == ConnMode::FailedToConnect {
        status.push_str("status: failed to connect with peer.");
    } else if *conn_mode == ConnMode::Connected {
        status.push_str("status: SUCCESS! Press \"ENTER\".");
    } else if ip.len() == 0 {
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

fn main() {
    let title = "Pong";
    let mut window: PistonWindow = WindowSettings::new(title, [640, 480])
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let mut glyphs = window.load_font(assets.join("square.ttf")).unwrap();

    let mut peer_ip = String::new();
    let mut status = String::new();
    let mut conn_status = ConnMode::InvalidAddress;
    let mut lapse = 0;

    window.set_lazy(true);
    while let Some(e) = window.next() {
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
                    &mut glyphs,
                    &c.draw_state,
                    c.transform.trans(100.0, 100.0),
                    g,
                )
                .unwrap();

            // instruction text
            text::Text::new_color([1.0, 1.0, 1.0, 0.9], 8)
                .draw(
                    "type in the IP address of other player",
                    &mut glyphs,
                    &c.draw_state,
                    c.transform.trans(100.0, 200.0),
                    g,
                )
                .unwrap();

            // instruction example text
            text::Text::new_color([1.0, 1.0, 1.0, 0.9], 8)
                .draw(
                    "ip/port - for example: 192.168.2.1/8000",
                    &mut glyphs,
                    &c.draw_state,
                    c.transform.trans(100.0, 220.0),
                    g,
                )
                .unwrap();

            // user input text
            text::Text::new_color([1.0, 1.0, 1.0, 1.0], 15)
                .draw(
                    &peer_ip,
                    &mut glyphs,
                    &c.draw_state,
                    c.transform.trans(100.0, 300.0),
                    g,
                )
                .unwrap();

            // validate user input and set status
            set_status(&mut status, &peer_ip, &mut conn_status);

            // status text
            text::Text::new_color([1.0, 1.0, 1.0, 1.0], 8)
                .draw(
                    &status,
                    &mut glyphs,
                    &c.draw_state,
                    c.transform.trans(100.0, 400.0),
                    g,
                )
                .unwrap();

            // Update glyphs before rendering.
            glyphs.factory.encoder.flush(device);
        });

        if conn_status == ConnMode::ReqForConnect {
            // Above function `draw_2d` executes fewer times than this block. As a result,
            // the current block  modifies `conn_status` even before the screen updates status.
            // Hence, ignoring for few frames to let the screen update for better user experience.
            // [TODO] execute tcpstream connect and modify conn_status asynchronously
            lapse -= 1;
            if lapse <= 0 {
                let peer_ip_port: SocketAddr = peer_ip.replace("/", ":").parse().unwrap();
                let conn = TcpStream::connect_timeout(&peer_ip_port, Duration::from_secs(5));
                if conn.is_ok() {
                    let stream = conn.unwrap();
                    // shutdown in main window, pass ip address to Game struct and reconnect
                    // in game window.
                    //[TODO] move this stream directly to Game struct
                    stream
                        .shutdown(Shutdown::Both)
                        .expect("shutdown call failed");
                    conn_status = ConnMode::Connected;
                } else {
                    conn_status = ConnMode::FailedToConnect;
                }
            }
        }

        // process key presses
        match e.press_args() {
            Some(Button::Keyboard(key)) => {
                if conn_status == ConnMode::FailedToConnect
                    || (conn_status == ConnMode::Connected && key != Key::Return)
                {
                    conn_status = ConnMode::InvalidAddress;
                }
                match key {
                    Key::D0 | Key::NumPad0 => peer_ip.push('0'),
                    Key::D1 | Key::NumPad1 => peer_ip.push('1'),
                    Key::D2 | Key::NumPad2 => peer_ip.push('2'),
                    Key::D3 | Key::NumPad3 => peer_ip.push('3'),
                    Key::D4 | Key::NumPad4 => peer_ip.push('4'),
                    Key::D5 | Key::NumPad5 => peer_ip.push('5'),
                    Key::D6 | Key::NumPad6 => peer_ip.push('6'),
                    Key::D7 | Key::NumPad7 => peer_ip.push('7'),
                    Key::D8 | Key::NumPad8 => peer_ip.push('8'),
                    Key::D9 | Key::NumPad9 => peer_ip.push('9'),
                    Key::Period | Key::NumPadPeriod => peer_ip.push('.'),
                    Key::Backspace => {
                        peer_ip.pop();
                    }
                    Key::Slash => peer_ip.push('/'),
                    Key::Return => {
                        if conn_status == ConnMode::ValidAddress {
                            conn_status = ConnMode::ReqForConnect;
                            lapse = 5;
                        } else if conn_status == ConnMode::Connected {
                            Game {
                                title: "Pong multiplayer (press X to exit game)",
                                exit_button: Button::Keyboard(Key::X),
                                stream: TcpStream::connect_timeout(
                                    &peer_ip.replace("/", ":").parse::<SocketAddr>().unwrap(),
                                    Duration::from_secs(5),
                                )
                                .unwrap(),
                            }
                            .run(&mut window);
                            window.set_title(title.into());
                        } else {
                        }
                    }
                    _ => (),
                };
            }
            _ => (),
        };
    }
}
