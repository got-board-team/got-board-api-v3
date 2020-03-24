#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use models::*;
use std::collections::HashMap;
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

fn insert_user(user: &Option<User>, users: &mut HashMap<i32, User>) {
    match user {
        Some(existing_user_in_match) => {
            users.insert(
                existing_user_in_match.id,
                User {
                    id: existing_user_in_match.id,
                    name: existing_user_in_match.name.clone(),
                    match_id: existing_user_in_match.match_id,
                },
            );
        }
        None => println!("No user found for this match."),
    }
}

pub fn show_all_matches2() -> HashMap<i32, MatchWithUsers> {
    use schema::*;

    let connection = establish_connection();

    let query_result: Vec<(Match, Option<User>)> = matches::table
        .left_join(users::table.on(users::match_id.eq(matches::id)))
        .load(&connection)
        .expect("Error loading matches");

    let mut response: HashMap<i32, MatchWithUsers> = HashMap::new();

    for (m, u) in &query_result {
        match response.get(&m.id) {
            Some(existing_match) => insert_user(u, &mut existing_match.users),
            None => {
                let mut users: HashMap<i32, User> = HashMap::new();

                insert_user(u, &mut users);

                response.insert(
                    m.id,
                    MatchWithUsers {
                        id: m.id,
                        name: m.name.clone(),
                        players_count: m.players_count,
                        users: users,
                    },
                );
            }
        }
    }

    response
}
