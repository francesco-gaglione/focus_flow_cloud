-- PostgreSQL Database Cleanup Script
-- Run this to completely remove the pomodoro tracking database schema

-- Drop triggers with proper PostgreSQL syntax (must specify table name)
DROP TRIGGER IF EXISTS update_categories_updated_at ON categories;

-- Drop the trigger function (PostgreSQL specific)
DROP FUNCTION IF EXISTS update_updated_at_column() CASCADE;

-- Drop indexes explicitly (PostgreSQL format)
-- Session events indexes
DROP INDEX IF EXISTS idx_events_type_time;
DROP INDEX IF EXISTS idx_events_session_time;

-- Pomodoro sessions indexes
DROP INDEX IF EXISTS idx_sessions_completed;
DROP INDEX IF EXISTS idx_sessions_concentration;
DROP INDEX IF EXISTS idx_sessions_started_at;
DROP INDEX IF EXISTS idx_sessions_category_started;
DROP INDEX IF EXISTS idx_sessions_task_started;

-- Tasks indexes
DROP INDEX IF EXISTS idx_tasks_scheduled_active;
DROP INDEX IF EXISTS idx_tasks_category_active;

-- Categories indexes
DROP INDEX IF EXISTS idx_categories_active;

-- Drop tables in reverse order of dependencies
-- Tables with foreign keys first
DROP TABLE IF EXISTS pomodoro_sessions CASCADE;
DROP TABLE IF EXISTS tasks CASCADE;

-- Tables without dependencies last
DROP TABLE IF EXISTS categories CASCADE;

-- NOTE: VACUUM and ANALYZE cannot be run inside transaction blocks (Diesel migrations)
-- These commands should be run manually after the migration if needed:
-- VACUUM;
-- ANALYZE;