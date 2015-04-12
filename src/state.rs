use time::SteadyTime;

pub type Players = Vec<Player>;

#[derive(Debug)]
pub struct GameState {
    pub players: Players,

    pub width: f64,
    pub height: f64,
}

impl GameState {
    pub fn new(w: f64, h: f64) -> Self {
        GameState {
            players: Players::new(),

            width: w,
            height: h,
        }
    }
}

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub access_token: String,
    pub last_seen: SteadyTime,

    pub x: f64,
    pub y: f64,

    pub direction: f64,
    pub angular_velocity: f64,
}

impl Player {
    pub fn new(n: String, t: String) -> Self {
        Player {
            name: n,
            last_seen: SteadyTime::now(),
            access_token: t,

            x: 0.0,
            y: 0.0,

            direction: 0.0,
            angular_velocity: 0.0,
        }
    }
}
