mod project;

use actix_cors::Cors;
use actix_web::{http, web, App, HttpServer};
use dotenvy::{dotenv, var};
use mongodb::Client;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let mongo_value = var("MONGO_URI");
    if mongo_value.is_err() {
        panic!("Could not read the MongoDB connection String.")
    }

    let mongo_uri = mongo_value.unwrap();
    let client = Client::with_uri_str(mongo_uri)
        .await
        .expect("Failed to establish the database connection.");

    HttpServer::new(move || {
        let cors = Cors::default()
            .send_wildcard()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS", "PATCH"])
            .allowed_headers(vec![http::header::ACCEPT, http::header::CONTENT_TYPE]);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(client.clone()))
            .service(
                web::scope("/api/v1/project")
                    .service(project::controller::create_project)
                    .service(project::controller::update_project)
                    .service(project::controller::delete_project),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
