use crate::logic::table::{Card, Deck, Table};
use crate::logic::error::LogicError;

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct Player {
    name: String,
    hand: Vec<Card>,
}

impl Player {
    pub fn new(name: &str) -> Player {
        Player { name: name.to_string(), hand: vec![] }
    }

    pub fn draw(&mut self, deck: &mut Deck) -> Option<Card> {
        return if let Some(card) = deck.draw() {
            self.hand.push(card.clone());
            Some(card)
        } else {
            None
        }
    }

    pub fn play_card(&mut self, table: &mut Table, card: Card) {
        table.place_card(self.clone(), card);
    }
}
