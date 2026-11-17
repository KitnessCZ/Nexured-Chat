use axum::{
    extract::{Query, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use chrono::Utc;
use nexured_shared::{CreateMessage, Message};
use serde::Deserialize;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::{env, net::SocketAddr};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::EnvFilter;

#[derive(Clone)]
struct AppState {
    db: SqlitePool,
}

#[derive(Debug, Deserialize)]
struct MessageQuery {
    limit: Option<i64>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let database_url = env::var("NEXURED_DATABASE_URL")
        .unwrap_or_else(|_| "sqlite://nexured.db?mode=rwc".to_string());

    let db = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS messages (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            sender TEXT NOT NULL,
            body TEXT NOT NULL,
            created_at TEXT NOT NULL
        )
        "#,
    )
    .execute(&db)
    .await?;

    let state = AppState { db };
    let app = Router::new()
        .route("/health", get(health))
        .route("/api/messages", get(list_messages).post(create_message))
        .with_state(state)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    let address: SocketAddr = env::var("NEXURED_BIND")
        .unwrap_or_else(|_| "0.0.0.0:8080".to_string())
        .parse()?;

    let listener = tokio::net::TcpListener::bind(address).await?;
    tracing::info!("Nexured API listening on http://{address}");
    axum::serve(listener, app).await?;
    Ok(())
}

async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "service": "nexured-api"
    }))
}

async fn list_messages(
    State(state): State<AppState>,
    Query(query): Query<MessageQuery>,
) -> Result<Json<Vec<Message>>, (StatusCode, String)> {
    let limit = query.limit.unwrap_or(50).clamp(1, 100);

    let rows = sqlx::query_as::<_, (i64, String, String, String)>(
        "SELECT id, sender, body, created_at FROM messages ORDER BY id DESC LIMIT ?",
    )
    .bind(limit)
    .fetch_all(&state.db)
    .await
    .map_err(internal_error)?;

    let messages = rows
        .into_iter()
        .rev()
        .map(|(id, sender, body, created_at)| Message {
            id,
            sender,
            body,
            created_at,
        })
        .collect();

    Ok(Json(messages))
}

async fn create_message(
    State(state): State<AppState>,
    Json(payload): Json<CreateMessage>,
) -> Result<(StatusCode, Json<Message>), (StatusCode, String)> {
    let sender = payload.sender.trim();
    let body = payload.body.trim();

    if sender.is_empty() || sender.chars().count() > 32 {
        return Err((
            StatusCode::BAD_REQUEST,
            "Sender must contain 1–32 characters".into(),
        ));
    }

    if body.is_empty() || body.chars().count() > 2000 {
        return Err((
            StatusCode::BAD_REQUEST,
            "Message must contain 1–2000 characters".into(),
        ));
    }

    let created_at = Utc::now().to_rfc3339();
    let result =
        sqlx::query("INSERT INTO messages (sender, body, created_at) VALUES (?, ?, ?)")
            .bind(sender)
            .bind(body)
            .bind(&created_at)
            .execute(&state.db)
            .await
            .map_err(internal_error)?;

    let message = Message {
        id: result.last_insert_rowid(),
        sender: sender.to_string(),
        body: body.to_string(),
        created_at,
    };

    Ok((StatusCode::CREATED, Json(message)))
}

fn internal_error(error: sqlx::Error) -> (StatusCode, String) {
    tracing::error!("database error: {error}");
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "Internal server error".into(),
    )
}
