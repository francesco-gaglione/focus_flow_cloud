-- Extensions needed for UUID generation
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- ==========================================
-- 1. USERS TABLE
-- ==========================================
CREATE TABLE IF NOT EXISTS users
(
    id              UUID PRIMARY KEY      DEFAULT gen_random_uuid(),
    username        VARCHAR(255) NOT NULL UNIQUE,
    hashed_password VARCHAR(255) NOT NULL,
    role            VARCHAR(20)  NOT NULL DEFAULT 'user' CHECK (role IN ('user', 'admin')),
    created_at      TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    deleted_at      TIMESTAMPTZ           DEFAULT NULL
);

-- ==========================================
-- 2. CATEGORIES
-- ==========================================
CREATE TABLE IF NOT EXISTS categories
(
    id          UUID PRIMARY KEY      DEFAULT gen_random_uuid(),
    user_id     UUID NOT NULL, -- Link to User
    name        VARCHAR(255) NOT NULL,
    description TEXT,
    color       VARCHAR(7) CHECK (color ~ '^#[0-9A-Fa-f]{6}$') NOT NULL,
    created_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    deleted_at  TIMESTAMPTZ           DEFAULT NULL,
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
    -- Name must be unique per user, not globally
    UNIQUE (user_id, name)
);

-- ==========================================
-- 3. TASKS
-- ==========================================
CREATE TABLE IF NOT EXISTS tasks
(
    id             UUID PRIMARY KEY      DEFAULT gen_random_uuid(),
    user_id        UUID NOT NULL, -- Link to User
    template_id    UUID,
    category_id    UUID,
    name           VARCHAR(255) NOT NULL,
    description    TEXT,
    scheduled_date DATE,
    created_at     TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    completed_at   TIMESTAMPTZ,
    deleted_at     TIMESTAMPTZ           DEFAULT NULL,
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
    FOREIGN KEY (category_id) REFERENCES categories (id) ON DELETE CASCADE
);

-- ==========================================
-- 4. FOCUS SESSIONS
-- ==========================================
CREATE TABLE IF NOT EXISTS focus_session
(
    id                  UUID PRIMARY KEY      DEFAULT gen_random_uuid(),
    user_id             UUID NOT NULL, -- Link to User
    task_id             UUID,
    category_id         UUID,
    session_type        VARCHAR(20) NOT NULL DEFAULT 'work'
        CHECK (session_type IN ('work', 'short_break', 'long_break')),
    actual_duration     BIGINT CHECK (actual_duration > 0),
    concentration_score INTEGER CHECK (concentration_score >= 0 AND concentration_score <= 5),
    notes               TEXT,
    started_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    ended_at            TIMESTAMPTZ,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
    FOREIGN KEY (task_id) REFERENCES tasks (id) ON DELETE CASCADE,
    FOREIGN KEY (category_id) REFERENCES categories (id) ON DELETE CASCADE
);

-- ==========================================
-- 5. USER SETTINGS
-- ==========================================
-- Now strictly tied to a user
CREATE TABLE IF NOT EXISTS user_settings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL, -- Link to User
    key VARCHAR(255) NOT NULL,
    value TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
    -- Key must be unique per user
    UNIQUE (user_id, key)
);

-- ==========================================
-- TRIGGERS & FUNCTIONS
-- ==========================================

CREATE OR REPLACE FUNCTION update_updated_at_column()
    RETURNS TRIGGER AS
$$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger for USERS
CREATE TRIGGER update_users_updated_at
    BEFORE UPDATE
    ON users
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

-- Trigger for CATEGORIES
CREATE TRIGGER update_categories_updated_at
    BEFORE UPDATE
    ON categories
    FOR EACH ROW
    WHEN (OLD.deleted_at IS NULL)
EXECUTE FUNCTION update_updated_at_column();

-- Trigger for USER SETTINGS
CREATE TRIGGER update_user_settings_updated_at
    BEFORE UPDATE
    ON user_settings
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

-- ==========================================
-- INDEXES
-- ==========================================

-- Indexes for Users
CREATE INDEX IF NOT EXISTS idx_users_username ON users(username);

-- Updated Indexes for Categories (Added user_id)
CREATE INDEX IF NOT EXISTS idx_categories_user_active
    ON categories (user_id, name) WHERE deleted_at IS NULL;

-- Updated Indexes for Tasks (Added user_id)
CREATE INDEX IF NOT EXISTS idx_tasks_user_category_active
    ON tasks (user_id, category_id, scheduled_date) WHERE deleted_at IS NULL;

CREATE INDEX IF NOT EXISTS idx_tasks_user_scheduled_active
    ON tasks (user_id, scheduled_date DESC) WHERE deleted_at IS NULL AND scheduled_date IS NOT NULL;

-- Updated Indexes for Sessions (Added user_id)
CREATE INDEX IF NOT EXISTS idx_sessions_user_task_started
    ON focus_session (user_id, task_id, started_at DESC);

CREATE INDEX IF NOT EXISTS idx_sessions_user_category_started
    ON focus_session (user_id, category_id, started_at DESC);

CREATE INDEX IF NOT EXISTS idx_sessions_user_started_at
    ON focus_session (user_id, started_at);

-- Updated Index for Settings
CREATE INDEX IF NOT EXISTS idx_user_settings_user_key
    ON user_settings (user_id, key);
