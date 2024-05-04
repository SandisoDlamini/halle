use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use tokio::net::TcpListener;
use lazy_static::{self, lazy_static};
use tera::Tera;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("examples/basic/templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![".html", ".sql"]);
        tera.register_filter("do_nothing", do_nothing_filter);
        tera
    };
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello_handler));

    // region: ---Start Server
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("->> LISTENING on {:?}\n", listener.local_addr().unwrap());
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
    // region: ---Start Server
}

// Region: ---HANDLER functions
async fn hello_handler() -> impl IntoResponse {
    println!("->> {:<12} - app - hello_handler", "HANDLER");

    Html("Hello Sphe")
}
// Region: --- HANDLER functions
