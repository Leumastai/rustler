
mod auth;
mod models;
mod schema;
mod repositories;

#[macro_use] extern crate  rocket;
extern crate diesel;
#[macro_use] extern crate rocket_sync_db_pools;
//#[macro_use] extern crate diesel_migrations; //this helps to startup db migrations everytime the app starts
//using r2d2 as the conection between rocket and diesel; literally a pool

use auth::BasicAuth; 
use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::{Rocket, Build};
use rocket::response::status::{self, Custom};
use rocket::serde::json::{Value, json, Json};
use models::NewRusteceans;
use repositories::RusteceanRepository;
// use serde_json::json; //for serializing adn de-serailizing rust data structure in different formats

#[database("sqlite_db_path")]
struct DBConn(diesel::SqliteConnection);

#[get("/")]
fn hello() -> Value {
    json!("Hello, World") // converts any rust data structure to json
}

#[get("/rusteceans")] // curl 127.0.0.1:8000/rusteceans -H 'Authorization: Basic QWxhZGRpbjpvcGVuIHNlc2FtZQ=='
async fn get_rusteceans(_auth: BasicAuth, db: DBConn) -> Result<Value, Custom<Value>> {
    // json!([{"id": 1, "name": "Robert Booby"}, {"id": 2, "name": "John Doe"}])
    db.run(|c| { //|c| is the callback that connects to the pool "db" and creates a connection with sqlite
        // let result: Vec<Rusteceans> = rustaceans::table.limit(100)
        //     .load::<Rusteceans>(c) //telling diesel to return the recods in Rustaceans format
        //     .expect("Failed to read Rusteceans entries"); //limit records to retrieve from the table and translate the returned records to rustaceans models
        RusteceanRepository::get_multiple(c, 100)
            .map(|rustaceans| json!(rustaceans))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    }).await
}

#[get("/rusteceans/<id>")]
async fn view_rusteceans(id: i32, _auth: BasicAuth, db:DBConn) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        //move create ownership for variable within the callback so they won't be availble after the closure of teh callback
        // Move is useful in asynchronous programming or when you need to transfer ownership of variables into the closure to ensure they remain valid even after the enclosing scope has exited.
        // By using move, you're explicitly stating that the closure should take ownership of any variables it captures, allowing you to move resources into the closure rather than borrowing them. 
        // This can help prevent lifetime issues and ensure the correctness of your code. Note all variable enclose within the closure won't be 
        // available after the closer is closed

        // all this is replace by the repository module
        // let result = rustaceans::table.find(id)
        //     .get_result::<Rusteceans>(c) //get_result converts its to rusteceans type which serializes to json
        //     .expect("Failed to find user with ID");
        RusteceanRepository::find(c, id)
            .map(|rustaceans| json!(rustaceans))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    }).await
}

#[post("/rusteceans", format = "json", data = "<new_rusteceans>")] // curl 127.0.0.1:8000/rusteceans -X POST -H 'Content-type: application/json'
async fn create_rusteceans(_auth: BasicAuth, db: DBConn, new_rusteceans: Json<NewRusteceans>) -> Result<Value, Custom<Value>>  {
    // json!({"id": 3, "name": "John Lark", "email": "john.lark007@gmail.com"})
    db.run(|c| { // |c| is the callback
        // usign diesel to insert data (.values()) into the rustaceans::table.
        // let result: usize = diesel::insert_into(rustaceans::table)
        //     .values(new_rusteceans.into_inner()) //into_inner wraps the json from new_rusteceans to NewRusteceans
        //     .execute(c)
        //     .expect("Failed inserting new rusteceans entry");

        RusteceanRepository::create(c, new_rusteceans.into_inner())
            .map(|rustaceans| json!(rustaceans))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    }).await
}

#[put("/rusteceans/<id>", format = "json", data = "<update_rusteceans>")]
async fn update_rusteceans(id: i32, _auth: BasicAuth, db: DBConn, update_rusteceans: Json<NewRusteceans>) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        // let result = diesel::update(rustaceans::table.find(id))
        //     .set((
        //         rustaceans::email.eq(update_rusteceans.email.to_owned()), // create owned data from string
        //         rustaceans::name.eq(update_rusteceans.name.to_owned())
        //     ))
        //     .execute(c)
        //     .expect("Failed updating rusteceans");
        // json!(result)
        RusteceanRepository::update(c, id, update_rusteceans.into_inner())
            .map(|rustaceans| json!(rustaceans))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    }).await
}

#[delete("/rusteceans/<id>")]
async fn delete_rusteceans(id: i32, _auth: BasicAuth, db:DBConn) -> status::NoContent { //prefix _id if its not beign used
    db.run(move |c| {
        // diesel::delete(rustaceans::table.find(id))  
        //     .execute(c)
        //     .expect("Failed deleting Rusteceans.");
        RusteceanRepository::delete(c, id)
            .expect("Failed deleting Rusteceans.");
        status::NoContent
    }).await
}


async fn run_db_migration(rocket: Rocket<Build>) -> Rocket<Build> {
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

    DBConn::get_one(&rocket)
        .await
        .expect("Failed to retrieve databaes connections")
        .run(|c| {
            c.run_pending_migrations(MIGRATIONS).expect(
                "Migrations failed");
        })
        .await;

    rocket
}

#[catch(404)] //for all 404 pages this is what will be called by rocket
fn not_found() -> Value {
    json!("Not Found")
}

#[rocket::main]
async  fn main() {
    let _ = rocket::build()
        .mount("/", routes![
            hello,
            get_rusteceans,
            view_rusteceans,
            create_rusteceans,
            update_rusteceans,
            delete_rusteceans])
        .register("/", catchers![ //define catchers in the macros
            not_found
        ])
        .attach(DBConn::fairing())
        .attach(AdHoc::on_ignite("Running DB migration", run_db_migration))
        .launch()
        .await;
}

// NOTE:: for production and testing checkout "/home/alijoe/Documents/rust_prods/"
// cargo build --release to optimize code for production
// run release at target/release/getting_started_apis
// in rust, the getting_started_apis binary file is all you need to
// ruyn your application, you don't need teh whole codebase. Which is sweet!

/* 
Once you have this binary file, you do not need the entire codebase to execute it in production. 
You only need the binary executable file along with any necessary configuration files or resources it depends on. You can copy this binary file to the production environment and run it independently without needing the source code or any development-related files.

*/