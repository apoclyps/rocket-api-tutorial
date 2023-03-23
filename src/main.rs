#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket_sync_db_pools;

mod auth;
mod models;
mod schema;

use auth::BasicAuth;
use diesel::prelude::*;
use models::{Hero, NewHero};
use rocket::response::status;
use rocket::serde::json::{json, Json, Value};
use schema::heroes;

#[database("sqlite")]
struct DBConn(diesel::SqliteConnection);

#[get("/")]
fn index() -> Value {
    json!("Weclome to Hero API")
}

#[get("/heros")]
async fn get_heros(_auth: BasicAuth, db: DBConn) -> Value {
    db.run(|c| {
        let result = heroes::table
            .limit(100)
            .load::<Hero>(c)
            .expect("Failed to read Rustacean entries");
        json!(result)
    })
    .await
}

#[get("/heros/<id>")]
fn view_hero(id: i32, _auth: BasicAuth) -> Value {
    json!({"id": id, "name": "Clark Kent", "email": "clark.kent@dailyplanet.org"})
}

#[post("/heros", format = "json", data = "<new_hero>")]
async fn create_hero(_auth: BasicAuth, db: DBConn, new_hero: Json<NewHero>) -> Value {
    db.run(|c| {
        let result = diesel::insert_into(heroes::table)
            .values(new_hero.into_inner())
            .execute(c)
            .expect("Failed to create new hero");

        json!(result)
    })
    .await
}

#[put("/heros/<id>", format = "json")]
fn update_hero(id: i32, _auth: BasicAuth) -> Value {
    json!({"id": id, "name": "Clark Kent", "email": "clark.kent@dailyplanet.org"})
}

#[delete("/heros/<_id>")]
fn delete_hero(_id: i32, _auth: BasicAuth) -> status::NoContent {
    status::NoContent
}

#[catch(401)]
fn unauthorized() -> Value {
    json!({"status": "401", "reason": "Unauthorized"})
}

#[catch(404)]
fn not_found() -> Value {
    json!({"status": "404", "reason": "Not Found"})
}

#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            routes![
                index,
                get_heros,
                view_hero,
                create_hero,
                update_hero,
                delete_hero
            ],
        )
        .register("/", catchers![not_found, unauthorized])
        .attach(DBConn::fairing())
        .launch()
        .await;
}
