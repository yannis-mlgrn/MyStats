-- Add resting heart rate and step goal to daily_sleep table
ALTER TABLE daily_sleep ADD COLUMN IF NOT EXISTS resting_heart_rate INTEGER;
ALTER TABLE daily_sleep ADD COLUMN IF NOT EXISTS step_goal INTEGER DEFAULT 10000;
