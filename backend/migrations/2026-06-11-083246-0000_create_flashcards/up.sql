-- Flashcard folders (hierarchical, flashcard-specific)
CREATE TABLE flashcard_folders
(
    id         UUID PRIMARY KEY      DEFAULT gen_random_uuid(),
    user_id    UUID         NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    name       VARCHAR(255) NOT NULL,
    parent_id  UUID REFERENCES flashcard_folders (id) ON DELETE CASCADE,
    path       TEXT         NOT NULL DEFAULT '/',
    created_at TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_flashcard_folders_user_id ON flashcard_folders (user_id);
CREATE INDEX idx_flashcard_folders_parent_id ON flashcard_folders (parent_id);

CREATE TRIGGER update_flashcard_folders_updated_at
    BEFORE UPDATE
    ON flashcard_folders
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Flashcards with FSRS memory state
CREATE TABLE flashcards
(
    id         UUID PRIMARY KEY     DEFAULT gen_random_uuid(),
    user_id    UUID        NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    front      TEXT        NOT NULL,
    back       TEXT        NOT NULL,
    stability  REAL        NOT NULL DEFAULT 0.0,
    difficulty REAL        NOT NULL DEFAULT 0.0,
    due_date   TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_flashcards_user_id ON flashcards (user_id);
CREATE INDEX idx_flashcards_due_date ON flashcards (user_id, due_date) WHERE due_date IS NOT NULL;

CREATE TRIGGER update_flashcards_updated_at
    BEFORE UPDATE
    ON flashcards
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Join table: flashcard membership in folders (many-to-many)
CREATE TABLE flashcard_folder_items
(
    flashcard_id UUID NOT NULL REFERENCES flashcards (id) ON DELETE CASCADE,
    folder_id    UUID NOT NULL REFERENCES flashcard_folders (id) ON DELETE CASCADE,
    PRIMARY KEY (flashcard_id, folder_id)
);

CREATE INDEX idx_flashcard_folder_items_folder_id ON flashcard_folder_items (folder_id);
