#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
pub struct MemoryState {
    stability: f32,
    difficulty: f32,
}

impl MemoryState {
    pub fn new(stability: f32, difficulty: f32) -> Self {
        Self {
            stability,
            difficulty,
        }
    }

    pub fn stability(&self) -> f32 {
        self.stability
    }

    pub fn difficulty(&self) -> f32 {
        self.difficulty
    }

    pub fn update_stability(&mut self, stability: f32) {
        self.stability = stability;
    }

    pub fn update_difficulty(&mut self, difficulty: f32) {
        self.difficulty = difficulty;
    }
}

#[cfg(test)]
mod test {
    use crate::flashcards::value_objects::memory_state::MemoryState;

    #[test]
    fn test_default() {
        let state = MemoryState::default();

        assert_eq!(state.stability(), 0.0);
        assert_eq!(state.difficulty(), 0.0);
    }

    #[test]
    fn test_creation() {
        let state = MemoryState::new(1.0, 2.0);

        assert_eq!(state.stability(), 1.0);
        assert_eq!(state.difficulty(), 2.0);
    }

    #[test]
    fn test_update_stability() {
        let mut state = MemoryState::new(1.0, 2.0);
        state.update_stability(2.0);
        assert_eq!(state.stability(), 2.0);
    }

    #[test]
    fn test_update_difficulty() {
        let mut state = MemoryState::new(1.0, 2.0);
        state.update_difficulty(4.0);
        assert_eq!(state.difficulty(), 4.0);
    }
}
