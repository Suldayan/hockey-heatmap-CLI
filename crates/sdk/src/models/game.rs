use serde::{Deserialize, Serialize};
use super::{PlayEvent, RosterSpot};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GameData {
    pub id: i64,
    pub season: i32,
    pub game_type: i32,
    pub game_date: String,
    pub plays: Vec<PlayEvent>,
    #[serde(default)]
    pub roster_spots: Vec<RosterSpot>,
}