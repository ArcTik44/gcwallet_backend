mod controllers;
mod model;

use controllers::user_controller;
use dotenv::dotenv;
use actix_web::{ App, HttpServer, get, HttpResponse, Responder, web};
use mongodb::Client;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json("Hello from rust and mongoDB")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    dotenv().ok();
    let mongo_uri = std::env::var("DB_URL")
    .unwrap_or_else(|_| "mongodb://localhost:27017".into());
    
    let client = Client::with_uri_str(mongo_uri).await.expect("No MongoDB connection key");
    HttpServer::new(move||{ 
        App::new()
        .app_data(web::Data::new(client.clone()))
        .service(hello)
        .service(user_controller::sign_in)
        .service(user_controller::sign_up)
        .service(user_controller::update_user)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
