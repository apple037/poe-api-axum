// main.rs

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use std::io;
use std::net::SocketAddr;
use tracing::Level;

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
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/hb", get(hb))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

// Health check handler
async fn hb() -> impl IntoResponse {
    "Ok!"
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<models::CreateUserRequest>,
) -> (StatusCode, Json<models::UserDto>) {
    // insert your application logic here
    let user = models::UserDto::new(payload.username);

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}
