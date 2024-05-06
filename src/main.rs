use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use lazy_static::lazy_static;
use tera::Tera;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing::{debug, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![".html", ".sql"]);
        tera
    };
}

#[tokio::main]
async fn main() {
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
    let app = Router::new()
        .route("/", get(hello_handler))
        .route("/main-page", get(hello_handler2))
        .nest_service(
            "/assets",
            ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
        );
    // region: -- create router

    // region: ---Start Server
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    info!(
        "router initialized, now LISTENING on {:?}\n",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
    // region: ---Start Server
}

// Region: ---HANDLER functions
async fn hello_handler() -> impl IntoResponse {
    debug!("{:<12} - app - hello_handler", "HANDLER");

    let context1 = tera::Context::new();
    let page_content = TEMPLATES.render("index.html", &context1).unwrap();
    Html(page_content)
}

async fn hello_handler2() -> impl IntoResponse {
    debug!("{:<12} - app - hello_handler2", "HANDLER");

    let context1 = tera::Context::new();
    let page_content = TEMPLATES.render("main-page.html", &context1).unwrap();
    Html(page_content)
}
// Region: --- HANDLER functions
