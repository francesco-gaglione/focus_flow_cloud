use crate::flashcards::value_objects::memory_state::MemoryState;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct Flashcard {
    id: Uuid,
    user_id: Uuid,
    front: String,
    back: String,
    memory_state: MemoryState,
    due_date: Option<DateTime<Utc>>,
}

impl Flashcard {
    pub fn new(front: &str, back: &str, user_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            front: front.to_string(),
            back: back.to_string(),
            user_id,
            memory_state: MemoryState::new(0., 0.),
            due_date: Some(Utc::now()),
        }
    }

    pub fn reconstitute(
        id: Uuid,
        user_id: Uuid,
        front: String,
        back: String,
        memory_state: MemoryState,
        due_date: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            id,
            user_id,
            front,
            back,
            memory_state,
            due_date,
        }
    }

    pub fn front(&self) -> &str {
        &self.front
    }

    pub fn back(&self) -> &str {
        &self.back
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn user_id(&self) -> &Uuid {
        &self.user_id
    }

    pub fn memory_state(&self) -> &MemoryState {
        &self.memory_state
    }

    pub fn update_front(&mut self, front: &str) {
        self.front = front.to_string();
    }

    pub fn update_back(&mut self, back: &str) {
        self.back = back.to_string();
    }

    pub fn update_memory_state(&mut self, state: MemoryState) {
        self.memory_state = state;
    }

    pub fn due_date(&self) -> Option<DateTime<Utc>> {
        self.due_date
    }

    pub fn update_due_date(&mut self, due_date: DateTime<Utc>) {
        self.due_date = Some(due_date);
    }
}

#[cfg(test)]
pub mod flashcard_tests {
    use crate::flashcards::entities::flashcard::Flashcard;
    use crate::flashcards::value_objects::memory_state::MemoryState;

    #[test]
    fn test_flashcard_creation() {
        let front = "card front";
        let back = "card back";
        let user_id = uuid::Uuid::new_v4();
        let flashcard = Flashcard::new(front, back, user_id);

        assert_eq!(flashcard.front(), front);
        assert_eq!(flashcard.back(), back);
        assert_eq!(flashcard.memory_state(), &MemoryState::new(0., 0.));
    }

    #[test]
    fn test_flashcard_update_front() {
        let user_id = uuid::Uuid::new_v4();
        let mut flashcard = Flashcard::new("front", "back", user_id);
        flashcard.update_front("new front");
        assert_eq!(flashcard.front(), "new front");
    }

    #[test]
    fn test_flashcard_update_back() {
        let user_id = uuid::Uuid::new_v4();
        let mut flashcard = Flashcard::new("front", "back", user_id);
        flashcard.update_back("new back");
        assert_eq!(flashcard.back(), "new back");
    }

    #[test]
    fn test_flashcard_update_memory_state() {
        let user_id = uuid::Uuid::new_v4();
        let mut flashcard = Flashcard::new("front", "back", user_id);
        let new_state = MemoryState::new(2.5, 0.3);
        flashcard.update_memory_state(new_state.clone());
        assert_eq!(flashcard.memory_state(), &new_state);
    }

    #[test]
    fn test_flashcard_update_due_date() {
        use chrono::Utc;
        let user_id = uuid::Uuid::new_v4();
        let mut flashcard = Flashcard::new("front", "back", user_id);
        assert!(flashcard.due_date().is_some());
        let now = Utc::now();
        flashcard.update_due_date(now);
        assert_eq!(flashcard.due_date(), Some(now));
    }
}
