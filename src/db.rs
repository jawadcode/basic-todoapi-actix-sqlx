use crate::get_env;
use anyhow::Result;
use sqlx::postgres::PgPoolOptions;
use std::fs;

pub async fn init() -> Result<sqlx::PgPool> {
    // Create a DB Pool with a max of 4 connections
    let db_conn = get_env("DB_CONN");
    let db_pool = PgPoolOptions::new()
        .max_connections(6)
        .connect(&db_conn)
        .await?;
    // Read schema into string and then split by double newlines
    let schema = fs::read_to_string("schema.sql")?;
    for query in schema.split("\n\n") {
        sqlx::query(query).execute(&db_pool).await?;
    }
    // Return pool for use by the API
    Ok(db_pool)
}
