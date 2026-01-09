use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlayEvent {
    pub event_id: i32,
    pub period_descriptor: PeriodDescriptor,
    pub time_in_period: String,
    pub situation_code: String,
    pub home_team_defending_side: String,
    pub type_code: i32,
    pub type_desc_key: String,
    #[serde(default)]
    pub details: Option<EventDetails>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PeriodDescriptor {
    pub number: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EventDetails {
    pub x_coord: Option<i32>,
    pub y_coord: Option<i32>,
    pub zone_code: Option<String>,
    pub shot_type: Option<String>,
    pub shooting_player_id: Option<i64>,
}