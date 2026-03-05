ALTER TABLE tasks
    DROP CONSTRAINT IF EXISTS check_dates_order;

ALTER TABLE tasks
    DROP COLUMN IF EXISTS scheduled_end_date,
    ALTER COLUMN scheduled_date TYPE DATE
    USING scheduled_date::DATE;
