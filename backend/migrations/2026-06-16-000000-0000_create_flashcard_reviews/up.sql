CREATE TABLE flashcard_reviews (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    flashcard_id UUID NOT NULL REFERENCES flashcards(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    rating VARCHAR(10) NOT NULL,
    reviewed_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX flashcard_reviews_user_id_reviewed_at_idx ON flashcard_reviews(user_id, reviewed_at);
CREATE INDEX flashcard_reviews_flashcard_id_idx ON flashcard_reviews(flashcard_id);
