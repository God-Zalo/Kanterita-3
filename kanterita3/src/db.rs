use crate::models::CreatePerson;
use crate::models::{DbPeople};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use std::io;
use actix_web::web;

pub async fn get_people(client: &Client) -> Result<Vec<DbPeople>, io::Error>{

	let statement = client.prepare("SELECT * FROM db_people ORDER BY id ASC").await.unwrap();

	let people = client.query(&statement, &[])
		.await
		.expect("Error getting people")
		.iter()
		.map(|row| DbPeople::from_row_ref(row).unwrap())
		.collect::<Vec<DbPeople>>();
	Ok(people)

}


pub async fn create_person(client: &Client, json: web::Json<CreatePerson>) -> Result<DbPeople, io::Error>{

	let statement = client.prepare("INSERT INTO db_people (identificacion, nombre, genero, estado_civil, fecha_nacimiento, numero_telefono, direccion, correo, validado, observacion) 
									VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) returning id, identificacion, nombre").await.unwrap();

	client.query(&statement, &[&json.identificacion, &json.nombre, &json.genero, &json.estado_civil,
								&json.fecha_nacimiento, &json.numero_telefono, &json.direccion,
								&json.correo, &json.validado, &json.observacion])
		.await
		.expect("Error creating person")
		.iter()
		.map(|row| DbPeople::from_row_ref(row).unwrap())
		.collect::<Vec<DbPeople>>()
		.pop()
		.ok_or(io::Error::new(io::ErrorKind::Other, "Error creating person"))
}


/*
TODO: Something not working properly on get a single person

pub async fn get_person(client: &Client, person_id: i32) -> Result<Vec<DbPeople>, io::Error>{

	let statement = client.prepare("SELECT * FROM db_people WHERE id = $1 ORDER BY id ASC").await.unwrap();

	let person = client.query(&statement, &[&person_id])
		.await
		.expect("Error getting person")
		.iter()
		.map(|row| DbPeople::from_row_ref(row).unwrap())
		.collect::<Vec<DbPeople>>();
	Ok(person)
}
*/