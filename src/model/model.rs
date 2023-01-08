
use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct User{
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub cards:Option<Vec<Card>>,
    pub email:Option<String>,
    pub password:Option<String>,
    pub username:Option<String>,
    pub user_type:u8
}
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct UserType;
impl UserType{
    pub const ADMIN:u8 = 1;
    pub const REG_USER:u8 = 2;
    pub const GYM_OWNER:u8 = 3;
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Gym{
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id:Option<ObjectId>,
    pub name:Option<String>,
    pub address:Option<String>,
    pub email:Option<String>
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Login{
    pub email:Option<String>,
    pub password: Option<String>
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Register{
    pub username:Option<String>,
    pub email: Option<String>,
    pub password:Option<String>,
    pub user_type: Option<u8>
}
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct UpdateData{
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub username:Option<String>,
    pub email:Option<String>,
    pub password:Option<String>
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Card{
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub gym: Option<Gym>,
    pub barcode_id:Option<String>,
    pub subscription: Option<SubscriptionType>
}
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct InputCard{
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub user_id:Option<ObjectId>,
    pub barcode_id:Option<String>
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum SubscriptionType{
    DateExpire(DateExpiration),
    Entries(Entries)
}
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct DateExpiration{
    date_expiration: String
}
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Entries{
    entries: u32
}