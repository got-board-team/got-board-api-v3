#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use models::*;
// use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn show_all_matches() {
    use schema::matches::dsl::*;

    println!("-- WANDERSOOOONN ---");

    let connection = establish_connection();
    let results = matches
        .limit(5)
        .load::<Match>(&connection)
        .expect("Error loading matches");

    println!("Displaying {} matches", results.len());
    for m in results {
        println!("{}", m.name);
        println!("{}", m.players_count);
        println!("----------\n");
    }
}
