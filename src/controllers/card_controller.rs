use actix_web::{Responder, get,post, web::{Data, Path, Json}, HttpResponse};
use mongodb::{Client, Collection, bson::doc};
use crate::model::model::{Card, InsertGymCard};
const MONGO_DB: &'static str = "dev";
const MONGO_COLLECTION: &'static str = "cards";

#[post("/api/gym/newcard")]
pub async fn gym_new_card(data:Data<Client>,card_insert:Json<InsertGymCard>)->impl Responder{
    let card_coll:Collection<Card> = data.database(MONGO_DB).collection(MONGO_COLLECTION);
    let insert_data = Card{
        _id: None,
        barcode: card_insert.barcode.to_owned(),
        gym: card_insert.gym.to_owned(),
        subscription: card_insert.subscription.to_owned()
    };
    let new_card = card_coll.insert_one(insert_data, None).await;
    match new_card{
        Ok(_)=>HttpResponse::Ok().body(format!("Card has been inserted")),
        Err(err)=>HttpResponse::InternalServerError().body(err.to_string())
    }
}