use axum::{Router, extract::DefaultBodyLimit, http::StatusCode, routing::post};

mod flags;
mod hash;
mod routes;
mod storage;
mod sample;

// macro test
macro_rules! load_env {
    () => {
        load_env!(".env", ".env.local")
    };
    ($(
        $f:literal),+ // grabs 1 or more string literals (e.g. "hi")
        $(,)? // allows you to include an optional trailing comma
    ) => {
        $(
            dotenvy::from_path(concat!(env!("CARGO_MANIFEST_DIR"), "/../", $f)).ok();
        )+
    };
}

const BODY_LIMIT: usize = 100 * 1024 * 1024; // ~100mb (also = 100 << 20)

#[tokio::main]
async fn main() {
    load_env!();

    let addr = std::env::var("BACKEND_IP").unwrap_or_else(|_| "127.0.0.1".into());
    let port: u16 = std::env::var("BACKEND_PORT")
        .unwrap_or_else(|_| "3001".into())
        .parse()
        .expect("must be a number <3");

    let app = Router::new()
        .route("/upload", post(routes::upload::handle))
        .layer(DefaultBodyLimit::max(BODY_LIMIT));

    let listener = tokio::net::TcpListener::bind(format!("{addr}:{port}"))
        .await
        .unwrap();
    
    //#[cfg(debug_assertions)]
    println!("listening on {addr}:{port}");
    
    axum::serve(listener, app).await.unwrap();
}

//////////////////////////////////////

pub fn internal<E: ToString>(err: E) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
