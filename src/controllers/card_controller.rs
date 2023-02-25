use actix_web::{Responder, post, web::{Data, Json, Path}, HttpResponse, get};
use mongodb::{Client, Collection, bson::{doc, oid::ObjectId}};
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

#[get("/api/cards/{id}")]
pub async fn card_with_id(data:Data<Client>,card_id:Path<String>)->impl Responder{
    let card_id = card_id.into_inner();
    let card_coll:Collection<Card> = data.database(MONGO_DB).collection(MONGO_COLLECTION);

    match card_coll.find_one(doc!{"_id":ObjectId::parse_str(&card_id).unwrap()}, None).await{
        Ok(Some(card))=>HttpResponse::Ok().json(card),
        Ok(None)=>HttpResponse::NotFound().body("This card does not exist"),
        Err(err)=>HttpResponse::InternalServerError().body(err.to_string())
    }
}