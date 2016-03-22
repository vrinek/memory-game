use piston_window::*;
use opengl_graphics::glyph_cache::GlyphCache;

use card_interface::CardInterface;
use deck::Deck;

pub struct DeckInterface {
    pub cards: Vec<CardInterface>,
}

fn inside(coords: [f64; 2], bounds: [f64; 4]) -> bool {
    coords[0] >= bounds[0] && coords[0] <= bounds[2] && coords[1] >= bounds[1] &&
    coords[1] <= bounds[3]
}

impl DeckInterface {
    pub fn new() -> DeckInterface {
        DeckInterface {
            cards: vec![
                CardInterface::new(1),
                CardInterface::new(2),
                CardInterface::new(3),
                CardInterface::new(4),
                CardInterface::new(5),
                CardInterface::new(6),
                CardInterface::new(7),
                CardInterface::new(8),
            ],
        }
    }

    pub fn hover(&mut self, mouse_position: [f64; 2]) {
        for card in &mut self.cards {
            if inside(mouse_position, card.bounds()) {
                card.over = true;
            } else {
                card.over = false;
            }
        }
    }

    pub fn apply_button(&mut self, deck: &mut Deck, button: Button) {
        if button == Button::Mouse(MouseButton::Left) {
            for card in &self.cards {
                if card.over {
                    deck.turn_up(card.index);
                }
            }

            for card in &mut self.cards {
                card.selected = deck.is_open(card.index);
                card.scored = deck.is_scored(card.index);
            }
        }
    }

    pub fn render<G>(&self, context: Context, graphics: &mut G, cache: &mut GlyphCache<'static>)
        where G: Graphics
    {
        for card in &self.cards {
            card.render(context, graphics, cache);
        }
    }
}
