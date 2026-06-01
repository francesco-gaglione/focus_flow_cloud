CREATE TABLE IF NOT EXISTS reminders
(
    id            UUID         PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id       UUID         NOT NULL,
    title         VARCHAR(255) NOT NULL,
    description   TEXT         NOT NULL DEFAULT '',
    date          TIMESTAMPTZ  NOT NULL,
    reminder_sent BOOLEAN      NOT NULL DEFAULT FALSE,
    created_at    TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at    TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
);

CREATE TRIGGER update_reminders_updated_at
    BEFORE UPDATE ON reminders
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

CREATE INDEX IF NOT EXISTS idx_reminders_user_date
    ON reminders (user_id, date) WHERE reminder_sent = FALSE;
