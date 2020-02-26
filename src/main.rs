#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate diesel;

use rocket_contrib::json::JsonValue;

use self::models::*;
use got_board_api_v3::*;

#[get("/matches")]
fn read() -> JsonValue {
    show_all_matches();

    json!([
        Match{
            id: 1,
            name: String::from("My first match"),
            players_count: 3,
        },
        Match{
            id: 2,
            name: String::from("My second match"),
            players_count: 4,
        },
    ])
}

fn main() {
    rocket::ignite()
        .mount("/", routes![read])
        .launch();
}
