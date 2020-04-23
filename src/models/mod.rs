use crate::db;
use crate::schema::{matches, matches_users, pieces, users};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
// https://stackoverflow.com/questions/38676229/timestamp-in-rusts-diesel-library-with-postgres
use chrono::NaiveDateTime;
use itertools::Itertools;
use serde_json;

#[derive(Insertable, Deserialize, AsChangeset)]
#[table_name = "users"]
pub struct UserAttr {
    pub email: String,
    pub name: String,
}

#[derive(Queryable, Identifiable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub email: String,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize, AsChangeset)]
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
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Identifiable, Associations, Serialize, Deserialize)]
#[belongs_to(User)]
#[belongs_to(Match)]
#[table_name = "matches_users"]
pub struct MatchesUsers {
    pub id: i32,
    pub match_id: i32,
    pub user_id: i32,
    pub house_name: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[table_name = "matches_users"]
pub struct MatchesUsersAttr {
    pub user_id: i32,
    pub house_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct MatchWithUsers {
    pub id: i32,
    pub name: String,
    pub players_count: i32,
    pub players: Vec<MatchesUsers>,
}

#[derive(Serialize, Deserialize, RustcEncodable)]
pub struct Message {
    pub name: String,
}

#[derive(Queryable, Identifiable, Serialize, Deserialize, Insertable)]
#[table_name = "pieces"]
pub struct Piece {
    pub id: i32,
    pub match_id: i32,
    pub piece_type: String,
    pub x: i32,
    pub y: i32,
    pub location: String,
    pub house_name: Option<String>,
    pub spec: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Insertable, AsChangeset)]
#[table_name = "pieces"]
pub struct PieceParams {
    pub piece_type: String,
    pub x: i32,
    pub y: i32,
    pub location: String,
    pub house_name: Option<String>,
    pub spec: Option<serde_json::Value>,
}

impl Match {
    pub fn get(id: i32) -> MatchWithUsers {
        let connection = db::establish_connection();

        let query_result: Vec<(Match, Option<MatchesUsers>)> = matches::table
            .left_join(matches_users::table.on(matches::id.eq(matches_users::match_id)))
            .filter(matches::id.eq(id))
            .load(&connection)
            .expect("Could not load matches with players");

        let response: _ = query_result
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
                    players: u
                        .into_iter()
                        .chain(g.flat_map(|(_, u)| u.into_iter()))
                        .collect(),
                }
            })
            .nth(0)
            .unwrap();

        response
    }

    pub fn create(mat: MatchAttr) -> Match {
        let connection = db::establish_connection();

        diesel::insert_into(matches::table)
            .values((
                &mat,
                matches::columns::created_at.eq(diesel::dsl::now),
                matches::columns::updated_at.eq(diesel::dsl::now),
            ))
            .get_result::<Match>(&connection)
            .expect("Error saving new match")
    }

    pub fn join(match_id: i32, user_id: i32, house_name: String) -> MatchesUsers {
        let connection = db::establish_connection();

        // Add business rule to restrict join to players_count cap and 1 player per house
        diesel::insert_into(matches_users::table)
            .values((
                matches_users::columns::match_id.eq(&match_id),
                matches_users::columns::user_id.eq(&user_id),
                matches_users::columns::house_name.eq(&house_name),
                matches_users::columns::created_at.eq(diesel::dsl::now),
            ))
            .get_result::<MatchesUsers>(&connection)
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

    pub fn all() -> Vec<Match> {
        let connection = db::establish_connection();

        matches::table
            .load::<Match>(&connection)
            .expect("Cound not load matches")
    }
}

impl User {
    pub fn all() -> Vec<User> {
        let connection = db::establish_connection();

        users::table
            .load::<User>(&connection)
            .expect("Cound not load users")
    }

    pub fn get(id: i32) -> User {
        let connection = db::establish_connection();

        users::table
            .find(id)
            .first(&connection)
            .expect("Could not load user")
    }

    pub fn create(user: UserAttr) -> User {
        let connection = db::establish_connection();

        diesel::insert_into(users::table)
            .values((
                &user,
                users::columns::created_at.eq(diesel::dsl::now),
                users::columns::updated_at.eq(diesel::dsl::now),
            ))
            .get_result::<User>(&connection)
            .expect("Error saving new user")
    }

    pub fn update(id: i32, user: UserAttr) -> bool {
        let connection = db::establish_connection();

        diesel::update(users::table.find(id))
            .set(&user)
            .execute(&connection)
            .is_ok()
    }

    pub fn delete(id: i32) -> bool {
        let connection = db::establish_connection();

        diesel::delete(users::table.find(id))
            .execute(&connection)
            .is_ok()
    }
}

impl Piece {
    pub fn create(match_id: i32, piece_attr: PieceParams) -> Piece {
        let connection = db::establish_connection();

        diesel::insert_into(pieces::table)
            .values((&piece_attr, pieces::columns::match_id.eq(&match_id)))
            .get_result::<Piece>(&connection)
            .expect("Error saving piece to match")
    }

    pub fn update(piece_id: i32, piece: PieceParams) -> bool {
        let connection = db::establish_connection();

        diesel::update(pieces::table.find(&piece_id))
            .set(&piece)
            .execute(&connection)
            .is_ok()
    }
}
