#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod db;
pub mod models;
pub mod schema;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use models::{Match, MatchAttr};
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

fn main() {
    rocket::ignite()
        .mount("/", routes![all, get, create, update, delete])
        .launch();
}
