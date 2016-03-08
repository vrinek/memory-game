extern crate rand;

use rand::Rng;
use card::Card;

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new(half_size: u8) -> Deck {
        let mut deck = Deck { cards: vec![] };
        for i in 1..half_size {
            let (a, b) = Card::new_pair(i);
            deck.cards.push(a);
            deck.cards.push(b);
        }

        let mut rng = rand::weak_rng();
        rng.shuffle(&mut deck.cards);
        drop(rng);

        deck
    }

    pub fn turn_up(&mut self, index: usize) {
        self.maybe_turn_down_cards();
        self.cards[index - 1].turn_up();
    }

    fn maybe_turn_down_cards(&mut self) {
        let mut facing_up_cards = 0;

        for card in &self.cards {
            if card.is_up() && !card.is_scored() {
                facing_up_cards += 1;
            }
        }

        if facing_up_cards >= 2 {
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
