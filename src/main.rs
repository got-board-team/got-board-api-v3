#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate pusher;
extern crate rustc_serialize;

pub mod db;
pub mod models;
pub mod schema;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use dotenv::dotenv;
use models::{Match, MatchAttr, Message};
use pusher::Pusher;
use rocket_contrib::json::{Json, JsonValue};

#[get("/matches")]
fn all() -> JsonValue {
    json!(Match::all())
}

#[get("/matches/<id>")]
fn get(id: i32) -> JsonValue {
    json!(Match::get(id))
}

#[post("/matches", format = "json", data = "<mat>")]
fn create(mat: Json<MatchAttr>) -> JsonValue {
    let match_attributes = MatchAttr { ..mat.into_inner() };
    json!(Match::create(match_attributes))
}

#[put("/matches/<id>", format = "json", data = "<mat>")]
fn update(id: i32, mat: Json<MatchAttr>) -> JsonValue {
    let match_attributes = MatchAttr { ..mat.into_inner() };
    json!(Match::update(id, match_attributes))
}

#[delete("/matches/<id>")]
fn delete(id: i32) -> JsonValue {
    json!({ "success": Match::delete(id) })
}

#[post("/messages", format = "json", data = "<message>")]
fn pusher_message(message: Json<Message>) -> JsonValue {
    dotenv().ok();
    let api_id = dotenv::var("API_ID").expect("API_ID is not loaded");
    let key = dotenv::var("KEY").expect("Pusher KEY not set");
    let app_secret = dotenv::var("APP_SECRET").expect("Pusher APP_SECRET not set");
    let mut pusher = Pusher::new(&api_id, &key, &app_secret).finalize();
    let msg = Message {
        ..message.into_inner()
    };
    match pusher.trigger("game", "update", &msg) {
        Ok(_) => json!({ "success": &msg }),
        Err(error) => json!({ "error": error }),
    }
}

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![all, get, create, update, delete, pusher_message],
        )
        .launch();
}
