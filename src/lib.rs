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

    let matches_vec = matches::table.load::<Match>(&connection).expect("error loading matches");

    let users_vec = User::belonging_to(&matches_vec)
        .load::<User>(&connection).expect("error loading users")
        .grouped_by(&matches_vec);
    let my_query = matches_vec.into_iter().zip(users_vec);
    let data = my_query.collect::<Vec<_>>();

    println!("Displaying {} matches", data.len());
    data
}

pub fn show_all_matches2() -> Vec<MatchWithUsers> {
    use schema::*;

    let connection = establish_connection();

    let query_result: Vec<(Match, Option<User>)> = matches::table
        .left_join(users::table.on(users::match_id.eq(matches::id)))
        .load(&connection)
        .expect("Error loading matches");

    let result = &query_result.into_iter().map(|qr| MatchWithUsers{
        id: qr.0.id,
        name: qr.0.name,
        players_count: qr.0.players_count,
        users: vec!(
            User{
                id: qr.1.id,
                name: qr.1.name,
                match_id: qr.1.match_id,
            }),
    }).collect();

    &result;
}
