use serde::{Serialize, Deserialize};
use crate::schema::{matches, houses};

#[derive(Queryable, Associations, Serialize, Deserialize)]
#[belongs_to(Match)]
#[table_name = "houses"]
pub struct House {
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
