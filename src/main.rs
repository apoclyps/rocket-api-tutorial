#[macro_use]
extern crate rocket;
extern crate diesel;
#[macro_use]
extern crate rocket_sync_db_pools;

mod auth;

use auth::BasicAuth;
use rocket::response::status;
use rocket::serde::json::{json, Value};

#[database("sqlite")]
struct DBConn(diesel::SqliteConnection);


#[get("/")]
fn index() -> Value {
    json!("Weclome to Hero API")
}

#[get("/heros")]
fn get_heros(_auth: BasicAuth, _db: DBConn) -> Value {
    json!([{"id": 1, "name": "Clark Kent"}, {"id": 2, "name": "Bruce Wayne"}])
}

#[get("/heros/<id>")]
fn view_hero(id: i32, _auth: BasicAuth) -> Value {
    json!({"id": id, "name": "Clark Kent", "email": "clark.kent@dailyplanet.org"})
}

#[post("/heros", format = "json")]
fn create_hero(_auth: BasicAuth) -> Value {
    json!({"id": 3, "name": "John Doe", "email": "clark.kent@dailyplanet.org"})
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
