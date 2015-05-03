/// The version of shipit
pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");

/// The address that the server should listen on.
pub const ADDRESS: &'static str = "tcp://*:1337";

/// How many requests to buffer before stalling clients.
pub const RECEIVE_HWM: i32 = 16;

/// The time the server may spend handling incoming requests before
/// another simulation tick must happen.
pub const HANDLE_MSG_TIMEOUT_NS: i64 = 1_000_000;

/// The time it takes for a player to be evicted if that player hasn't
/// sent any requests during that time.
pub const INACTIVITY_TIMEOUT_NS: i64 = 10_000_000_000;

/// How wide the arena is in units.
pub const ARENA_WIDTH: f64 = 1024.0;

/// How tall the arena is in units.
pub const ARENA_HEIGHT: f64 = 1024.0;

/// How fast ships move in units per second.
pub const SHIP_SPEED: f64 = 128.0;

/// How far ships can see in units.
pub const SHIP_SCAN_DISTANCE: f64 = 100.0;
