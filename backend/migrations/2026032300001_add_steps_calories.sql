-- Add steps and calories to daily_sleep table
ALTER TABLE daily_sleep ADD COLUMN IF NOT EXISTS steps INTEGER DEFAULT 0;
ALTER TABLE daily_sleep ADD COLUMN IF NOT EXISTS active_calories INTEGER DEFAULT 0;
