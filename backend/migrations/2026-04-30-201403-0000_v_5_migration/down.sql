-- Rollback subtasks table
DROP TABLE subtasks;

-- Undo the addition of priority and rename title back to name
-- We assume the original name column structure is restored
ALTER TABLE tasks DROP COLUMN priority;
ALTER TABLE tasks RENAME COLUMN title TO name;

-- Drop the indexes created in up.sql
DROP INDEX IF EXISTS idx_tasks_user_category_active;
DROP INDEX IF EXISTS idx_tasks_user_scheduled_active;
DROP INDEX IF EXISTS idx_subtasks_task_id;
DROP INDEX IF EXISTS idx_subtasks_user_id;

-- Restore dropped columns (adding them back is necessary to reverse the drops)
ALTER TABLE tasks ADD COLUMN scheduled_end_date TIMESTAMP;
ALTER TABLE tasks ADD COLUMN deleted_at TIMESTAMP;

-- Restore description to categories (Conditional addition to prevent "already exists" error)
DO $$ BEGIN
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'categories' AND column_name = 'description') THEN
        ALTER TABLE categories ADD COLUMN description TEXT;
    END IF;
END $$;
