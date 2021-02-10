use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};

use crate::models::*;

#[post("/todos")]
pub async fn post_todo(pool: web::Data<Pool>, body: web::Json<TodoInsert>) -> impl Responder {
    let todo = body.into_inner();

    match Todo::insert(&pool, todo).await {
        Ok(t) => HttpResponse::Ok().json(&t),
        _ => HttpResponse::InternalServerError().body("An Error Occurred while inserting the todo"),
    }
}

#[get("/todos")]
pub async fn get_todos(pool: web::Data<Pool>) -> impl Responder {
    match Todo::get_all(pool.get_ref()).await {
        Ok(t) => HttpResponse::Ok().json(&t),
        _ => HttpResponse::InternalServerError().body("An Error Occurred while getting all todos"),
    }
}

#[get("/todos/{id}")]
pub async fn get_todo(pool: web::Data<Pool>, id: web::Path<i32>) -> impl Responder {
    match Todo::get_by_id(&pool, id.into_inner()).await {
        Ok(t) => HttpResponse::Ok().json(&t),
        _ => HttpResponse::InternalServerError().body("Todo not found"),
    }
}

#[patch("/todos/{id}")]
pub async fn patch_todo(
    pool: web::Data<Pool>,
    id: web::Path<i32>,
    body: web::Json<TodoInsert>,
) -> impl Responder {
    let todo = body.into_inner();

    match Todo::update_by_id(&pool, id.into_inner(), todo).await {
        Ok(t) => HttpResponse::Ok().json(&t),
        _ => HttpResponse::InternalServerError().body("An Error Occurred while updating the todo"),
    }
}

#[delete("/todos/{id}")]
pub async fn delete_todo(pool: web::Data<Pool>, id: web::Path<i32>) -> impl Responder {
    match Todo::delete_by_id(&pool, id.into_inner()).await {
        Ok(i) => HttpResponse::Ok().body(format!("Deleted todo #{}", i)),
        _ => HttpResponse::InternalServerError()
            .body("An Error Occurred while attempting to deleting the todo"),
    }
}

#[patch("/todos/{id}/toggle")]
pub async fn toggle_todo(pool: web::Data<Pool>, id: web::Path<i32>) -> impl Responder {
    match Todo::toggle_completed(&pool, id.into_inner()).await {
        Ok(t) => HttpResponse::Ok().json(&t),
        _ => HttpResponse::InternalServerError()
            .body("An Error Occurred while attempting to toggle the todo"),
    }
}

#[get("/todos/filter/{filter}")]
pub async fn filter_todos(pool: web::Data<Pool>, filter: web::Path<String>) -> impl Responder {
    let completed = match filter.into_inner().as_str() {
        "completed" => true,
        "incomplete" => false,
        _ => return HttpResponse::BadRequest().body("Invalid filter"),
    };

    match Todo::filter_by_completed(&pool, completed).await {
        Ok(todos) => HttpResponse::Ok().json(&todos),
        _ => HttpResponse::InternalServerError().body("An Error Occured while getting the todos"),
    }
}

#[get("/todos/search")]
pub async fn search_todos(pool: web::Data<Pool>, query: web::Query<Query>) -> impl Responder {
    let query_string = match &query.query {
        Some(q) => String::from(q),
        None => return HttpResponse::BadRequest().body("The 'query' query parameter is required"),
    };

    match Todo::search(&pool, query_string, query.limit).await {
        Ok(todos) => HttpResponse::Ok().json(&todos),
        _ => {
            HttpResponse::InternalServerError().body("An Error Occurred while searching the todos")
        }
    }
}
