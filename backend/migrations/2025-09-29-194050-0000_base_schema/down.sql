-- ==========================================
-- DROPPING TRIGGERS
-- ==========================================
DROP TRIGGER IF EXISTS update_user_settings_updated_at ON user_settings;
DROP TRIGGER IF EXISTS update_categories_updated_at ON categories;
DROP TRIGGER IF EXISTS update_users_updated_at ON users;

-- ==========================================
-- DROPPING FUNCTIONS
-- ==========================================
DROP FUNCTION IF EXISTS update_updated_at_column() CASCADE;

-- ==========================================
-- DROPPING INDEXES
-- ==========================================

-- User Settings
DROP INDEX IF EXISTS idx_user_settings_user_key;

-- Focus Sessions
DROP INDEX IF EXISTS idx_sessions_user_started_at;
DROP INDEX IF EXISTS idx_sessions_user_category_started;
DROP INDEX IF EXISTS idx_sessions_user_task_started;

-- Tasks
DROP INDEX IF EXISTS idx_tasks_user_scheduled_active;
DROP INDEX IF EXISTS idx_tasks_user_category_active;

-- Categories
DROP INDEX IF EXISTS idx_categories_user_active;

-- Users
DROP INDEX IF EXISTS idx_users_username;
DROP INDEX IF EXISTS idx_users_email;

-- ==========================================
-- DROPPING TABLES
-- ==========================================

DROP TABLE IF EXISTS user_settings CASCADE;
DROP TABLE IF EXISTS focus_session CASCADE;
DROP TABLE IF EXISTS tasks CASCADE;
DROP TABLE IF EXISTS categories CASCADE;
DROP TABLE IF EXISTS users CASCADE;

-- DROP EXTENSION IF EXISTS "uuid-ossp";
-- DROP EXTENSION IF EXISTS "pgcrypto";ASCADE;
