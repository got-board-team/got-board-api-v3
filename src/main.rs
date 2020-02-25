#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
extern crate diesel;

use rocket_contrib::json::JsonValue;

mod models;
use models::{Match};
use models::{House};

mod match_model;

#[get("/matches")]
fn read() -> JsonValue {
    match_model::show_all_matches();
    let mock1: Vec<House> = vec!(
        House{
            name: String::from("Lannister"),
            player_id: 1
        },
        House{
            name: String::from("Baratheon"),
            player_id: 2
        },
    );

    let mock2: Vec<House> = vec!(
        House{
            name: String::from("Lannister"),
            player_id: 1
        }
    );

    json!([
        Match{
            id: 1,
            name: String::from("My first match"),
            players_count: 3,
            houses: mock1
        },
        Match{
            id: 2,
            name: String::from("My second match"),
            players_count: 4,
            houses: mock2
        },
    ])
}

fn main() {
    rocket::ignite()
        .mount("/", routes![read])
        .launch();
}
