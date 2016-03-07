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
            deck.cards.push(Card::new(i));
            deck.cards.push(Card::new(i));
        }

        let mut rng = rand::weak_rng();
        rng.shuffle(&mut deck.cards);
        drop(rng);

        deck
    }

    pub fn turn_up(&mut self, index: usize) {
        self.cards[index - 1].turn_up();
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
