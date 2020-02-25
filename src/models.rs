#[derive(Serialize, Deserialize)]
pub struct House {
    pub name: &'static str,
    pub player_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Match {
    pub id: i32,
    pub name: &'static str,
    pub players_count: i32,
    pub houses: Vec<House>,
}
