#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use itertools::Itertools;

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

    let response: Vec<_> = query_result
        .into_iter()
        // Note that this assumes that `query_result` is sorted by match id since
        // `group_by` only considers consecutive matches.
        .group_by(|(m, _)| m.id)
        .into_iter()
        .map(|(id, mut g)| {
            // Now `g` is an iterator of `(Match, Option<User>)` where all the
            // matches are the same. We take the first item to get the match
            // information. Note that it is safe to unwrap here because `group_by`
            // would never call us with an empty `g`.
            let (m, u) = g.next().unwrap();
            MatchWithUsers {
                id: id,
                name: m.name,
                players_count: m.players_count,
                // We got the first user along with the match information, now
                // we append the other users from the remaining items in `g`.
                users: u
                    .into_iter()
                    .chain(g.flat_map(|(_, u)| u.into_iter()))
                    .collect(),
            }
        })
        .collect();

    response
}
