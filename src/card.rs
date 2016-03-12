use std::fmt;

#[derive(Debug, PartialEq)]
enum CardState {
    Open,
    Closed,
    Scored,
}

#[derive(Debug, PartialEq)]
pub struct Card {
    number: u8,
    state: CardState,
}

impl Card {
    pub fn new(number: u8) -> Card {
        Card {
            number: number,
            state: CardState::Closed,
        }
    }

    pub fn new_pair(number: u8) -> (Card, Card) {
        (Card::new(number), Card::new(number))
    }

    pub fn turn_up(&mut self) {
        self.state = CardState::Open;
    }

    pub fn turn_down(&mut self) {
        self.state = CardState::Closed;
    }

    pub fn is_up(&self) -> bool {
        self.state == CardState::Open
    }

    pub fn is_scored(&self) -> bool {
        self.state == CardState::Scored
    }

    pub fn score(&mut self) {
        self.state = CardState::Scored;
    }

    pub fn number(&self) -> u8 {
        self.number
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.state {
            CardState::Open => write!(f, "[{:2}]", self.number),
            CardState::Scored => write!(f, "[{:2}]", self.number),
            CardState::Closed => write!(f, "[  ]"),
        }
    }
}

#[test]
fn it_initializes_closed() {
    let card = Card::new(1);
    assert_eq!(card.state, CardState::Closed);
}

#[test]
fn it_can_be_turned_up() {
    let mut card = Card::new(1);
    card.turn_up();
    assert_eq!(card.state, CardState::Open);
}

#[test]
fn it_responds_whether_it_is_up() {
    let mut card = Card::new(1);
    assert_eq!(card.is_up(), false);
    card.turn_up();
    assert_eq!(card.is_up(), true);
    card.turn_down();
    assert_eq!(card.is_up(), false);
}
