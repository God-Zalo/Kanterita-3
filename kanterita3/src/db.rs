use crate::models::{DbPeople};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use std::io;

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


pub async fn create_person(client: &Client, identificacion: String, nombre: String, 
							genero: String, estado_civil: String,
							fecha_nacimiento: String, numero_telefono: i64,
							direccion: String, correo: String, validado: bool) -> Result<DbPeople, io::Error>{
	let statement = client.prepare("INSERT INTO db_people (identificacion, nombre, genero, estado_civil, fecha_nacimiento, numero_telefono, direccion, correo, validado) 
									VALUES ($1, $2, $3, $4, $5, $6, $7, $8 ,$9) returning id, identificacion, nombre").await.unwrap();

	client.query(&statement, &[&identificacion, &nombre, &genero, &estado_civil,
								&fecha_nacimiento, &numero_telefono, &direccion,
								&correo, &validado])
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