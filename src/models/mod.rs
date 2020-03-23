use crate::schema::{matches, users};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Queryable, Identifiable, Associations, Serialize, Deserialize)]
#[belongs_to(Match)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub name: String,
    pub match_id: i32,
}

#[derive(Queryable, Identifiable, Serialize, Deserialize)]
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
    pub users: HashMap<i32, User>,
}
