extern crate diesel;
use diesel::prelude::*;

use models;

pub fn show_all_matches() {
    use got_board_api_v3::schema::matches::dsl::*;

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
