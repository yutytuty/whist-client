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
    value: i32,
    suit: Suit
}
