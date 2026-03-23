use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct WeeklyStats {
    pub running_km: f64,
    pub cycling_km: f64,
    pub swimming_m: f64,
}

#[derive(Serialize)]
pub struct WeeklyStatsHistory {
    pub year: i32,
    pub week: i32,
    pub monday_date: String,
    pub running_km: f64,
    pub cycling_km: f64,
    pub swimming_m: f64,
}

#[derive(Serialize, FromRow)]
pub struct DailySleep {
    pub date: chrono::NaiveDate,
    pub total_sleep_seconds: i32,
    pub deep_sleep_seconds: i32,
    pub light_sleep_seconds: i32,
    pub rem_sleep_seconds: i32,
    pub awake_seconds: i32,
    pub quality_score: i32,
    pub steps: i32,
    pub active_calories: i32,
    pub resting_heart_rate: Option<i32>,
    pub step_goal: i32,
}

#[derive(Deserialize)]
pub struct UpdateStepGoal {
    pub date: chrono::NaiveDate,
    pub goal: i32,
}
