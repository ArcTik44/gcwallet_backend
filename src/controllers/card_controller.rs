use actix_web::{Responder, get,post, web::{Data, Path, Json}, HttpResponse};
use mongodb::{Client, Collection, bson::doc};
use crate::model::model::{Card, InsertGymCard};
const MONGO_DB: &'static str = "dev";
const MONGOCOLLECTION: &'static str = "cards";

#[get("/api/cards/{barcode}")]
pub async fn card_by_id(data: Data<Client>,barcode:Path<String>)->impl Responder{
    let coll:Collection<Card> = data.database(MONGO_DB).collection(MONGOCOLLECTION);
    let result = coll.find_one(doc!{
        "barcode":(barcode.into_inner()).to_owned()
    }, None).await;
    
    match result{
        Ok(_card)=>HttpResponse::Ok().json(_card),
        Err(_)=>HttpResponse::NotFound().body("this card doesn't exist")
    }
}

#[post("/api/gym/newcard")]
pub async fn gym_new_card(data:Data<Client>,card_insert:Json<InsertGymCard>)->impl Responder{
    let card_coll:Collection<Card> = data.database(MONGO_DB).collection(MONGOCOLLECTION);
    let insert_data = Card{
        id: None,
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