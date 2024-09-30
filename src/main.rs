extern crate sfml;

use sfml::graphics::*;
use sfml::system::Vector2i;
use sfml::system::*;
use sfml::window;
use sfml::window::*;

fn vec2i_to_2f(vec2i: Vector2i)-> Vector2f {
    Vector2f::new(vec2i.x as f32,vec2i.y as f32)
}

fn main() {
    let mut window = RenderWindow::new(
        (800, 600),
        "Hello",
        Style::default(),
        &ContextSettings::default(),
    );

    window.set_framerate_limit(60);
    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => window.close(),
                Event::TextEntered { unicode } => println!("{}", unicode),
                _ => (),
            }
        }

        window.set_active(true);
        window.clear(Color::BLACK);

        if Key::is_pressed(Key::Left) {
            println!("going left");
        }
        let mouse_pos = RenderWindow::mouse_position(&window);
        println!("{:?}", mouse_pos);
        let font = Font::from_file("fonts/open-sans/OpenSans-Regular.ttf").unwrap();

        let text_string = "Hello";
        let render_state = RenderStates::default();
        let mut text = Text::new(text_string, &font, text_string.len() as u32);
        text.set_character_size(30);
        let mouse_pos_f = vec2i_to_2f(mouse_pos);
        text.set_position(mouse_pos_f);

        window.draw_text(&text, &render_state);
        window.display();
    }
}
