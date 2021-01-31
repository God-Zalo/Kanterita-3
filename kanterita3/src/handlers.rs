use crate::models::{Status, CreatePerson};
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


pub async fn create_person(db_pool: web::Data<Pool>, json: web::Json<CreatePerson>) -> impl Responder {
	let client: Client =
		db_pool.get().await.expect("Error connecting to the database");

	
	let result = db::create_person(&client, json.identificacion.clone(),
										 json.nombre.clone(),
										 json.genero.clone(),
										 json.estado_civil.clone(),
										 json.fecha_nacimiento.clone(),
										 json.numero_telefono.clone(),
										 json.direccion.clone(),
										 json.correo.clone(),
										 json.validado.clone()).await;

	match result {
		Ok(person) => HttpResponse::Ok().json(person),
		Err(_) => HttpResponse::InternalServerError().into()
	}


}


/*
TODO: Something not working properly on get a single person

pub async fn get_person(db_pool: web::Data<Pool>, path: web::Path<i32>) -> impl Responder{

	let client: Client =
		db_pool.get().await.expect("Error connecting to the database");

	
	let result = db::get_person(&client, path.0).await;

	match result {
		Ok(person) => HttpResponse::Ok().json(person),
		Err(_) => HttpResponse::InternalServerError().into()
	}
}
*/