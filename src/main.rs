#[macro_use]
extern crate rocket;

use rocket::response::status;
use rocket::serde::json::{json, Value};

#[get("/")]
fn index() -> Value {
    json!("Weclome to Hero API")
}

#[get("/heros")]
fn get_heros() -> Value {
    json!([{"id": 1, "name": "Clark Kent"}, {"id": 2, "name": "Bruce Wayne"}])
}

#[get("/heros/<id>")]
fn view_hero(id: i32) -> Value {
    json!({"id": id, "name": "Clark Kent", "email": "clark.kent@dailyplanet.org"})
}

#[post("/heros", format = "json")]
fn create_hero() -> Value {
    json!({"id": 3, "name": "John Doe", "email": "clark.kent@dailyplanet.org"})
}

#[put("/heros/<id>", format = "json")]
fn update_hero(id: i32) -> Value {
    json!({"id": id, "name": "Clark Kent", "email": "clark.kent@dailyplanet.org"})
}

#[delete("/heros/<_id>")]
fn delete_hero(_id: i32) -> status::NoContent {
    status::NoContent
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
        .register("/", catchers![not_found])
        .launch()
        .await;
}
