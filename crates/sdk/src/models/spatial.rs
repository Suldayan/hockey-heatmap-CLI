#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

impl Coordinate {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    // Goalie position is considered at + or - 89
    pub fn distance_to_goal(&self, defending_side: &str) -> f64 {
        let goal_x = if defending_side == "left" { -89 } else { 89 };
        let dx = (goal_x - self.x) as f64;
        let dy = (0 - self.y) as f64;

        // Pythagorean Theorem to get the distance 
        (dx * dx + dy * dy).sqrt()
    }

    pub fn angle_to_goal(&self, defending_side: &str) -> f64 {
        let goal_x = if defending_side == "left" { -89 } else { 89 };
        let dx = (goal_x - self.x) as f64;
        let dy = (0 - self.y) as f64;
        dy.atan2(dx).to_degrees()
    }
}

#[derive(Debug, Clone)]
pub enum Zone {
    DefensiveZone,
    NeutralZone,
    OffensiveZone,
}

#[derive(Debug, Clone)]
pub struct ShotAttempt {
    pub coordinate: Coordinate,
    pub shot_type: String,
    pub player_id: i64,
    pub distance: f64,
    pub angle: f64,
    pub is_goal: bool,
}
