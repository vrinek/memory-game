extern crate rand;
extern crate piston_window;
extern crate opengl_graphics;

use piston_window::*;

mod card;
mod card_interface;
mod deck;
mod deck_interface;
mod app;

use app::App;

fn main() {
    let mut app = App::new(8);

    let window: PistonWindow = WindowSettings::new("Hello Piston!", [960, 540])
                                   .exit_on_esc(true)
                                   .vsync(true)
                                   .build()
                                   .unwrap();

    app.run(window);
}
