use bcrypt::{DEFAULT_COST,hash, verify};
use crate::model::model::{User, Login, Register, UpdateData, InputCard, UserType, Card};
use actix_web::{web::{Path, Json, Data}, HttpResponse, Responder, post, get, put};
use mongodb::{Client, bson::{doc, oid::ObjectId, Bson}, Collection};
const MONGO_DB: &'static str = "dev";
const MONGO_COLLECTION: &'static str = "users";
const MONGO_COLLECTION_CARDS: &'static str = "cards";

#[get("/api/users/{id}")]
pub async fn get_user_by_id(data: Data<Client>,user_id:Path<String>) -> impl Responder {
    let user_id = user_id.into_inner();
    let collection:Collection<User> = data.database(MONGO_DB).collection(MONGO_COLLECTION); 
    
    match collection
        .find_one(doc! {"_id": ObjectId::parse_str(&user_id).unwrap()}, None)
        .await
    {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => {
            HttpResponse::NotFound().body(format!("No user found with username {user_id}"))
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/api/users/signin")]
pub async fn sign_in(data: Data<Client>, user:Json<Login>) -> impl Responder{
    let collection:Collection<User> = data.database(MONGO_DB).collection(MONGO_COLLECTION);
    let user_find = collection.find_one(
        doc!{
            "email": user.email.to_owned()
        }, None).await;

    match user_find {
        Ok(Some(user_found))=>
        match verify(user.password.as_deref().map(|s| s.as_bytes()).unwrap_or(&[]),user_found.password.as_ref().unwrap().as_str()){
            Ok(true)=> HttpResponse::Ok().json(user_found),
            Ok(false)=>HttpResponse::NotFound().body(format!("Wrong Credentials")),
            Err(_)=>HttpResponse::InternalServerError().body(format!("Shit happened"))
        }
        Ok(None)=> HttpResponse::NotFound().body(format!("User does not exist")),
        Err(err)=> HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/api/users/signup")]
pub async fn sign_up(data: Data<Client>,user:Json<Register>)-> impl Responder{
    let hash_pass = hash(user.password.as_deref().map(|s| s.as_bytes()).unwrap_or(&[]),DEFAULT_COST).unwrap();
    let coll = data.database(MONGO_DB).collection(MONGO_COLLECTION);
    let data = User{
        _id:None,
        cards:Vec::new(),
        email:user.email.to_owned(),
        password:Some(hash_pass.to_owned()),
        username:user.username.to_owned(),
        user_type: UserType::User
    };
    let inserted = coll.insert_one(data,None).await;
    match inserted{
        Ok(_)=>HttpResponse::Ok().body("new user inserted"),
        Err(err)=>HttpResponse::InternalServerError().body(err.to_string())
    }
}
 
#[put("/api/users/update")]
pub async fn update_user(data: Data<Client>, user:Json<UpdateData>)-> impl Responder{
    let coll:Collection<User> = data.database(MONGO_DB).collection(MONGO_COLLECTION);
    
    let mut update_doc = doc! {};

    if let Some(email) = user.email.as_deref().filter(|&s| !s.is_empty()) {
        update_doc.insert("email", Bson::String(email.to_owned()));
    }

    if let Some(password) = user.password.as_deref().filter(|&s| !s.is_empty()) {
        let update_pass = hash(password.as_bytes(), DEFAULT_COST).unwrap();
        update_doc.insert("password", Bson::String(update_pass));
    }

    if let Some(username) = user.username.as_deref().filter(|&s| !s.is_empty()) {
        update_doc.insert("username", Bson::String(username.to_owned()));
    }

    let updated = coll
        .update_one(doc! {"_id":&user._id}, doc! {"$set": update_doc}, None)
        .await;

    match updated{
        Ok(_)=>HttpResponse::Ok().body("user update successful"),
        Err(err)=>HttpResponse::InternalServerError().body(err.to_string())
    }
}

#[post("/api/users/newcard")]
pub async fn user_new_card(data: Data<Client>, input_card:Json<InputCard>)->impl Responder{
    let coll_user:Collection<User> = data.database(MONGO_DB).collection(MONGO_COLLECTION);
    let coll_card:Collection<Card> = data.database(MONGO_DB).collection(MONGO_COLLECTION_CARDS);
    
    let find_card = match coll_card.find_one(doc! {
        "barcode": input_card.barcode.to_owned(),
        "gym": input_card.gym_id.to_owned()
    }, None).await {
        Ok(Some(card)) => card,
        Ok(None) => return HttpResponse::NotFound().body("This card does not exist"),
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    };

    let update_result = coll_user.update_one(doc! {
        "_id": input_card.user_id.to_owned(),
    }, doc! {
        "$push": {
            "cards": find_card._id.clone()
        }
    }, None).await;

    match update_result {
        Ok(_) => HttpResponse::Ok().body("Card has been connected to your account"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}