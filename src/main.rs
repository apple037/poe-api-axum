use axum::{
    routing::{get, post}, Router,
};
use std::io;
use std::net::SocketAddr;
use tracing::Level;

mod handlers;
mod models;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .with_writer(io::stdout)
        .init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `handlers::root`
        .route("/", get(handlers::root))
        // `GET /hb` goes to `handlers::hb`
        .route("/hb", get(handlers::hb))
        // `POST /users` goes to `handlers::create_user`
        .route("/users", post(handlers::create_user));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
