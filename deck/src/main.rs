use core::num;

use rand::{seq::SliceRandom, thread_rng};

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        let suits = ["Hearts", "Spades", "Diamonds", "Clubs"];
        let values = ["Ace", "Two", "Three"];
        let mut cards = vec![];
        for suit in suits {
            for value in values {
                let card = format!("{} of {}", value, suit);
                cards.push(card);
            }
        }
        Deck { cards }
    }
    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }
    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off(self.cards.len() - num_cards)
    }
}

fn main() {
    let mut deck = Deck::new();
    //deck.shuffle();
    println!("here is your deck: {:#?}", deck);
    let hand = deck.deal(3);
    println!("here is your deck: {:#?}", deck);
    println!("here is your hand: {:#?}", hand);
}
