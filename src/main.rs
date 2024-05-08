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

    let api_router = Router::new().route("/hello", get(htmx_hello));

    let app = Router::new()
        .nest("/api", api_router)
        .route("/home", get(load_home_page))
        .route("/articles", get(load_articles_page))
        .route("/explore", get(load_explore_page))
        .route("/blog", get(load_blog_page))
        .route("/portfolio", get(load_portfolio_page))
        .route("/interests", get(load_interests_page))
        .route("/settings", get(load_settings_page))
        .route("/about", get(load_about_page))
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
async fn htmx_hello() -> &'static str {
    debug!("{:<12} - app: loading hello api...", "HANDLER");

    "Hello from htmx!!"
}

async fn load_settings_page() -> impl IntoResponse {
    debug!("{:<12} - app: loading settings page...", "HANDLER");

    let context1 = tera::Context::new();
    let page_content = TEMPLATES.render("settings.html", &context1).unwrap();
    Html(page_content)
}

async fn load_home_page() -> impl IntoResponse {
    debug!("{:<12} - app: loading main page...", "HANDLER");

    let context1 = tera::Context::new();
    let page_content = TEMPLATES.render("main-page.html", &context1).unwrap();
    Html(page_content)
}

async fn load_articles_page() -> impl IntoResponse {
    debug!("{:<12} - app: loading articles page...", "HANDLER");

    let context1 = tera::Context::new();
    let page_content = TEMPLATES.render("articles.html", &context1).unwrap();
    Html(page_content)
}

async fn load_blog_page() -> impl IntoResponse {
    debug!("{:<12} - app: loading blog page...", "HANDLER");

    let context1 = tera::Context::new();
    let page_content = TEMPLATES.render("blog.html", &context1).unwrap();
    Html(page_content)
}

async fn load_explore_page() -> impl IntoResponse {
    debug!("{:<12} - app: loading explore page...", "HANDLER");

    let context1 = tera::Context::new();
    let page_content = TEMPLATES.render("explore.html", &context1).unwrap();
    Html(page_content)
}

async fn load_portfolio_page() -> impl IntoResponse {
    debug!("{:<12} - app: loading portfolio page...", "HANDLER");

    let context1 = tera::Context::new();
    let page_content = TEMPLATES.render("portfolio.html", &context1).unwrap();
    Html(page_content)
}

async fn load_interests_page() -> impl IntoResponse {
    debug!("{:<12} - app: loading interests page...", "HANDLER");

    let context1 = tera::Context::new();
    let page_content = TEMPLATES.render("interests.html", &context1).unwrap();
    Html(page_content)
}

async fn load_about_page() -> impl IntoResponse {
    debug!("{:<12} - app: loading about page...", "HANDLER");

    let context1 = tera::Context::new();
    let page_content = TEMPLATES.render("about.html", &context1).unwrap();
    Html(page_content)
}
// Region: --- HANDLER functions
