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
use rocket::http::Status;
use rocket::response::status::{self, Custom};
use rocket::serde::json::{json, Json, Value};

use crate::repositories::HeroesRepository;

#[database("sqlite")]
struct DBConn(diesel::SqliteConnection);

#[get("/")]
fn index() -> Value {
    json!("Weclome to Hero API")
}

#[get("/heros")]
async fn get_heros(_auth: BasicAuth, db: DBConn) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        HeroesRepository::all(c, 100)
            .map(|hero| json!(hero))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[get("/heros/<id>")]
async fn view_hero(id: i32, _auth: BasicAuth, db: DBConn) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        HeroesRepository::find(c, id)
            .map(|hero| json!(hero))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[post("/heros", format = "json", data = "<new_hero>")]
async fn create_hero(
    _auth: BasicAuth,
    db: DBConn,
    new_hero: Json<NewHero>,
) -> Result<Value, Custom<Value>> {
    db.run(|c| {
        HeroesRepository::create(c, new_hero.into_inner())
            .map(|hero| json!(hero))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[put("/heros/<id>", format = "json", data = "<hero>")]
async fn update_hero(
    id: i32,
    _auth: BasicAuth,
    db: DBConn,
    hero: Json<Hero>,
) -> Result<Value, Custom<Value>> {
    db.run(move |c| {
        HeroesRepository::save(c, id, hero.into_inner())
            .map(|hero| json!(hero))
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[delete("/heros/<id>")]
async fn delete_hero(
    id: i32,
    _auth: BasicAuth,
    db: DBConn,
) -> Result<status::NoContent, Custom<Value>> {
    db.run(move |c| {
        if HeroesRepository::find(c, id).is_err() {
            return Err(Custom(Status::NotFound, json!("${id} Hero Not Found")));
        }

        HeroesRepository::delete(c, id)
            .map(|_| status::NoContent)
            .map_err(|e| Custom(Status::InternalServerError, json!(e.to_string())))
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
