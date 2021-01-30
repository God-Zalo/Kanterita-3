use crate::models::{DbPeople};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use std::io;

pub async fn get_people(client: &Client) -> Result<Vec<DbPeople>, io::Error>{

	let statement = client.prepare("SELECT * FROM db_people").await.unwrap();

	let people = client.query(&statement, &[])
		.await
		.expect("Error getting people")
		.iter()
		.map(|row| DbPeople::from_row_ref(row).unwrap())
		.collect::<Vec<DbPeople>>();
	Ok(people)

}