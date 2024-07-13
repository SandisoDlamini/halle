use axum::{response::Html, routing::get, Router};
use lazy_static::lazy_static;
use tera::Tera;
use tracing::debug;

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
                    println!("Template parsing error(s): {}", e);
                    ::std::process::exit(1);
                }
            };

            Html(page_content)
        }
    };
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

pub fn routes() -> Router {
    Router::new()
        .route("/", get(home))
        .route("/articles", get(articles))
        .route("/articles/article1", get(article1))
        .route("/articles/article2", get(article2))
        .route("/history", get(history))
        .route("/blog", get(blog))
        .route("/places", get(places))
        .route("/interests", get(interests))
        .route("/events", get(events))
        .route("/about", get(about))
}
