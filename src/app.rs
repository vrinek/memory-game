use std::path::Path;

use piston_window::*;
use opengl_graphics::glyph_cache::GlyphCache;

use deck::Deck;
use deck_interface::DeckInterface;

pub struct App {
    deck: Deck,
    deck_interface: DeckInterface,
    glyph_cache: GlyphCache<'static>,
}

impl App {
    pub fn new(deck_size: u8) -> App {
        let deck = Deck::new(deck_size, None);
        let deck_interface = DeckInterface::new();

        let font_path = Path::new("assets/SourceCodePro-Regular.ttf");
        let mut cache = GlyphCache::new(font_path).unwrap();

        App {
            deck: deck,
            deck_interface: deck_interface,
            glyph_cache: cache,
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
                self.deck_interface.render(c, g, &mut self.glyph_cache);
            });
        }

        println!("Well done!");
    }
}
