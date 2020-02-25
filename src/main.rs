#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket_contrib::json::JsonValue;

mod models;
use models::{Match};
use models::{House};

mod match;

#[get("/matches")]
fn read() -> JsonValue {
    matches();
    let mock1: Vec<House> = vec!(
        House{
            name: "Lannister",
            player_id: 1
        },
        House{
            name: "Baratheon",
            player_id: 2
        },
    );

    let mock2: Vec<House> = vec!(
        House{
            name: "Lannister",
            player_id: 1
        }
    );

    json!([
        Match{
            id: 1,
            name: "My first match",
            players_count: 3,
            houses: mock1
        },
        Match{
            id: 2,
            name: "My second match",
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
