// External stuff
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate protobuf;
extern crate rand;
extern crate shipit;
extern crate time;
extern crate unicode_normalization;
extern crate zmq;

mod handler;
mod state;
mod settings;
mod util;

// Standard library
use std::result::Result;

// Other libraries
use time::Duration;
use time::SteadyTime;

// Modules
use shipit::comm;
use shipit::error::Error;
use settings::*;
use state::{GameState, Players};

fn evict_players(players: &mut Players,
                 now: &SteadyTime,
                 inactivity_timeout: &Duration) {
    players.retain(|ref p| *now - p.last_seen < *inactivity_timeout);
}

fn tick(state: &mut GameState, now: &SteadyTime, d: &Duration) {
    if *d == Duration::zero() {
        warn!("Tick of zero duration; the system timer is probably lo-res");
        return;
    }
    trace!("Tick at {}", now);

    let delta = d.num_nanoseconds().map_or(std::f64::MAX, |n| n as f64 / 1e9);

    for player in state.players.iter_mut() {
        player.direction =
            (player.direction + player.angular_velocity * delta)
            % (std::f64::consts::PI * 2.0);

        player.x =
            (player.x + SHIP_SPEED * player.direction.cos() * delta)
            % state.width;
        player.y =
            (player.y + SHIP_SPEED * player.direction.sin() * delta)
            % state.height;

        trace!("Player updated: {:?}", player);
    }
}

fn run_server() -> Result<(), Error> {
    let mut ctx = zmq::Context::new();

    info!("Starting server");

    let mut state = GameState::new(ARENA_WIDTH, ARENA_HEIGHT);
    let mut sender = comm::Sender::new();
    let mut receiver = comm::Receiver::new();
    let mut server = try!(ctx.socket(zmq::REP));
    try!(server.set_rcvhwm(RECEIVE_HWM));

    try!(server.bind(ADDRESS));
    info!("Server started on address {}", ADDRESS);

    let mut poll_items = [server.as_poll_item(zmq::POLLIN)];

    let inactivity_timeout = Duration::nanoseconds(INACTIVITY_TIMEOUT_NS);

    let mut last_tick = SteadyTime::now();
    loop {
        let now = SteadyTime::now();
        evict_players(&mut state.players, &now, &inactivity_timeout);

        try!(zmq::poll(&mut poll_items, 0));

        if (poll_items[0].get_revents() & zmq::POLLIN) != 0 {
            let req = try!(receiver.recv_request(&mut server));
            let resp = handler::handle(&req, &now, &mut state);
            try!(sender.send_response(&mut server, &resp));
        }

        tick(&mut state, &now, &(now - last_tick));
        last_tick = now;
    }
}

fn main() {
    env_logger::init().unwrap();
    run_server().unwrap();
}
