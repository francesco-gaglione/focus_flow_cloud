#[derive(Debug, Clone, PartialEq)]
pub enum CardState {
    New,
    Learning,
    Review,
    Relearning,
}
