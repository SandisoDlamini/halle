#![allow(unused)]
use crate::AppState;
use axum::{
    body,
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};
use std::sync::Arc;

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

pub fn routes(State(state): State<Arc<AppState>>, Path(id): Path<i32>) -> Router {
    Router::new()
        .route("/users", get(fetch_users))
        .route(
            format!("/users/{id}/articles").as_str(),
            get(fetch_user_articles),
        )
        .route(
            format!("/users/{id}/articles").as_str(),
            post(create_user_articles),
        )
        .with_state(state)
}

pub async fn fetch_users(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let result = sqlx::query_as::<_, User>("SELECT id, first_name, last_name FROM users")
        .fetch_all(&state.conn)
        .await
        .unwrap_or_else(|_| panic!("fetching user failed"));

    Json(result)
}

pub async fn fetch_user_articles(Path(id): Path<i32>) -> impl IntoResponse {
    format!("GET /users/{id}/articles")
}

pub async fn create_user_articles(
    Path(id): Path<i32>,
    body: Json<CreateArticleBody>,
) -> impl IntoResponse {
    format!("POST /users/{id}/articles")
}
