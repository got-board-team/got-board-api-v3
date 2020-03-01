use serde::{Serialize, Deserialize};
use crate::schema::{matches, users};

#[derive(Queryable, Associations, Serialize, Deserialize)]
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
