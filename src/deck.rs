extern crate rand;

use rand::{Rng, SeedableRng, XorShiftRng};
use card::Card;

pub struct Deck {
    rng_seed: Option<[u32; 4]>,
    cards: Vec<Card>,
}

impl Deck {
    pub fn new(half_size: u8, rng_seed: Option<[u32; 4]>) -> Deck {
        let mut deck = Deck {
            rng_seed: rng_seed,
            cards: Vec::with_capacity((half_size * 2) as usize),
        };

        deck.add_cards(half_size);
        deck.shuffle();
        deck
    }

    fn num_facing_up_cards(&self) -> u8 {
        let mut num = 0;
        for card in &self.cards {
            if card.is_up() {
                num += 1;
            }
        }
        num
    }

    fn add_cards(&mut self, half_size: u8) {
        for i in 1..(half_size + 1) {
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
    }

    fn maybe_turn_down_cards(&mut self) {
        if self.num_facing_up_cards() >= 2 {
            for card in &mut self.cards {
                card.turn_down();
            }
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

#[test]
fn it_initializes_with_the_correct_number_of_cards() {
    let deck = Deck::new(3, None);
    assert_eq!(deck.len(), 6);
}

#[test]
fn it_counts_facing_up_cards() {
    let mut deck = Deck::new(3, None);
    deck.turn_up(2);
    assert_eq!(deck.num_facing_up_cards(), 1);
    deck.turn_up(1);
    assert_eq!(deck.num_facing_up_cards(), 2);
}

#[test]
fn it_turns_cards_facing_down_at_every_two_up() {
    let mut deck = Deck::new(3, None);
    deck.turn_up(2);
    deck.turn_up(1);
    assert_eq!(deck.num_facing_up_cards(), 2);
    deck.turn_up(3);
    assert_eq!(deck.num_facing_up_cards(), 1);
}
