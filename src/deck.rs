extern crate rand;

use rand::{Rng, SeedableRng, XorShiftRng};
use card::Card;

pub struct Deck {
    rng_seed: Option<[u32; 4]>,
    cards: Vec<Card>,
}

impl Deck {
    pub fn new(size: u8, rng_seed: Option<[u32; 4]>) -> Deck {
        let mut deck = Deck {
            rng_seed: rng_seed,
            cards: Vec::with_capacity(size as usize),
        };

        deck.add_cards(size / 2);
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

    fn num_scored_cards(&self) -> u8 {
        let mut num = 0;
        for card in &self.cards {
            if card.is_scored() {
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
        self.maybe_score_cards();
    }

    fn maybe_turn_down_cards(&mut self) {
        if self.num_facing_up_cards() >= 2 {
            for card in &mut self.cards {
                if card.is_up() {
                    card.turn_down();
                }
            }
        }
    }

    // This method iterates twice over the cards. This is not terribly effective but we can safely
    // assume that most memory games will not exceed 100s of cards.
    fn maybe_score_cards(&mut self) {
        if self.num_facing_up_cards() != 2 {
            return;
        }

        let mut matching_number: Option<u8> = None;
        for card in &self.cards {
            if card.is_up() {
                if matching_number == None {
                    // this is the first open card, hopefully the first part of the match
                    matching_number = Some(card.number());
                } else if Some(card.number()) == matching_number {
                    // the second other open card was a match
                    break;
                } else {
                    // the second other open card was not a match
                    matching_number = None;
                }
            }
        }

        if let Some(matching_number) = matching_number {
            for card in &mut self.cards {
                if card.number() == matching_number {
                    card.score();
                }
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

// For reference:
// Deck::new(6, Some([1, 2, 3, 4]));
// [ 1][ 3][ 3][ 2][ 2][ 1]

#[test]
fn it_initializes_with_the_correct_number_of_cards() {
    let deck = Deck::new(6, Some([1, 2, 3, 4]));
    assert_eq!(deck.len(), 6);
}

#[test]
fn it_counts_facing_up_cards() {
    let mut deck = Deck::new(6, Some([1, 2, 3, 4]));
    deck.turn_up(2);
    assert_eq!(deck.num_facing_up_cards(), 1);
    deck.turn_up(1);
    assert_eq!(deck.num_facing_up_cards(), 2);
}

#[test]
fn it_turns_cards_facing_down_at_every_two_up() {
    let mut deck = Deck::new(6, Some([1, 2, 3, 4]));
    deck.turn_up(2);
    deck.turn_up(1);
    assert_eq!(deck.num_facing_up_cards(), 2);
    deck.turn_up(3);
    assert_eq!(deck.num_facing_up_cards(), 1);
}

#[test]
fn it_scores_same_cards() {
    let mut deck = Deck::new(6, Some([1, 2, 3, 4]));

    deck.turn_up(1);
    deck.turn_up(6);
    // Match the 1s
    assert_eq!(deck.num_scored_cards(), 2);
    assert_eq!(deck.num_facing_up_cards(), 0);

    deck.turn_up(2);
    deck.turn_up(4);
    // No match
    assert_eq!(deck.num_scored_cards(), 2);
    assert_eq!(deck.num_facing_up_cards(), 2);

    deck.turn_up(2);
    deck.turn_up(3);
    // Match the 3s
    assert_eq!(deck.num_scored_cards(), 4);
    assert_eq!(deck.num_facing_up_cards(), 0);
}
