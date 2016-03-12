use std::fmt;

#[derive(Debug, PartialEq)]
enum Facing {
    Up,
    Down,
}

#[derive(Debug)]
pub struct Card {
    facing: Facing,
    number: u8,
    scored: bool,
}

impl Card {
    pub fn new(number: u8) -> Card {
        Card {
            facing: Facing::Down,
            number: number,
            scored: false,
        }
    }

    pub fn new_pair(number: u8) -> (Card, Card) {
        (Card::new(number), Card::new(number))
    }

    pub fn turn_up(&mut self) {
        self.facing = Facing::Up;
    }

    pub fn turn_down(&mut self) {
        self.facing = Facing::Down;
    }

    pub fn is_up(&self) -> bool {
        self.facing == Facing::Up
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.facing {
            Facing::Up => write!(f, "[{:2}]", self.number),
            Facing::Down => write!(f, "[  ]"),
        }
    }
}

#[test]
fn it_initializes_facing_down() {
    let card = Card::new(1);
    assert_eq!(card.facing, Facing::Down);
}

#[test]
fn it_can_be_turned_up() {
    let mut card = Card::new(1);
    card.turn_up();
    assert_eq!(card.facing, Facing::Up);
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
