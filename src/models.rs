#[derive(Serialize, Deserialize)]
pub struct House {
    pub name: String,
    pub player_id: i32,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct Match {
    pub id: i32,
    pub name: String,
    pub players_count: i32,
    pub houses: Vec<House>,
}
