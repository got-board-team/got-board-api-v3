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

    for qr in &query_result {
        let mut current_match: Option<MatchWithUsers> =
            response.into_iter().find(|m| m.id == qr.0.id);

        match current_match {
            Some(existing_match) => {
                println!("Inser user into match: {}", existing_match.name);
            }
            None => {
                println!("No existing match. Add to response.");
                let mut user = qr.1.unwrap();
                response.push(MatchWithUsers {
                    id: qr.0.id,
                    name: qr.0.name.clone(),
                    players_count: qr.0.players_count,
                    users: vec![User {
                        id: user.id,
                        name: user.name,
                        match_id: user.match_id,
                    }],
                });
            }
        }
    }

    response

    // match mat {
    //     Some(match_found) => {
    //         println!("Found!");
    //     }
    //     None => {
    //         println!("None.");
    //         &response.push((
    //             Match {
    //                 id: 1,
    //                 name: String::from("My first match"),
    //                 players_count: 3,
    //             },
    //             Some(User {
    //                 id: 1,
    //                 name: String::from("Rafael"),
    //                 match_id: 1,
    //             }),
    //         ));
    //     }
    // }

    //     // match &qr.1 {
    //     //     Some(match_user) => {
    //     //         current_match.users.push(User {
    //     //             id: match_user.id,
    //     //             name: match_user.name.clone(),
    //     //             match_id: match_user.match_id,
    //     //         });
    //     //     }
    //     //     None => println!("No user for this match."),
    //     // }

    //     // response.push(m);
    // }
}
