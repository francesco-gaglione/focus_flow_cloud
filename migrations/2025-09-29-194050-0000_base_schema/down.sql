-- Drop user preferences table
DROP TABLE IF EXISTS user_preferences CASCADE;
-- Drop triggers with proper PostgreSQL syntax (must specify table name)
DROP TRIGGER IF EXISTS update_categories_updated_at ON categories;

-- Drop the trigger function (PostgreSQL specific)
DROP FUNCTION IF EXISTS update_updated_at_column() CASCADE;

-- Drop indexes explicitly (PostgreSQL format)
DROP INDEX IF EXISTS idx_sessions_concentration;
DROP INDEX IF EXISTS idx_sessions_started_at;
DROP INDEX IF EXISTS idx_sessions_category_started;
DROP INDEX IF EXISTS idx_sessions_task_started;
DROP INDEX IF EXISTS idx_tasks_scheduled_active;
DROP INDEX IF EXISTS idx_tasks_category_active;
DROP INDEX IF EXISTS idx_categories_active;

-- Drop tables in reverse order of dependencies
DROP TABLE IF EXISTS focus_session CASCADE;
DROP TABLE IF EXISTS tasks CASCADE;
DROP TABLE IF EXISTS categories CASCADE;
