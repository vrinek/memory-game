extern crate rand;
use rand::Rng;
use std::io;

mod card;
use card::Card;

fn main() {
    let mut deck: Vec<Card> = vec![];
    for i in 1..9 {
        deck.push(Card::new(i));
        deck.push(Card::new(i));
    }

    let mut rng = rand::weak_rng();
    rng.shuffle(&mut deck);
    drop(rng);

    loop {
        display_deck(&deck);

        println!("Please enter a number from 1-{}:", deck.len());

        let mut picked_card_index = String::new();
        io::stdin().read_line(&mut picked_card_index)
            .expect("Failed to read line");

        let picked_card_index: usize = match picked_card_index.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        deck[picked_card_index - 1].turn_up();
    }
}

fn display_deck(deck: &Vec<Card>) {
    for card in deck {
        print!("{}", card);
    }
    print!("\n");
}
