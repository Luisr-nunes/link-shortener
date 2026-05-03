use sqlx::postgress::PgPoolOptions;
use sqlx::PgPool;
use dotenvy::dotenv;
use std::env;

pub async fn estabilish_connection() -> result<PgPool, sqlx::Error> {

    dotenv().ok():

    let database_url = env::var("DATABASE_URL").expect("A variável DATABASE_URL deve ser definida no arquivo .env");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
}