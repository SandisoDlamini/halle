use std::sync::Arc;

use axum::{routing::get, Router};
use sqlx::{postgres::PgPoolOptions, PgPool};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing::{debug, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod web;

const POSTGRES_URL: &str = env!("POSTGRES_URL");

struct AppState {
    conn: PgPool,
}

struct Article {
    pub title: String,
    pub date_of_publication: String,
    pub category: String,
    pub description: String,
    pub content: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = PgPoolOptions::new()
        .max_connections(5)
        .connect(POSTGRES_URL)
        .await?;

    sqlx::migrate!("./migrations").run(&conn).await?;

    let article = Article {
        title: "Salem's Lot".to_string(),
        date_of_publication: "2024-05-09".to_string(),
        category: "Horror".to_string(),
        description: "An interesting Horror story.".to_string(),
        content: "It all stated".to_string(),
    };

    create_article(&article, &conn).await?;

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "axum_webapp=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("initializing router...");

    // region: -- create router
    let assets_path = std::env::current_dir().unwrap();
    info!("loading assets from {:?}", &assets_path);

    let api_router = Router::new().route("/hello", get(htmx_hello));

    let state = AppState { conn };

    let app = Router::new()
        .nest("/api", api_router)
        .with_state(Arc::new(state))
        .merge(web::routes_handler::routes())
        .nest_service(
            "/assets",
            ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
        );
    // endregion: -- create router

    // region: ---Start Server
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    info!(
        "router initialized, now LISTENING on {:?}\n",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app.into_make_service()).await?;
    // endregion: ---Start Server

    Ok(())
}

// region: ---HANDLER functions
async fn htmx_hello() -> &'static str {
    debug!("{:<12} - app: loading hello api...", "HANDLER");

    "Hello from htmx!!"
}

async fn create_article(
    article: &Article,
    conn: &sqlx::PgPool,
) -> Result<(), Box<dyn std::error::Error>> {
    let query = "INSERT INTO articles 
        (title, date_of_publication, category, description, content) 
        VALUES ($1, $2, $3, $4, $5)";

    sqlx::query(query)
        .bind(&article.title)
        .bind(&article.date_of_publication)
        .bind(&article.category)
        .bind(&article.description)
        .bind(&article.content)
        .execute(conn)
        .await?;

    Ok(())
}
// endregion: --- HANDLER functions
