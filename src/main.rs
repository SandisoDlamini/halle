use std::error::Error;

use axum::{response::Html, routing::get, Router};
use lazy_static::lazy_static;
use tera::Tera;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing::{debug, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

const POSTGRES_URL: &str = env!("POSTGRES_URL");

struct Article {
    pub title: String,
    pub date_of_publication: String,
    pub category: String,
    pub description: String,
    pub content: String,
}

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

macro_rules! create_page_function {
    ($func_name:ident, $expression:expr) => {
        async fn $func_name() -> Html<String> {
            debug!(
                "{:<12} - app: loading {} page...",
                "HANDLER",
                stringify!($func_name)
            );

            let context1 = tera::Context::new();
            let template_name = $expression;
            let page_content = match TEMPLATES.render(template_name, &context1) {
                Ok(t) => t,
                Err(e) => {
                    println!("Parsing error(s): {}", e);
                    ::std::process::exit(1);
                }
            };

            Html(page_content)
        }
    };
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let conn = sqlx::postgres::PgPool::connect(POSTGRES_URL).await.unwrap();

    match sqlx::migrate!("./migrations").run(&conn).await {
        Ok(t) => t,
        Err(e) => {
            println!("Migration error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    let article = Article {
        title: "Salem's Lot".to_string(),
        date_of_publication: "2024-05-09".to_string(),
        category: "Horror".to_string(),
        description: "An interesting Horror story.".to_string(),
        content: "It all stated".to_string(),
    };

    match create_article(&article, &conn).await {
        Ok(t) => t,
        Err(e) => {
            println!("Database creation error(s): {}", e);
            ::std::process::exit(1);
        }
    }

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

    let articles_router = Router::new()
        .route("/articles", get(articles))
        .route("/articles/article1", get(article1))
        .route("/articles/article2", get(article2));

    let app = Router::new()
        .route("/", get(home))
        .nest("/api", api_router)
        .merge(articles_router)
        .route("/history", get(history))
        .route("/blog", get(blog))
        .route("/places", get(places))
        .route("/interests", get(interests))
        .route("/events", get(events))
        .route("/about", get(about))
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

async fn create_article(article: &Article, conn: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
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

create_page_function!(home, "home.html");
create_page_function!(places, "places.html");
create_page_function!(interests, "interests.html");
create_page_function!(articles, "articles.html");
create_page_function!(article1, "article1.html");
create_page_function!(history, "history.html");
create_page_function!(events, "events.html");
create_page_function!(blog, "blog.html");
create_page_function!(about, "about.html");
create_page_function!(article2, "article2.html");
// endregion: --- HANDLER functions
