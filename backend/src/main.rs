use axum::{
    routing::get,
    Router,
    Json,
    extract::State,
};
use serde::Serialize;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use std::sync::Arc;
use chrono::{Datelike, Utc};

#[derive(Serialize)]
struct WeeklyStats {
    running_km: f64,
    cycling_km: f64,
    swimming_m: f64,
}

#[allow(dead_code)]
struct AppState {
    db: PgPool,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to database");

    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await.expect("Failed to run migrations");

    let shared_state = Arc::new(AppState { db: pool });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/stats/weekly", get(get_weekly_stats))
        .with_state(shared_state)
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_weekly_stats(State(_state): State<Arc<AppState>>) -> Json<WeeklyStats> {
    let now = Utc::now();
    let _year = now.year();
    let _week = now.iso_week().week() as i32;

    // TODO: fetch real garmin data and insert into db
    Json(WeeklyStats {
        running_km: 42.1,
        cycling_km: 154.2,
        swimming_m: 3200.0,
    })
}
