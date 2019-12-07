use piston_window::*;
use pong::*;

fn main() {
    let title = "Pong: Welcome";
    let mut window: PistonWindow = WindowSettings::new(title, [640, 480])
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let mut glyphs = window.load_font(assets.join("square.ttf")).unwrap();

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

            // press S to host
            text::Text::new_color([1.0, 1.0, 1.0, 0.9], 15)
                .draw(
                    "press S to act as server",
                    &mut glyphs,
                    &c.draw_state,
                    c.transform.trans(95.0, 250.0),
                    g,
                )
                .unwrap();

            // press C to connect
            text::Text::new_color([1.0, 1.0, 1.0, 0.9], 15)
                .draw(
                    "press C to act as client",
                    &mut glyphs,
                    &c.draw_state,
                    c.transform.trans(95.0, 300.0),
                    g,
                )
                .unwrap();
            
            // info url
            text::Text::new_color([0.0, 1.0, 0.0, 0.9], 12)
                .draw(
                    "github.com/sathwikmatsa/pong",
                    &mut glyphs,
                    &c.draw_state,
                    c.transform.trans(100.0, 400.0),
                    g,
                )
                .unwrap();


            // Update glyphs before rendering.
            glyphs.factory.encoder.flush(device);
        });

        // process key presses
        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::C => {
                    ClientOption::new(
                        "Pong: Enter Server IP/PORT (press X to go back)",
                        Button::Keyboard(Key::X),
                    )
                    .run(&mut window, &mut glyphs);
                    window.set_title(title.into());
                }
                _ => (),
            };
        }
    }
}
