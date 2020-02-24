#[derive(Serialize, Deserialize)]
pub struct House {
    pub name: String,
    pub playerId: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct Match {
    pub id: Option<i32>,
    pub name: String,
    pub playersCount: String,
    pub houses: [House],
}
