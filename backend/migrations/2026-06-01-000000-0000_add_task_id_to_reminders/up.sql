ALTER TABLE reminders ADD COLUMN task_id UUID REFERENCES tasks(id) ON DELETE CASCADE;

CREATE INDEX IF NOT EXISTS idx_reminders_task_id ON reminders (task_id) WHERE task_id IS NOT NULL;
