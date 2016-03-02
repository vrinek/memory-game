use std::fmt;

#[derive(Debug)]
enum Facing {
    Up,
    Down,
}

#[derive(Debug)]
pub struct Card {
    facing: Facing,
    number: u8,
}

impl Card {
    pub fn new(number: u8) -> Card {
        Card { facing: Facing::Down, number: number }
    }
    
    pub fn turn_up(&mut self) {
        self.facing = Facing::Up;
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.facing {
            Facing::Up => write!(f, "[{}]", self.number),
            Facing::Down => write!(f, "[  ]"),
        }
        
    }
}
