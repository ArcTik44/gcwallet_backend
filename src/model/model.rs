use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};


#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct User{
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub cards:Vec<Option<Card>>,
    pub email:Option<String>,
    pub password:Option<String>,
    pub username:Option<String>,
    pub user_type: UserType
}
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum UserType{
   GymUser,
   User,
   Admin
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
pub struct InsertGymCard{
    pub user_type:UserType,
    pub barcode:Option<String>,
    pub gym: Option<ObjectId>,
    pub subscription: Option<SubscriptionType>
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
    pub user_type: Option<UserType>
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
    pub gym: Option<ObjectId>,
    pub barcode:Option<String>,
    pub subscription: Option<SubscriptionType>
}
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct InputCard{
    pub user_id:Option<ObjectId>,
    pub barcode:Option<String>,
    pub gym_id:Option<ObjectId>
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