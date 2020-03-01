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

pub fn show_all_matches() -> Vec<(Match, Vec<User>)> {
    use schema::matches;

    let connection = establish_connection();

    let matches = matches::table.load::<Match>(&connection).expect("error loading matches");

    let u = User::belonging_to(&matches)
        .load::<User>(&connection).expect("error loading users")
        .grouped_by(&matches);
    let data = matches.into_iter().zip(u).collect::<Vec<_>>();

    println!("Displaying {} matches", data.len());
    data
}
