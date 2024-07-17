use crate::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};
use std::sync::Arc;
use tracing::debug;

#[derive(Serialize, FromRow)]
struct User {
    id: i32,
    first_name: String,
    last_name: String,
}

#[derive(Serialize, FromRow)]
struct Article {
    id: i32,
    title: String,
    content: String,
    created_by: i32,
}

#[derive(Debug, Deserialize)]
pub struct CreateArticleBody {
    pub title: String,
    pub content: String,
}

pub fn routes(State(state): State<Arc<AppState>>) -> Router {
    Router::new()
        .route("/users", get(fetch_users))
        .route("/users/:id/articles", get(fetch_user_articles))
        .route("/users/:id/articles", post(create_user_articles))
        .with_state(state)
}

pub async fn fetch_users(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    debug!("{:<12} - app: fetching users...", "HANDLER");
    match sqlx::query_as::<_, User>("SELECT id, first_name, last_name FROM users")
        .fetch_all(&state.conn)
        .await
    {
        Ok(users) => (StatusCode::OK, Json(users)).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "Failed no users").into_response(),
    }
}

pub async fn fetch_user_articles(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    debug!(
        "{:<12} - app: fetching user articles for user {id:?}...",
        "HANDLER"
    );
    match sqlx::query_as::<_, Article>(
        "SELECT id, title, content, created_by FROM articles WHERE created_by = $1",
    )
    .bind(id)
    .fetch_all(&state.conn)
    .await
    {
        Ok(articles) => (StatusCode::OK, Json(articles)).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "Found no user articles").into_response(),
    }
}

pub async fn create_user_articles(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(body): Json<CreateArticleBody>,
) -> impl IntoResponse {
    debug!(
        "{:<12} - app: creating user article for user {id:?}...",
        "HANDLER"
    );
    match sqlx::query_as::<_, Article>(
        "INSERT INTO articles (title, content, created_by) VALUES ($1, $2, $3) RETURNING id, title, content, created_by"
    )
        .bind(body.title.to_string())
        .bind(body.content.to_string())
        .bind(id)
        .fetch_all(&state.conn)
        .await
    {
        Ok(article) => (StatusCode::OK, Json(article)).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create user article").into_response(),
    }
}
