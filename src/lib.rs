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

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn show_all_matches() -> Vec<(Match, Vec<User>)> {
    use schema::matches;

    let connection = establish_connection();

    let matches_vec = matches::table
        .load::<Match>(&connection)
        .expect("error loading matches");

    let users_vec = User::belonging_to(&matches_vec)
        .load::<User>(&connection)
        .expect("error loading users")
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

    println!("Quert result: {}", &query_result.len());

    let mut response = Vec::new();

    for qr in &query_result {
        let matches = &response;
        let currentMatch = matches.into_iter().find(|mm| &mm.id == &qr.0.id);

        match currentMatch {
            Some(m) => {
                let u = match qr.1 {
                    Some(user) => m.users.push(User {
                        id: user.id,
                        name: user.name.clone(),
                        match_id: user.match_id,
                    }),
                    None => println!("No user."),
                };
            }
            None => {
                println!("No match.");
                response.push(MatchWithUsers {
                    id: qr.0.id,
                    name: qr.0.name.clone(),
                    players_count: qr.0.players_count,
                    users: match qr.1 {
                        Some(user) => vec![user],
                        None => Vec::new(),
                    },
                })
            }
        }
    }

    response
}
