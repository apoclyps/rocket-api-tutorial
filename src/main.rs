#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket_sync_db_pools;

mod auth;
mod models;
mod repositories;
mod schema;

use auth::BasicAuth;
use models::{Hero, NewHero};
use rocket::response::status;
use rocket::serde::json::{json, Json, Value};

use crate::repositories::HeroesRepository;

#[database("sqlite")]
struct DBConn(diesel::SqliteConnection);

#[get("/")]
fn index() -> Value {
    json!("Weclome to Hero API")
}

#[get("/heros")]
async fn get_heros(_auth: BasicAuth, db: DBConn) -> Value {
    db.run(|c| {
        let result = HeroesRepository::all(c, 100).expect("Failed to read Rustacean entries");

        json!(result)
    })
    .await
}

#[get("/heros/<id>")]
async fn view_hero(id: i32, _auth: BasicAuth, db: DBConn) -> Value {
    db.run(move |c| {
        let result = HeroesRepository::find(c, id).expect("Failed to read Hero row");
        json!(result)
    })
    .await
}

#[post("/heros", format = "json", data = "<new_hero>")]
async fn create_hero(_auth: BasicAuth, db: DBConn, new_hero: Json<NewHero>) -> Value {
    db.run(|c| {
        let result: Hero =
            HeroesRepository::create(c, new_hero.into_inner()).expect("Failed to create new hero");

        json!(result)
    })
    .await
}

#[put("/heros/<id>", format = "json", data = "<hero>")]
async fn update_hero(id: i32, _auth: BasicAuth, db: DBConn, hero: Json<Hero>) -> Value {
    db.run(move |c| {
        let result =
            HeroesRepository::save(c, id, hero.into_inner()).expect("Failed to update hero");

        json!(result)
    })
    .await
}

#[delete("/heros/<id>")]
async fn delete_hero(id: i32, _auth: BasicAuth, db: DBConn) -> status::NoContent {
    db.run(move |c| {
        HeroesRepository::delete(c, id).expect("Failed to delete hero");

        status::NoContent
    })
    .await
}

#[catch(401)]
fn unauthorized() -> Value {
    json!({"status": "401", "reason": "Unauthorized"})
}

#[catch(404)]
fn not_found() -> Value {
    json!({"status": "404", "reason": "Not Found"})
}

#[catch(422)]
fn unprocessable() -> Value {
    json!("Invalid entity. required fields are missing")
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
        .register("/", catchers![not_found, unprocessable, unauthorized])
        .attach(DBConn::fairing())
        .launch()
        .await;
}
