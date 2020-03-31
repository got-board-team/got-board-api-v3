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

use rocket_contrib::json::JsonValue;

#[get("/matches")]
fn read() -> JsonValue {
    json!(models::Match::all())
}

fn main() {
    rocket::ignite().mount("/", routes![read]).launch();
}
