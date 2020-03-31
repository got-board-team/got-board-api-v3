#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate diesel;
extern crate dotenv;
extern crate rocket_contrib;

use rocket_contrib::json::JsonValue;
use serde_json::json;

#[get("/matches")]
fn read() -> JsonValue {
    json!(models::Match::all())
}

fn main() {
    rocket::ignite().mount("/", routes![read]).launch();
}
