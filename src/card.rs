use std::fmt;

// Possible state transitions:
// START: Closed
// Closed -> Open
// Open -> Closed
// Open -> Scored
#[derive(Debug, PartialEq)]
enum State {
    Open,
    Closed,
    Scored,
}

#[derive(Debug, PartialEq)]
pub struct Card {
    number: u8,
    state: State,
}

impl Card {
    pub fn new(number: u8) -> Card {
        Card {
            number: number,
            state: State::Closed,
        }
    }

    pub fn new_pair(number: u8) -> (Card, Card) {
        (Card::new(number), Card::new(number))
    }

    pub fn open(&mut self) {
        if self.state == State::Closed {
            self.state = State::Open;
        }
    }

    pub fn close(&mut self) {
        if self.state == State::Open {
            self.state = State::Closed;
        }
    }

    pub fn is_open(&self) -> bool {
        self.state == State::Open
    }

    pub fn is_scored(&self) -> bool {
        self.state == State::Scored
    }

    pub fn score(&mut self) {
        if self.state == State::Open {
            self.state = State::Scored;
        }
    }

    pub fn number(&self) -> u8 {
        self.number
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.state {
            State::Open => write!(f, "[{:2}o]", self.number),
            State::Scored => write!(f, "[{:2}s]", self.number),
            State::Closed => write!(f, "[  c]"),
        }
    }
}

#[test]
fn it_initializes_closed() {
    let card = Card::new(1);
    assert_eq!(card.state, State::Closed);
}

#[test]
fn it_can_be_turned_up() {
    let mut card = Card::new(1);
    card.open();
    assert_eq!(card.state, State::Open);
}

#[test]
fn it_responds_whether_it_is_open() {
    let mut card = Card::new(1);
    assert_eq!(card.is_open(), false);
    card.open();
    assert_eq!(card.is_open(), true);
    card.close();
    assert_eq!(card.is_open(), false);
}
