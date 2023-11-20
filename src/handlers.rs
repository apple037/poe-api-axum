use axum::{
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use crate::models::{CreateUserRequest, UserDto};

pub async fn root() -> &'static str {
    "Hello, World!"
}

pub async fn hb() -> impl IntoResponse {
    "Ok!"
}

pub async fn create_user(Json(payload): Json<CreateUserRequest>) -> (StatusCode, Json<UserDto>) {
    // insert your application logic here
    let user = UserDto::new(payload.username);

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}
