mod config;
mod models;
mod handlers;
mod db;
mod file_handler;

use actix_web::{HttpServer, App, web};
use std::io;
use dotenv::dotenv;
use tokio_postgres::NoTls;
use crate::handlers::*;


#[actix_rt::main]
async fn main() -> io::Result<()> {

    std::fs::create_dir_all("./tmp").unwrap();

    dotenv().ok();

    let config = crate::config::Config::from_env().unwrap();

    let pool = config.pg.create_pool(NoTls).unwrap();

    println!("Starting server at http://{}:{}/", config.server.host, config.server.port);

    HttpServer::new( move || {

        App::new()
            .data(pool.clone())
            .route("/", web::get().to(index))
            .route("/", web::post().to(save_file))
            .route("/people{_:/?}", web::get().to(get_people))
            .route("/people{_:/?}", web::post().to(create_person))
            //.route("/people/{person_id}{_:/?}", web::get().to(get_person))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
