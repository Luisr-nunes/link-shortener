use axum::{ 
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use sqlx::PgPool;
use crate::models::{Link, CreateLinkRequest};
use rand::{distributions::Alphanumeric, Rng};

fn generate_short_code() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect()

}

pub async fn shorten_link(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateLinkRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {

    let short_code = generate_short_code().to_lowercase();

    let query = r#""
        INSERT INTO links (short_code, long_url, clicks)
        VALUES ($1, $2, 0)
        RETURNING id, short_code, long_url, clicks
    "#;

    let created_link: Link = sqlx::query_as(&query)
    .bind(&short_code)
    .bind(&payload.long_url)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        ( 
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Erro ao salvar no banco de dados: {}", e),
        )
    })?;

    Ok((StatusCode::CREATED, Json(created_link)))
}