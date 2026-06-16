use domain::flashcards::value_objects::memory_state::MemoryState;

pub trait SpacedRepetitionPort: Send + Sync {
    fn next_states(&self, memory_state: Option<MemoryState>, elapsed_days: u32) -> SrsNextStates;
}

pub struct SrsNextStates {
    pub again: SrsCardState,
    pub hard: SrsCardState,
    pub good: SrsCardState,
    pub easy: SrsCardState,
}

#[derive(Clone)]
pub struct SrsCardState {
    pub memory_state: MemoryState,
    pub interval_days: u32,
}
