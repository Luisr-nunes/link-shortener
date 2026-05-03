use axum::{http::StatusCode, response::IntoResponse, Json};
use crate::models::CreateLinkRequest;

pub async fn shorten_link(Json(payload): Json<CreateLinkRequest>) -> impl IntoResponse {
    
    let _ = payload;

    (StatusCode::OK, "Encurtador de links está funcionando!")
}