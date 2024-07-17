use std::sync::Arc;

use axum::{
    body::Bytes,
    extract::{MatchedPath, State},
    http::{HeaderMap, Request},
    response::Response,
    routing::get,
    Router,
};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;
use tokio::net::TcpListener;
use tower_http::{classify::ServerErrorsFailureClass, services::ServeDir, trace::TraceLayer};
use tracing::{debug, info, info_span, Span};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod web;

#[derive(Clone)]
struct AppState {
    conn: PgPool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // region: -- tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "axum_webapp=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(|request: &Request<_>| {
            // Log the matched route's path (with placeholders not filled in).
            // Use request.uri() or OriginalUri if you want the real path.
            let matched_path = request
                .extensions()
                .get::<MatchedPath>()
                .map(MatchedPath::as_str);

            info_span!(
                "http_request",
                method = ?request.method(),
                matched_path,
                some_other_field = tracing::field::Empty,
            )
        })
        .on_request(|_request: &Request<_>, _span: &Span| {
            // You can use `_span.record("some_other_field", value)` in one of these
            // closures to attach a value to the initially empty field in the info_span
            // created above.
        })
        .on_response(|_response: &Response, _latency: Duration, _span: &Span| {
            // ...
        })
        .on_body_chunk(|_chunk: &Bytes, _latency: Duration, _span: &Span| {
            // ...
        })
        .on_eos(
            |_trailers: Option<&HeaderMap>, _stream_duration: Duration, _span: &Span| {
                // ...
            },
        )
        .on_failure(
            |_error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                // ...
            },
        );
    // endregion: -- tracing

    // region: -- database initialization
    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost".to_string());

    let conn = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_connection_str)
        .await?;

    sqlx::migrate!("./migrations").run(&conn).await?;

    let state = AppState { conn };
    info!("setting up postgres database...");
    // region: -- database initialization

    // region: -- create router

    info!("initializing router...");

    let assets_path = std::env::current_dir().unwrap();
    info!("loading assets from {:?}", &assets_path);

    let api_router = Router::new().route("/hello", get(htmx_hello));

    let app = Router::new()
        .nest("/api", api_router)
        .layer(trace_layer)
        .with_state(Arc::new(state.clone()))
        .merge(web::routes_handler::routes())
        .merge(web::init_db::routes(State(Arc::new(state.clone()))))
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

// endregion: --- HANDLER functions
