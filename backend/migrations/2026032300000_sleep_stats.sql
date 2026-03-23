CREATE TABLE IF NOT EXISTS daily_sleep (
    id SERIAL PRIMARY KEY,
    date DATE NOT NULL UNIQUE,
    total_sleep_seconds INT NOT NULL DEFAULT 0,
    deep_sleep_seconds INT NOT NULL DEFAULT 0,
    light_sleep_seconds INT NOT NULL DEFAULT 0,
    rem_sleep_seconds INT NOT NULL DEFAULT 0,
    awake_seconds INT NOT NULL DEFAULT 0,
    quality_score INT NOT NULL DEFAULT 0,
    sleep_start TIMESTAMP WITH TIME ZONE,
    sleep_end TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
