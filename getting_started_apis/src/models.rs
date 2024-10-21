use diesel::{prelude::Insertable, Queryable};
use super::schema::rustaceans;

#[derive(serde::Serialize, Queryable)] // we're serializing from NRusteaceans type to json
pub struct Rusteceans {
    pub id: i32,
    pub name: String, 
    pub email: String,
    pub created_at: String,
}

#[derive(serde::Deserialize, Insertable)] // we are deserializing from json to NewRusteceans datatype
#[diesel(table_name = rustaceans)]
pub struct NewRusteceans {
    pub name: String,
    pub email: String,
}