use actix_web::{Responder, get, web::Data, HttpResponse};
use mongodb::Client;
use crate::model::model::Gym;
use futures::stream::StreamExt;

const MONGO_DB: &'static str = "dev";
const MONGO_COLLECTION: &'static str = "gyms";

#[get("/api/gyms")]
pub async fn get_all_gyms(data: Data<Client>) -> impl Responder {
    let gym_collection = data.database(MONGO_DB).collection::<Gym>(MONGO_COLLECTION);
    match gym_collection.find(None, None).await {
        Ok(mut cursor) => {
            let mut gyms:Vec<Gym> = Vec::new();
            while let Some(result) = cursor.next().await {
                match result {
                    Ok(gym) => gyms.push(gym),
                    Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
                }
            }
            HttpResponse::Ok().json(gyms)
        },
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
