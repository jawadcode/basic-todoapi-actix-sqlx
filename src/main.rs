use actix_web::{middleware::Logger, web, App, HttpServer, Responder};
use anyhow::Result;
use pretty_env_logger;
use std::env;

mod db;
mod models;
mod routes;

fn get_env(name: &str) -> String {
    env::var(name).expect(&format!("{} environment variable not set", name))
}

async fn hello() -> impl Responder {
    "Hello there!"
}

#[actix_web::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    let address = get_env("ADDRESS");
    let db_pool = db::init().await?;

    HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .route("/", web::get().to(hello))
            .service(
                web::scope("/api")
                    .service(routes::post_todo)
                    .service(routes::search_todos)
                    .service(routes::get_todo)
                    .service(routes::get_todos)
                    .service(routes::patch_todo)
                    .service(routes::delete_todo)
                    .service(routes::toggle_todo)
                    .service(routes::filter_todos),
            )
            .wrap(Logger::default())
    })
    .bind(address)?
    .run()
    .await?;

    Ok(())
}
