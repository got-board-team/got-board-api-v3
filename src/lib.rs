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

pub fn show_all_matches() -> Vec<(Match, House)> {
    use schema::*;

    let connection = establish_connection();

    let results: Vec<(Match, House)> = matches::table
        .inner_join(houses::table.on(houses::match_id.eq(matches::id)))
        .load(&connection)
        .expect("Error loading matches");

    println!("Displaying {} matches", results.len());
    results
}
