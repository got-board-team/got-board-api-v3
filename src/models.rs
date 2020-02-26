use serde::{Serialize, Deserialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Match {
    pub id: i32,
    pub name: String,
    pub players_count: i32,
}
