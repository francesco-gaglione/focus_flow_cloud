ALTER TABLE tasks
    ADD COLUMN scheduled_end_date TIMESTAMPTZ,
    ALTER COLUMN scheduled_date TYPE TIMESTAMPTZ USING scheduled_date::TIMESTAMPTZ;
