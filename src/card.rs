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
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.facing {
            Facing::Up => write!(f, "[{:2}]", self.number),
            Facing::Down => write!(f, "[  ]"),
        }

    }
}
