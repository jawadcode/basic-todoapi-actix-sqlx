use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Result};

#[derive(Serialize, Deserialize)]
pub struct TodoInsert {
    pub title: String,
    pub description: String,
    pub completed: bool,
}

#[derive(Serialize, FromRow)]
pub struct Todo {
    pub id: i32,
    pub title: Option<String>,
    pub description: Option<String>,
    pub completed: Option<bool>,
}

#[derive(Deserialize)]
pub struct Query {
    pub query: Option<String>,
    pub limit: Option<i64>,
}

pub type Pool = PgPool;
pub type Todos = Vec<Todo>;

impl Todo {
    pub async fn insert(pool: &Pool, todo: TodoInsert) -> Result<Todo> {
        Ok(sqlx::query_as!(
            Todo,
            "INSERT INTO todos (title, description, completed)
                VALUES ($1, $2, $3) RETURNING id, title, description, completed",
            todo.title,
            todo.description,
            todo.completed,
        )
        .fetch_one(pool)
        .await?)
    }
    pub async fn get_all(pool: &Pool) -> Result<Todos> {
        Ok(sqlx::query_as!(
            Todo,
            "SELECT id, title, description, completed FROM todos
            ORDER BY id desc LIMIT 200"
        )
        .fetch_all(pool)
        .await?)
    }
    pub async fn get_by_id(pool: &Pool, id: i32) -> Result<Todo> {
        Ok(sqlx::query_as!(
            Todo,
            "SELECT id, title, description, completed FROM todos WHERE id=$1",
            id
        )
        .fetch_one(pool)
        .await?)
    }
    pub async fn update_by_id(pool: &Pool, id: i32, todo: TodoInsert) -> Result<Todo> {
        Ok(sqlx::query_as!(
            Todo,
            "UPDATE todos SET title=$1, description=$2, completed=$3 WHERE id=$4
            RETURNING id, title, description, completed",
            todo.title,
            todo.description,
            todo.completed,
            id
        )
        .fetch_one(pool)
        .await?)
    }
    pub async fn delete_by_id(pool: &Pool, id: i32) -> Result<i32> {
        Ok(
            sqlx::query!("DELETE FROM todos WHERE id=$1 RETURNING id", id,)
                .fetch_one(pool)
                .await?
                .id,
        )
    }
    pub async fn toggle_completed(pool: &Pool, id: i32) -> Result<Todo> {
        Ok(sqlx::query_as!(
            Todo,
            "UPDATE todos SET completed = NOT completed WHERE id=$1
            RETURNING id, title, description, completed",
            id,
        )
        .fetch_one(pool)
        .await?)
    }
    pub async fn filter_by_completed(pool: &Pool, completed: bool) -> Result<Todos> {
        let completed = if completed { true } else { false };

        Ok(sqlx::query_as!(
            Todo,
            "SELECT id, title, description, completed FROM todos 
            WHERE completed=$1 ORDER BY id DESC LIMIT 200",
            completed
        )
        .fetch_all(pool)
        .await?)
    }
    pub async fn search(pool: &Pool, query: String, limit: Option<i64>) -> Result<Todos> {
        let limit = limit.unwrap_or(200);

        Ok(sqlx::query_as!(
            Todo,
            "SELECT id, title, description, completed FROM todos
            WHERE tsv @@ plainto_tsquery($1) ORDER BY id DESC LIMIT $2",
            query,
            limit,
        )
        .fetch_all(pool)
        .await?)
    }
}
