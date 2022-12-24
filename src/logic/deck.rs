use rand::seq::SliceRandom;
use rand::thread_rng;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Copy, Clone, Debug)]
pub enum Suit {
    Spades,
    Clubs,
    Hearts,
    Diamonds,
}

impl Suit {
    pub fn to_string(&self) -> String {
        match *self {
            Suit::Spades => "Spades",
            Suit::Clubs => "Clubs",
            Suit::Hearts => "Hearts",
            Suit::Diamonds => "Diamonds"
        }.to_string()
    }
}

pub struct Card {
    pub value: i32,
    pub suit: Suit
}

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut deck = Deck { cards: vec![] };
        deck.initialize_deck();
        deck
    }

    pub fn initialize_deck(&mut self) {
        for suit in Suit::iter() {
            for value in 2..14 {
                self.cards.push(Card { value, suit })
            }
        }
        self.cards.shuffle(&mut thread_rng());
    }
}
