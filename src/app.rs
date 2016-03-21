use piston_window::*;

use deck::Deck;
use deck_interface::DeckInterface;

pub struct App {
    deck: Deck,
    deck_interface: DeckInterface,
}

impl App {
    pub fn new(deck_size: u8) -> App {
        let deck = Deck::new(deck_size, None);
        let deck_interface = DeckInterface::new();

        App {
            deck: deck,
            deck_interface: deck_interface,
        }
    }

    pub fn run(&mut self, window: PistonWindow) {
        self.deck.display_cards();

        for e in window {
            if self.deck.is_won() {
                break;
            }

            if let Some(mouse_position) = e.mouse_cursor_args() {
                self.deck_interface.hover(mouse_position);
            }

            if let Some(button) = e.press_args() {
                self.deck_interface.apply_button(&mut self.deck, button);
                self.deck.display_cards();
            }

            e.draw_2d(|c, g| {
                clear([1.0; 4], g);
                self.deck_interface.render(c, g);
            });
        }

        println!("Well done!");
    }
}
