//multipart
use std::io::Write;
use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};

//everything else
use crate::models::{Status,CreatePerson};
use crate::db;
use deadpool_postgres::{Pool, Client};
use actix_web::{web, Error, Responder, HttpResponse};


pub async fn status() -> impl Responder {
    web::HttpResponse::Ok()
        .json(Status { status: "Ok".to_string() })
}


pub async fn save_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_filename().unwrap();
        let filepath = format!("./tmp/{}", sanitize_filename::sanitize(&filename));

        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(filepath))
            .await
            .unwrap();

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&data).map(|_| f)).await?;
        }
    }
    Ok(web::HttpResponse::Ok().json(Status { status: "Ok".to_string()}))
}


pub async fn index() -> HttpResponse {
    let html = r#"<html>
        <head><title>Upload Test</title></head>
        <body>
            <form target="/" method="post" enctype="multipart/form-data" accept-charset=utf-8 >
                <input type="file" multiple name="file"/>
                <button type="submit">Submit</button>
            </form>
        </body>
    </html>"#;

    HttpResponse::Ok().body(html)
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

	
	let result = db::create_person(&client, json).await;

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