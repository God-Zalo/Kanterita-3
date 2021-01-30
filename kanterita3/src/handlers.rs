use crate::models::Status;
use crate::db;
use deadpool_postgres::{Pool, Client};
use actix_web::{web, Responder, HttpResponse};


pub async fn status() -> impl Responder {
    web::HttpResponse::Ok()
        .json(Status { status: "Ok".to_string() })
}

pub async fn get_people(db_pool: web::Data<Pool>) -> impl Responder{

	let client: Client =
		db_pool.get().await.expect("Error connecting to the database");

	
	let result = db::get_people(&client).await;

	match result {
		Ok(people) => HttpResponse::Ok().json(people),
		Err(_) => HttpResponse::InternalServerError().into()
	}
}