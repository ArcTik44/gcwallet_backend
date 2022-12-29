use actix_web::{Responder, get, web::{Data, Path}, HttpResponse};
use mongodb::{Client, Collection, bson::doc};

use crate::model::model::Card;
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