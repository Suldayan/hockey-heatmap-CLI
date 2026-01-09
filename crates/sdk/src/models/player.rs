use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RosterSpot {
    pub team_id: i32,
    pub player_id: i64,
    pub first_name: HashMap<String, String>,
    pub last_name: HashMap<String, String>,
    pub position_code: String,
}

impl RosterSpot {
    pub fn full_name(&self) -> String {
        format!(
            "{} {}",
            self.first_name.get("default").unwrap_or(&"".to_string()),
            self.last_name.get("default").unwrap_or(&"".to_string())
        )
    }
}