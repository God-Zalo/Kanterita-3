use serde::{Serialize, Deserialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Serialize)]
pub struct Status {
	pub status: String
}

#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table="db_people")]
pub struct DbPeople {
	pub id: i32,
	pub identificacion: String,
	pub nombre: String,
	pub genero: String,
	pub estado_civil: String,
	pub fecha_nacimiento: String,
	pub numero_telefono: i64,
	pub direccion: String,
	pub correo: String,
	pub validado: bool

}