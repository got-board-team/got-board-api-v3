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

#[post("/matches", format = "json", data = "<mat>")]
fn create(mat: Json<MatchAttr>) -> JsonValue {
    let insert = MatchAttr { ..mat.into_inner() };
    json!(Match::create(insert))
}

#[get("/matches")]
fn read() -> JsonValue {
    json!(Match::all())
}

fn main() {
    rocket::ignite().mount("/", routes![read, create]).launch();
}
