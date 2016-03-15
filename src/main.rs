extern crate rand;
extern crate piston_window;

use piston_window::*;

mod card;
mod card_interface;
mod deck;
mod deck_interface;

use deck::Deck;
use deck_interface::DeckInterface;

fn main() {
    let window: PistonWindow = WindowSettings::new("Hello Piston!", [960, 540])
                                   .exit_on_esc(true)
                                   .vsync(true)
                                   .build()
                                   .unwrap();

    let mut deck = Deck::new(8, None);
    let mut deck_interface = DeckInterface::new();
    deck.display_cards();

    for e in window {
        if deck.is_won() {
            break;
        }

        if let Some(mouse_position) = e.mouse_cursor_args() {
            deck_interface.hover(mouse_position);
        }

        if let Some(button) = e.press_args() {
            deck_interface.apply_button(&mut deck, button);
            deck.display_cards();
        }

        e.draw_2d(|c, g| {
            clear([1.0; 4], g);
            deck_interface.render(c, g);
        });
    }

    println!("Well done!");
}
