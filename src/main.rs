mod db;
mod handlers;
mod models;

use axum::{routing::post, Router};
use axum::routing::get;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {

    let pool = match db::estabilish_connection().await {
        Ok(pool) => {
            println!("Conexão com o banco de dados establecida com sucesso!");
            pool
        }
        Err(err) => {
            eprintln!("Falha ao connectar ao banco de dados: {}", err);
            return;
        }
    };

    let app = Router::new()
        .nest_service("/", ServeDir::new("static"))
        .route("/api/shorten", post(handlers::shorten_link))
        .route("/r/:short_code", get(handlers::redirect_link))
        .with_state(pool);


    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Servidor executando em http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}