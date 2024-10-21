
use diesel::prelude::*;
use diesel::{SqliteConnection, QueryResult};
use crate::models::{Rusteceans, NewRusteceans}; 
use crate::schema::rustaceans;

pub struct RusteceanRepository; //unit-liked struct

impl RusteceanRepository {
    pub fn find(c: &mut SqliteConnection, id: i32) -> QueryResult<Rusteceans> {
        rustaceans::table.find(id).get_result::<Rusteceans>(c)
    }

    pub fn get_multiple(c: &mut SqliteConnection, limit: i64) -> QueryResult<Vec<Rusteceans>> {
        rustaceans::table.limit(limit).load::<Rusteceans>(c)
    }
    
    pub fn create(c: &mut SqliteConnection, data: NewRusteceans) -> QueryResult<Rusteceans> {
        diesel::insert_into(rustaceans::table)
            .values(data)
            .execute(c)?;

        let last_id:i32 = Self::last_inserted_id(c)?; 
        Self::find(c, last_id)
    }

    fn last_inserted_id(c: &mut SqliteConnection) -> QueryResult<i32> {
        rustaceans::table.select(rustaceans::id)
            .order(rustaceans::id.desc())
            .first(c)
    }

    pub fn update(c: &mut SqliteConnection, id: i32, update_rusteceans: NewRusteceans) -> QueryResult<Rusteceans> {
        diesel::update(rustaceans::table.find(id))
            .set((
                rustaceans::email.eq(update_rusteceans.email.to_owned()), // create owned data from string
                rustaceans::name.eq(update_rusteceans.name.to_owned())
            ))
            .execute(c)?;

        Self::find(c, id)
    }

    pub fn delete(c:&mut SqliteConnection, id:i32) -> QueryResult<usize> {
        diesel::delete(rustaceans::table.find(id))
            .execute(c)
    }
}
