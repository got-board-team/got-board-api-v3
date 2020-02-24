#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket_contrib::json::Json;

mod models;
use models::{Match};

#[get("/matches")]
fn read() -> Json<Match> {
    Json(json!([
        "hero 1",
        "hero 2"
    ]))
}

fn main() {
    rocket::ignite()
        .mount("/matches", routes![read])
        .launch();
}
