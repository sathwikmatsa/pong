use piston_window::*;
use pong::*;

fn set_status(status: &mut String, ip: &String) {
    status.clear();
    if ip.len() == 0 {
        status.push_str("status: waiting for ip/port input.");
    } else {
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
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let mut glyphs = window.load_font(assets.join("square.ttf")).unwrap();

    let mut peer_ip = String::new();
    let mut status = String::new();

    window.set_lazy(true);
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, device| {
            clear([0.054, 0.062, 0.098, 1.0], g);

            // red box highlight on PONG word
            rectangle([1.0, 0.0, 0.0, 1.0],[420.0, 40.0, 115.0, 100.0],
                c.transform,g);

            // welcome message
            text::Text::new_color([1.0, 1.0, 1.0, 0.9], 22)
                .draw("Welcome to PONG",&mut glyphs,
                    &c.draw_state,c.transform.trans(100.0, 100.0),g)
                .unwrap();

            // instruction text
            text::Text::new_color([1.0, 1.0, 1.0, 0.9], 8)
                .draw("type in the IP address of other player", &mut glyphs,
                    &c.draw_state, c.transform.trans(100.0, 200.0), g)
                .unwrap();

            // instruction example text
            text::Text::new_color([1.0, 1.0, 1.0, 0.9], 8)
                .draw("ip/port - for example: 192.168.2.1/8000", &mut glyphs,
                    &c.draw_state, c.transform.trans(100.0, 220.0), g)
                .unwrap();

            // user input text
            text::Text::new_color([1.0, 1.0, 1.0, 1.0], 15)
                .draw(&peer_ip, &mut glyphs,
                    &c.draw_state, c.transform.trans(100.0, 300.0), g)
                .unwrap();

            // validate user input and set status
            set_status(&mut status, &peer_ip);

            // status text
            text::Text::new_color([1.0, 1.0, 1.0, 1.0], 8)
                .draw(&status, &mut glyphs,
                    &c.draw_state, c.transform.trans(100.0, 400.0), g)
                .unwrap();

            // Update glyphs before rendering.
            glyphs.factory.encoder.flush(device);
        });

        // process key presses
        match e.press_args() {
            Some(Button::Keyboard(key)) => {
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
                        Game {
                            title: "Game loop (press X to exit game)",
                            exit_button: Button::Keyboard(Key::X),
                        }
                        .run(&mut window);
                        window.set_title(title.into());
                    }
                    _ => (),
                };
            }
            _ => (),
        };
    }
}

