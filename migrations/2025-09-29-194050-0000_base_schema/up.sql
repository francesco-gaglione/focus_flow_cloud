-- Extensions needed for UUID generation
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- Categories/Areas table
CREATE TABLE IF NOT EXISTS categories
(
    id          UUID PRIMARY KEY      DEFAULT gen_random_uuid(),
    name        VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    color       VARCHAR(7) CHECK (color ~ '^#[0-9A-Fa-f]{6}$') NOT NULL,
    created_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    deleted_at  TIMESTAMPTZ           DEFAULT NULL
);

-- Individual task instances
CREATE TABLE IF NOT EXISTS tasks
(
    id             UUID PRIMARY KEY      DEFAULT gen_random_uuid(),
    template_id    UUID,
    category_id    UUID,
    name           VARCHAR(255) NOT NULL,
    description    TEXT,
    scheduled_date DATE,
    created_at     TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    completed_at   TIMESTAMPTZ,
    deleted_at     TIMESTAMPTZ           DEFAULT NULL,
    FOREIGN KEY (category_id) REFERENCES categories (id) ON DELETE CASCADE
);

-- Pomodoro sessions table
CREATE TABLE IF NOT EXISTS focus_session
(
    id                       UUID PRIMARY KEY     DEFAULT gen_random_uuid(),
    task_id                  UUID,
    category_id              UUID,
    session_type             VARCHAR(20) NOT NULL DEFAULT 'work'
        CHECK (session_type IN ('work', 'short_break', 'long_break')),
    actual_duration_minutes  BIGINT CHECK (actual_duration_minutes > 0),
    concentration_score      INTEGER CHECK (concentration_score >= 0 AND concentration_score <= 5),
    notes                    TEXT,
    started_at               TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    ended_at                 TIMESTAMPTZ,
    created_at               TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    FOREIGN KEY (task_id) REFERENCES tasks (id) ON DELETE CASCADE,
    FOREIGN KEY (category_id) REFERENCES categories (id) ON DELETE CASCADE
);

CREATE OR REPLACE FUNCTION update_updated_at_column()
    RETURNS TRIGGER AS
$$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger for automatic timestamp updates
CREATE TRIGGER update_categories_updated_at
    BEFORE UPDATE
    ON categories
    FOR EACH ROW
    WHEN (OLD.deleted_at IS NULL)
EXECUTE FUNCTION update_updated_at_column();

-- Performance-optimized indexes
CREATE INDEX IF NOT EXISTS idx_categories_active
    ON categories (name) WHERE deleted_at IS NULL;

CREATE INDEX IF NOT EXISTS idx_tasks_category_active
    ON tasks (category_id, scheduled_date) WHERE deleted_at IS NULL;

CREATE INDEX IF NOT EXISTS idx_tasks_scheduled_active
    ON tasks (scheduled_date DESC) WHERE deleted_at IS NULL AND scheduled_date IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_sessions_task_started
    ON focus_session (task_id, started_at DESC);

CREATE INDEX IF NOT EXISTS idx_sessions_category_started
    ON focus_session (category_id, started_at DESC);

CREATE INDEX IF NOT EXISTS idx_sessions_started_at
    ON focus_session (started_at);

CREATE INDEX IF NOT EXISTS idx_sessions_concentration
    ON focus_session (concentration_score) WHERE concentration_score IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_sessions_completed
    ON focus_session (actual_duration_minutes) WHERE actual_duration_minutes IS NOT NULL;
