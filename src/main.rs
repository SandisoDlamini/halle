use axum::{response::Html, routing::get, Router};
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

    let articles_router = Router::new()
        .route("/articles", get(articles))
        .route("/articles/article1", get(load_article1_page));

    let app = Router::new()
        .route("/", get(load_home_page))
        .nest("/api", api_router)
        .merge(articles_router)
        .route("/history", get(load_history_page))
        .route("/blog", get(load_blog_page))
        .route("/places", get(load_places_page))
        .route("/interests", get(load_interests_page))
        .route("/events", get(load_events_page))
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

create_page_function!(articles, "articles.html");

async fn load_events_page() -> Html<String> {
    debug!("{:<12} - app: loading events page...", "HANDLER");

    let context1 = tera::Context::new();
    let page_content = match TEMPLATES.render("events.html", &context1) {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    Html(page_content)
}

async fn load_home_page() -> Html<String> {
    debug!("{:<12} - app: loading main page...", "HANDLER");

    let context1 = tera::Context::new();
    let page_content = match TEMPLATES.render("home.html", &context1) {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    Html(page_content)
}

async fn load_article1_page() -> Html<String> {
    debug!("{:<12} - app: loading article1 page...", "HANDLER");

    let context1 = tera::Context::new();
    let page_content = match TEMPLATES.render("article1.html", &context1) {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    Html(page_content)
}

async fn load_history_page() -> Html<String> {
    debug!("{:<12} - app: loading history page...", "HANDLER");

    let context1 = tera::Context::new();
    let page_content = match TEMPLATES.render("history.html", &context1) {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    Html(page_content)
}

async fn load_blog_page() -> Html<String> {
    debug!("{:<12} - app: loading blog page...", "HANDLER");

    let context1 = tera::Context::new();
    let page_content = match TEMPLATES.render("blog.html", &context1) {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    Html(page_content)
}

async fn load_places_page() -> Html<String> {
    debug!("{:<12} - app: loading places page...", "HANDLER");

    let context1 = tera::Context::new();
    let page_content = match TEMPLATES.render("places.html", &context1) {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    Html(page_content)
}

async fn load_interests_page() -> Html<String> {
    debug!("{:<12} - app: loading interests page...", "HANDLER");

    let context1 = tera::Context::new();
    let page_content = match TEMPLATES.render("interests.html", &context1) {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    Html(page_content)
}

async fn load_about_page() -> Html<String> {
    debug!("{:<12} - app: loading about page...", "HANDLER");

    let context1 = tera::Context::new();
    let page_content = match TEMPLATES.render("about.html", &context1) {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };

    Html(page_content)
}
// Region: --- HANDLER functions
