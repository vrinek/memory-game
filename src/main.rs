extern crate rand;

use std::io;

mod card;
mod deck;

use deck::Deck;

fn main() {
    let mut deck = Deck::new(8, None);

    loop {
        deck.display_cards();

        println!("Please enter a number from 1-{}:", deck.len());

        let mut picked_card_index = String::new();
        io::stdin()
            .read_line(&mut picked_card_index)
            .expect("Failed to read line");

        let picked_card_index: usize = match picked_card_index.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        deck.turn_up(picked_card_index);
    }
}
