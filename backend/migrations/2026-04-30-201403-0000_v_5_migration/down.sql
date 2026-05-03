-- Drop subtasks table
DROP TABLE IF EXISTS subtasks;

-- Restore partial indexes that reference deleted_at
DROP INDEX IF EXISTS idx_tasks_user_category_active;
DROP INDEX IF EXISTS idx_tasks_user_scheduled_active;

-- Remove added column
ALTER TABLE tasks DROP COLUMN priority;

-- Rename title → name
ALTER TABLE tasks RENAME COLUMN title TO name;

-- Restore removed columns
ALTER TABLE tasks ADD COLUMN deleted_at TIMESTAMPTZ;
ALTER TABLE tasks ADD COLUMN scheduled_end_date TIMESTAMPTZ;

-- Recreate original partial indexes
CREATE INDEX idx_tasks_user_category_active
    ON tasks (user_id, category_id, scheduled_date)
    WHERE deleted_at IS NULL;

CREATE INDEX idx_tasks_user_scheduled_active
    ON tasks (user_id, scheduled_date DESC)
    WHERE deleted_at IS NULL AND scheduled_date IS NOT NULL;
