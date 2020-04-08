use crate::db;
use crate::schema::{matches, matches_users, users};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
// https://stackoverflow.com/questions/38676229/timestamp-in-rusts-diesel-library-with-postgres
use chrono::NaiveDateTime;

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
}

#[derive(Queryable, Identifiable, Associations, Serialize, Deserialize)]
#[belongs_to(User)]
#[belongs_to(Match)]
#[table_name = "matches_users"]
pub struct MatchesUsers {
    pub id: i32,
    pub match_id: i32,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct MatchWithUsers {
    pub id: i32,
    pub name: String,
    pub players_count: i32,
    pub users: Vec<User>,
}

#[derive(Serialize, Deserialize, RustcEncodable)]
pub struct Message {
    pub name: String,
}

impl Match {
    pub fn get(id: i32) -> Match {
        let connection = db::establish_connection();

        matches::table
            .find(id)
            .first(&connection)
            .expect("Could not load match")
    }

    pub fn create(mat: MatchAttr) -> Match {
        let connection = db::establish_connection();

        diesel::insert_into(matches::table)
            .values(&mat)
            .get_result::<Match>(&connection)
            .expect("Error saving new match")
    }

    pub fn join(match_id: i32, user_id: i32) -> MatchesUsers {
        let connection = db::establish_connection();

        diesel::insert_into(matches_users::table)
            .values((
                matches_users::columns::match_id.eq(&match_id),
                matches_users::columns::user_id.eq(&user_id),
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
