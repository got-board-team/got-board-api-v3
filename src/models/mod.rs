use crate::db;
use crate::schema::{matches, users};
use diesel::prelude::*;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Identifiable, Associations, Serialize, Deserialize)]
#[belongs_to(Match)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub name: String,
    pub match_id: i32,
}

#[derive(Deserialize, Insertable, AsChangeset)]
#[table_name = "matches"]
pub struct MatchAttr {
    pub name: String,
    pub players_count: i32,
}

#[derive(Queryable, Identifiable, Serialize, Deserialize, Insertable)]
#[table_name = "matches"]
pub struct Match {
    pub id: i32,
    pub name: String,
    pub players_count: i32,
}

#[derive(Serialize, Deserialize)]
pub struct MatchWithUsers {
    pub id: i32,
    pub name: String,
    pub players_count: i32,
    pub users: Vec<User>,
}

impl Match {
    pub fn get(id: i32) -> Match {
        let connection = db::establish_connection();

        matches::table
            .find(id)
            .first(&connection)
            .expect("Could not load user")
    }

    pub fn create(mat: MatchAttr) -> Match {
        let connection = db::establish_connection();

        diesel::insert_into(matches::table)
            .values(&mat)
            .get_result::<Match>(&connection)
            .expect("Error saving new match")
    }

    pub fn delete(id: i32) -> bool {
        let connection = db::establish_connection();

        diesel::delete(matches::table.find(id))
            .execute(&connection)
            .is_ok()
    }

    pub fn update(id: i32, mat: MatchAttr) -> bool {
        let connection = db::establish_connection();

        diesel::update(matches::table.find(id))
            .set(&mat)
            .execute(&connection)
            .is_ok()
    }

    pub fn all() -> Vec<MatchWithUsers> {
        let connection = db::establish_connection();

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
}
