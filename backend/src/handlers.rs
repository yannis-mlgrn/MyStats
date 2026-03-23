use crate::AppState;
use crate::models::{DailySleep, WeeklyStats, WeeklyStatsHistory};
use crate::services::garmin::{sync_garmin_data, sync_sleep_data};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use chrono::{Datelike, Utc};
use std::sync::Arc;

pub async fn get_weekly_stats(State(state): State<Arc<AppState>>) -> Json<WeeklyStats> {
    // Sync missing data from garmin API
    sync_garmin_data(&state.db).await;

    let now = Utc::now();
    let year = now.iso_week().year();
    let week = now.iso_week().week() as i32;

    // Fetch from database
    let stats = sqlx::query_as!(
        WeeklyStats,
        r#"
        SELECT running_km, cycling_km, swimming_m
        FROM weekly_stats
        WHERE year = $1 AND week = $2
        "#,
        year,
        week
    )
    .fetch_optional(&state.db)
    .await
    .unwrap_or(None);

    match stats {
        Some(s) => Json(s),
        None => Json(WeeklyStats {
            running_km: 0.0,
            cycling_km: 0.0,
            swimming_m: 0.0,
        }),
    }
}

pub async fn get_weekly_history(
    State(state): State<Arc<AppState>>,
) -> Json<Vec<WeeklyStatsHistory>> {
    let raw_stats = sqlx::query!(
        r#"
        SELECT year, week, running_km, cycling_km, swimming_m
        FROM weekly_stats
        ORDER BY year DESC, week DESC
        LIMIT 12
        "#
    )
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();

    let mut stats: Vec<WeeklyStatsHistory> = raw_stats
        .into_iter()
        .map(|row| {
            let monday =
                chrono::NaiveDate::from_isoywd_opt(row.year, row.week as u32, chrono::Weekday::Mon)
                    .unwrap_or_else(|| chrono::NaiveDate::from_ymd_opt(row.year, 1, 1).unwrap());

            WeeklyStatsHistory {
                year: row.year,
                week: row.week,
                monday_date: monday.format("%d/%m").to_string(),
                running_km: row.running_km,
                cycling_km: row.cycling_km,
                swimming_m: row.swimming_m,
            }
        })
        .collect();

    // Reverse to get chronological order (oldest first)
    stats.reverse();
    Json(stats)
}

pub async fn get_sleep_stats(State(state): State<Arc<AppState>>) -> Json<Vec<DailySleep>> {
    // Sync missing data from garmin API
    sync_sleep_data(&state.db).await;

    let stats = sqlx::query_as!(
        DailySleep,
        r#"
        SELECT date, total_sleep_seconds, deep_sleep_seconds, light_sleep_seconds, rem_sleep_seconds, awake_seconds, quality_score,
               COALESCE(steps, 0) as "steps!", COALESCE(active_calories, 0) as "active_calories!",
               resting_heart_rate, COALESCE(step_goal, 10000) as "step_goal!"
        FROM daily_sleep
        ORDER BY date DESC
        LIMIT 7
        "#
    )
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();

    Json(stats)
}

pub async fn update_step_goal(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<crate::models::UpdateStepGoal>,
) -> impl IntoResponse {
    let _ = sqlx::query!(
        r#"
        UPDATE daily_sleep
        SET step_goal = $1
        WHERE date = $2
        "#,
        payload.goal,
        payload.date
    )
    .execute(&state.db)
    .await;

    StatusCode::OK
}
