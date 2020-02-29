#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate diesel;

use rocket_contrib::json::JsonValue;
use got_board_api_v3::*;

#[get("/matches")]
fn read() -> JsonValue {
    json!(show_all_matches())
}

fn main() {
    rocket::ignite()
        .mount("/", routes![read])
        .launch();
}
