/*
src/main.rs
*/

#[macro_use]
extern crate diesel;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

//dependencies
use std::{io,env};
use actix_web::{web, App, HttpServer};
use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;//EDITOR recommended

//module declaration
mod errors;
mod handlers;
mod models;
mod schema;

#[actix_rt::main]
async fn main() -> io::Result<()> {

    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // create db connection pool
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    // Start http server
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())//enables handlers connect independently
            .route("/users", web::get().to(handlers::get_users))
            .route("/users/{id}", web::get().to(handlers::get_user_by_id))
            .route("/users", web::post().to(handlers::add_user))
            .route("/users/{id}", web::delete().to(handlers::delete_user))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
