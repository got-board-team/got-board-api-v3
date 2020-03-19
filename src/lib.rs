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

    let mut response: Vec<MatchWithUsers> = Vec::new();

    for (m, u) in &query_result {
        let existing_match = &response
            .into_iter()
            .find(|match_with_user| match_with_user.id == m.id);

        match existing_match {
            Some(mut found_match) => {
                println!("Inser user into match: {}", found_match.name);
                found_match.users.push(u.unwrap());
            }
            None => {
                println!("No existing match. Add to response.");
                let user = u.as_ref().unwrap();
                response.push(MatchWithUsers {
                    id: m.id,
                    name: m.name.clone(),
                    players_count: m.players_count,
                    users: vec![User {
                        id: user.id,
                        name: user.name.clone(),
                        match_id: user.match_id,
                    }],
                });
            }
        }
    }

    response
}
