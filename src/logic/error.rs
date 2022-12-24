#[derive(thiserror::Error, Debug)]
pub enum LogicError {
    #[error("No more cards in deck")]
    DeckEmptyError
}
