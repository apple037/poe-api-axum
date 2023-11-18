use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

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
    Json(payload): Json<CreateUserRequest>,
) -> (StatusCode, Json<UserDto>) {
    // insert your application logic here
    let now = Utc::now();
    let user = UserDto {
        name: payload.username,
        created_at: now.to_string(),
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

#[derive(Serialize)]
pub struct UserDto {
    pub(crate) name: String,
    pub(crate) created_at: String,
}

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub(crate) username: String,
}
