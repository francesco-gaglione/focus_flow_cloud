-- Drop partial indexes that reference deleted_at before dropping the column
DROP INDEX IF EXISTS idx_tasks_user_category_active;
DROP INDEX IF EXISTS idx_tasks_user_scheduled_active;

-- Remove deprecated columns
ALTER TABLE tasks DROP COLUMN scheduled_end_date;
ALTER TABLE tasks DROP COLUMN deleted_at;

-- Remove description from categories (no longer part of domain model)
ALTER TABLE categories DROP COLUMN description;

-- Rename name → title
ALTER TABLE tasks RENAME COLUMN name TO title;

-- Add priority (nullable, validated by check constraint)
ALTER TABLE tasks
    ADD COLUMN priority VARCHAR(10)
    CHECK (priority IN ('low', 'medium', 'high', 'urgent'));

-- Recreate indexes without deleted_at condition
CREATE INDEX idx_tasks_user_category_active
    ON tasks (user_id, category_id, scheduled_date);

CREATE INDEX idx_tasks_user_scheduled_active
    ON tasks (user_id, scheduled_date DESC)
    WHERE scheduled_date IS NOT NULL;

-- Create subtasks table
CREATE TABLE subtasks (
    id          UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    task_id     UUID        NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
    user_id     UUID        NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title       VARCHAR(255) NOT NULL,
    description TEXT,
    completed_at TIMESTAMPTZ,
    sort_order  SMALLINT    NOT NULL DEFAULT 0,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_subtasks_task_id ON subtasks (task_id);
CREATE INDEX idx_subtasks_user_id ON subtasks (user_id);

-- Remove category_id from focus_session
DROP INDEX IF EXISTS idx_sessions_user_category_started;
ALTER TABLE focus_session DROP COLUMN category_id;


ALTER TABLE tasks ADD COLUMN deleted_at TIMESTAMPTZ;

CREATE INDEX idx_tasks_not_deleted ON tasks (user_id) WHERE deleted_at IS NULL;

-- TaskSchedule support
ALTER TABLE tasks
    ADD COLUMN IF NOT EXISTS schedule_type VARCHAR(12) NOT NULL DEFAULT 'unscheduled'
    CHECK (schedule_type IN ('unscheduled', 'all_day', 'at', 'span'));

ALTER TABLE tasks ADD COLUMN IF NOT EXISTS schedule_all_day_date DATE;
ALTER TABLE tasks ADD COLUMN IF NOT EXISTS schedule_duration_secs BIGINT;
