use uuid::Uuid;

use crate::flashcards::value_objects::card_state::CardState;

pub struct Flashcard {
    id: Uuid,
    user_id: Uuid,
    front: String,
    back: String,
    state: CardState,
}

impl Flashcard {
    pub fn new(front: &str, back: &str, user_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            front: front.to_string(),
            back: back.to_string(),
            state: CardState::New,
            user_id,
        }
    }

    pub fn update_state(&mut self, state: CardState) {
        self.state = state;
    }

    pub fn front(&self) -> &str {
        &self.front
    }

    pub fn back(&self) -> &str {
        &self.back
    }

    pub fn state(&self) -> &CardState {
        &self.state
    }

    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn user_id(&self) -> &Uuid {
        &self.user_id
    }

    pub fn update_front(&mut self, front: &str) {
        self.front = front.to_string();
    }

    pub fn update_back(&mut self, back: &str) {
        self.back = back.to_string();
    }
}

#[cfg(test)]
pub mod flashcard_tests {
    use crate::flashcards::{entities::flashcard::Flashcard, value_objects::card_state::CardState};

    #[test]
    fn test_flashcard_creation() {
        let front = "card front";
        let back = "card back";
        let user_id = uuid::Uuid::new_v4();
        let flashcard = Flashcard::new(front, back, user_id);

        assert_eq!(flashcard.front(), front);
        assert_eq!(flashcard.back(), back);
        assert_eq!(flashcard.state(), &CardState::New);
    }

    #[test]
    fn test_flashcard_state_transition() {
        let user_id = uuid::Uuid::new_v4();
        let mut flashcard = Flashcard::new("front", "back", user_id);

        assert_eq!(flashcard.state, CardState::New);
        flashcard.update_state(CardState::Learning);
        assert_eq!(flashcard.state, CardState::Learning);
        flashcard.update_state(CardState::Review);
        assert_eq!(flashcard.state, CardState::Review);
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
}
