use std::borrow::Borrow;
use std::collections::HashMap;
use std::ops::Deref;
use rand::seq::SliceRandom;
use rand::thread_rng;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use crate::logic::error::LogicError;
use crate::logic::player::Player;

#[derive(EnumIter, Copy, Clone, Debug, Hash, PartialEq, Eq)]
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

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct Card {
    pub value: i32,
    pub suit: Suit,
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

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}

pub struct Table {
    table: HashMap<Player, Option<Card>>,
}

impl Table {
    pub fn new(players: [Player; 4]) -> Table {
        let mut hashmap = HashMap::new();
        for player in players {
            hashmap.insert(player, None);
        }

        Table { table: hashmap }
    }

    pub fn place_card(&mut self, player: Player, card: Card) {
        if self.table.contains_key(&player) {
            self.table.insert(player, Some(card));
        }
    }
}
