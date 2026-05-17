-- Undo TaskSchedule columns
ALTER TABLE tasks DROP COLUMN IF EXISTS schedule_duration_secs;
ALTER TABLE tasks DROP COLUMN IF EXISTS schedule_all_day_date;
ALTER TABLE tasks DROP COLUMN IF EXISTS schedule_type;

-- Undo soft-delete additions (last in up.sql, first to undo)
DROP INDEX IF EXISTS idx_tasks_not_deleted;
ALTER TABLE tasks DROP COLUMN deleted_at;

-- Restore category_id to focus_session
ALTER TABLE focus_session ADD COLUMN category_id UUID REFERENCES categories(id);

-- Rollback subtasks table
DROP TABLE subtasks;

-- Drop the indexes created in up.sql
DROP INDEX IF EXISTS idx_tasks_user_category_active;
DROP INDEX IF EXISTS idx_tasks_user_scheduled_active;
DROP INDEX IF EXISTS idx_subtasks_task_id;
DROP INDEX IF EXISTS idx_subtasks_user_id;

-- Undo the addition of priority and rename title back to name
ALTER TABLE tasks DROP COLUMN priority;
ALTER TABLE tasks RENAME COLUMN title TO name;

-- Restore dropped columns
ALTER TABLE tasks ADD COLUMN scheduled_end_date TIMESTAMP;
ALTER TABLE tasks ADD COLUMN deleted_at TIMESTAMP;

-- Restore description to categories
DO $$ BEGIN
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name = 'categories' AND column_name = 'description') THEN
        ALTER TABLE categories ADD COLUMN description TEXT;
    END IF;
END $$;
