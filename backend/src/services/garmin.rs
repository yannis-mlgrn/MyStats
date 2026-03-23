#![allow(clippy::collapsible_if)]
use sqlx::PgPool;
use chrono::{Datelike, Utc, Duration};

async fn get_garmin_token() -> Option<String> {
    let token_env = std::env::var("GARMIN_TOKEN").unwrap_or_default();
    if !token_env.is_empty() {
        return Some(token_env);
    }

    let email = std::env::var("GARMIN_EMAIL").unwrap_or_default();
    let password = std::env::var("GARMIN_PASSWORD").unwrap_or_default();

    if email.is_empty() || password.is_empty() {
        println!("Missing Garmin credentials (EMAIL/PASSWORD or TOKEN)");
        return None;
    }

    let mut gc = garmin_client::GarminClient::new();
    if gc.login(&email, &password).await {
        if let Ok(session_data) = std::fs::read_to_string(".garmin_session.json") {
            if let Ok(session) = serde_json::from_str::<serde_json::Value>(&session_data) {
                return session["token"].as_str().map(std::string::ToString::to_string);
            }
        }
    }
    None
}

pub async fn sync_sleep_data(pool: &PgPool) {
    let token = match get_garmin_token().await {
        Some(t) => t,
        None => return,
    };

    let now = Utc::now().naive_utc().date();
    let req_client = reqwest::Client::new();

    // Sync last 7 days
    for i in 0..7 {
        let target_date = now - Duration::days(i);
        let date_str = target_date.format("%Y-%m-%d").to_string();

        // 1. Fetch Daily Summary Chart (Steps)
        let mut steps = 0;
        let calories = 0; // Defaulting for now

        let summary_url = format!("https://connectapi.garmin.com/wellness-service/wellness/dailySummaryChart?date={}", date_str);
        if let Ok(sum_resp) = req_client.get(&summary_url)
            .header("Authorization", format!("Bearer {}", token))
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36")
            .header("di-backend", "connectapi.garmin.com")
            .send().await {

            if sum_resp.status().is_success() {
                if let Ok(sum_json) = sum_resp.json::<serde_json::Value>().await {
                    if let Some(arr) = sum_json.as_array() {
                        for item in arr {
                            steps += item["steps"].as_i64().unwrap_or(0) as i32;
                        }
                    }
                }
            }
        }

        // 2. Fetch Sleep Data & RHR
        let mut total_s = 0;
        let mut deep_s = 0;
        let mut light_s = 0;
        let mut rem_s = 0;
        let mut awake_s = 0;
        let mut score = 0;
        let mut rhr: Option<i32> = None;

        let sleep_url = format!("https://connectapi.garmin.com/wellness-service/wellness/dailySleepData?date={}", date_str);
        if let Ok(resp) = req_client.get(&sleep_url)
            .header("Authorization", format!("Bearer {}", token))
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36")
            .header("di-backend", "connectapi.garmin.com")
            .send().await {

            if resp.status().is_success() {
                if let Ok(json) = resp.json::<serde_json::Value>().await {
                    // Extract RHR from root
                    rhr = json["restingHeartRate"].as_i64().map(|v| v as i32);

                    let sleep_dto = &json["dailySleepDTO"];
                    if !sleep_dto.is_null() {
                        total_s = sleep_dto["sleepTimeSeconds"].as_i64().unwrap_or(0) as i32;
                        deep_s = sleep_dto["deepSleepSeconds"].as_i64().unwrap_or(0) as i32;
                        light_s = sleep_dto["lightSleepSeconds"].as_i64().unwrap_or(0) as i32;
                        rem_s = sleep_dto["remSleepSeconds"].as_i64().unwrap_or(0) as i32;
                        awake_s = sleep_dto["awakeSleepSeconds"].as_i64().unwrap_or(0) as i32;

                        score = sleep_dto["sleepScores"]["overall"]["value"]
                            .as_i64()
                            .or(json["sleepScoreDTO"]["value"].as_i64())
                            .or(sleep_dto["sleepQualityTypeDTO"]["qualityValue"].as_i64())
                            .unwrap_or(0) as i32;
                    }
                }
            }
        }

        // 3. UPSERT into Database
        let _ = sqlx::query!(
            r#"
            INSERT INTO daily_sleep (date, total_sleep_seconds, deep_sleep_seconds, light_sleep_seconds, rem_sleep_seconds, awake_seconds, quality_score, steps, active_calories, resting_heart_rate)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            ON CONFLICT (date) DO UPDATE SET
                total_sleep_seconds = CASE WHEN EXCLUDED.total_sleep_seconds > 0 THEN EXCLUDED.total_sleep_seconds ELSE daily_sleep.total_sleep_seconds END,
                deep_sleep_seconds = CASE WHEN EXCLUDED.deep_sleep_seconds > 0 THEN EXCLUDED.deep_sleep_seconds ELSE daily_sleep.deep_sleep_seconds END,
                light_sleep_seconds = CASE WHEN EXCLUDED.light_sleep_seconds > 0 THEN EXCLUDED.light_sleep_seconds ELSE daily_sleep.light_sleep_seconds END,
                rem_sleep_seconds = CASE WHEN EXCLUDED.rem_sleep_seconds > 0 THEN EXCLUDED.rem_sleep_seconds ELSE daily_sleep.rem_sleep_seconds END,
                awake_seconds = CASE WHEN EXCLUDED.awake_seconds > 0 THEN EXCLUDED.awake_seconds ELSE daily_sleep.awake_seconds END,
                quality_score = CASE WHEN EXCLUDED.quality_score > 0 THEN EXCLUDED.quality_score ELSE daily_sleep.quality_score END,
                steps = EXCLUDED.steps,
                active_calories = EXCLUDED.active_calories,
                resting_heart_rate = COALESCE(EXCLUDED.resting_heart_rate, daily_sleep.resting_heart_rate)
            "#,
            target_date, total_s, deep_s, light_s, rem_s, awake_s, score, steps, calories, rhr
        )
        .execute(pool)
        .await;
    }
}

pub async fn sync_garmin_data(pool: &PgPool) {
    let token = match get_garmin_token().await {
        Some(t) => t,
        None => return,
    };

    let req_client = reqwest::Client::new();
    let url = "https://connectapi.garmin.com/activitylist-service/activities/search/activities?start=0&limit=50";

    let response = req_client
        .get(url)
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36")
        .header("di-backend", "connectapi.garmin.com")
        .send()
        .await;

    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                if let Ok(activities) = resp.json::<serde_json::Value>().await {
                    if let Some(arr) = activities.as_array() {
                        let mut weekly_aggregation: std::collections::HashMap<(i32, u32), (f64, f64, f64)> = std::collections::HashMap::new();

                        for act in arr {
                            if let Some(time_str) = act["startTimeLocal"].as_str() {
                                if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(time_str, "%Y-%m-%d %H:%M:%S") {
                                    let year = dt.iso_week().year();
                                    let week = dt.iso_week().week();

                                    let distance = act["distance"].as_f64().unwrap_or(0.0);
                                    let type_key = act["activityType"]["typeKey"].as_str().unwrap_or("");

                                    let entry = weekly_aggregation.entry((year, week)).or_insert((0.0, 0.0, 0.0));

                                    if type_key.contains("running") {
                                        entry.0 += distance;
                                    } else if type_key.contains("cycling") {
                                        entry.1 += distance;
                                    } else if type_key.contains("swimming") {
                                        entry.2 += distance;
                                    }
                                }
                            }
                        }

                        for ((year, week), (run_m, cycle_m, swim_m)) in weekly_aggregation {
                            let running_km = run_m / 1000.0;
                            let cycling_km = cycle_m / 1000.0;

                            let _ = sqlx::query!(
                                r#"
                                INSERT INTO weekly_stats (year, week, running_km, cycling_km, swimming_m)
                                VALUES ($1, $2, $3, $4, $5)
                                ON CONFLICT (year, week) DO UPDATE SET
                                    running_km = EXCLUDED.running_km,
                                    cycling_km = EXCLUDED.cycling_km,
                                    swimming_m = EXCLUDED.swimming_m
                                "#,
                                year, week as i32, running_km, cycling_km, swim_m
                            )
                            .execute(pool)
                            .await;
                        }
                    }
                }
            }
        },
        Err(e) => println!("Request error to Garmin API: {}", e),
    }
}
