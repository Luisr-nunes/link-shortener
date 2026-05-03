use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
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
    
    let query = r#"
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

pub async fn redirect_link(
    State(pool): State<PgPool>,
    Path(short_code): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    
    let query = r#"
        UPDATE links
        SET clicks = clicks + 1
        WHERE short_code = $1
        RETURNING long_url
    "#;

    let result: Option<(String,)> = sqlx::query_as(query)
        .bind(&short_code.to_lowercase())
        .fetch_optional(&pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Erro ao acessar o banco de dados: {}", e),
            )
        })?;

    if let Some((long_url,)) = result {
        Ok(Redirect::to(&long_url))
    } else {
        Err((
            StatusCode::NOT_FOUND,
            "Link não encontrado ou expirado.".to_string(),
        ))
    }
}