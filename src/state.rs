use std::collections::HashMap;

use time::SteadyTime;

pub struct GameState {
    // Access token → player
    pub players: HashMap<String, Player>,

    pub width: f64,
    pub height: f64,
}

impl GameState {
    pub fn new(w: f64, h: f64) -> Self {
        GameState {
            players: HashMap::new(),

            width: w,
            height: h,
        }
    }
}

pub struct Player {
    pub name: String,
    pub last_seen: SteadyTime,

    pub x: f64,
    pub y: f64,

    pub direction: f64,
    pub ang_vel: f64,
}

impl Player {
    pub fn new(n: String) -> Self {
        Player {
            name: n,
            last_seen: SteadyTime::now(),

            x: 0.0,
            y: 0.0,

            direction: 0.0,
            ang_vel: 0.0,
        }
    }
}
