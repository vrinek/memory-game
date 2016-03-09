extern crate rand;

use rand::{Rng, SeedableRng, XorShiftRng};
use card::Card;

pub struct Deck {
    rng_seed: Option<[u32; 4]>,
    cards: Vec<Card>,
    num_facing_up_cards: u8,
}

impl Deck {
    pub fn new(half_size: u8, rng_seed: Option<[u32; 4]>) -> Deck {
        let mut deck = Deck {
            rng_seed: rng_seed,
            cards: Vec::with_capacity((half_size * 2) as usize),
            num_facing_up_cards: 0,
        };

        deck.add_cards(half_size);
        deck.shuffle();
        deck
    }

    fn add_cards(&mut self, half_size: u8) {
        for i in 1..half_size {
            let (a, b) = Card::new_pair(i);
            self.cards.push(a);
            self.cards.push(b);
        }
    }

    fn shuffle(&mut self) {
        let mut rng = if let Some(seed) = self.rng_seed {
            XorShiftRng::from_seed(seed)
        } else {
            rand::weak_rng()
        };
        rng.shuffle(&mut self.cards);
    }

    pub fn turn_up(&mut self, index: usize) {
        self.maybe_turn_down_cards();
        self.cards[index - 1].turn_up();
        self.num_facing_up_cards += 1;
    }

    fn maybe_turn_down_cards(&mut self) {
        if self.num_facing_up_cards >= 2 {
            for card in &mut self.cards {
                card.turn_down();
            }
            self.num_facing_up_cards = 0;
        }
    }

    pub fn len(&self) -> usize {
        self.cards.len()
    }

    pub fn display_cards(&self) {
        for card in &self.cards {
            print!("{}", card);
        }
        print!("\n");
    }
}
