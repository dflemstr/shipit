#![feature(core, convert)]

// External stuff
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate protobuf;
extern crate rand;
extern crate time;
extern crate unicode_normalization;
extern crate zmq;

mod error;
mod handler;
mod state;
mod protocol;
mod shipit_protocol;
mod settings;
mod util;

// Standard library
use std::result::Result;

// Other libraries
use protobuf::core::Message;
use protobuf::error::ProtobufError;

use time::Duration;
use time::SteadyTime;

use zmq::Socket;

// Modules
use error::Error;
use protocol::{Request, Response, Error_Kind};
use settings::*;
use state::{GameState, Players};
use util::err_response;

fn poll(s: &mut Socket, msg: &mut zmq::Message) -> Result<bool, Error> {
    match s.recv(msg, 0) {
        Ok(()) => Ok(true),
        Err(zmq::Error::EAGAIN) => Ok(false),
        Err(e) => Err(std::convert::From::from(e)),
    }
}

fn respond(s: &mut Socket, resp: Response) -> Result<(), Error> {
    let bytes = try!(resp.write_to_bytes());
    try!(s.send(bytes.as_slice(), 0));
    Ok(())
}

fn evict_players(players: &mut Players,
                 now: &SteadyTime,
                 inactivity_timeout: &Duration) {
    players.retain(|ref p| *now - p.last_seen < *inactivity_timeout);
}

fn poll_req(server: &mut Socket,
            msg: &mut zmq::Message,
            now: &SteadyTime,
            state: &mut GameState) -> Result<bool, Error> {
    if try!(poll(server, msg)) {
        try!(respond(server, handle_msg(msg, now, state)));
        Ok(true)
    } else {
        Ok(false)
    }
}

fn handle_msg(msg: &zmq::Message,
              now: &SteadyTime,
              state: &mut GameState) -> Response {
    match protobuf::parse_from_bytes::<Request>(msg.as_ref()) {
        Ok(req) => handler::handle(&req, now, state),
        Err(ProtobufError::WireError(msg)) =>
            err_response(Error_Kind::WIRE_ERROR, &msg),
        Err(ProtobufError::IoError(e)) =>
            err_response(Error_Kind::IO_ERROR,
                         std::error::Error::description(&e)),
    }
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
            % std::f64::consts::PI_2;

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
    let mut server = try!(ctx.socket(zmq::REP));
    try!(server.set_rcvhwm(RECEIVE_HWM));

    try!(server.bind(ADDRESS));
    info!("Server started on address {}", ADDRESS);

    let handle_msg_timeout = Duration::nanoseconds(HANDLE_MSG_TIMEOUT_NS);
    let inactivity_timeout = Duration::nanoseconds(INACTIVITY_TIMEOUT_NS);
    let mut msg = try!(zmq::Message::new());

    let mut last_tick = SteadyTime::now();
    loop {
        let now = SteadyTime::now();
        evict_players(&mut state.players, &now, &inactivity_timeout);

        let mut has_msg = true;
        while has_msg && SteadyTime::now() < now + handle_msg_timeout {
            has_msg = try!(poll_req(&mut server, &mut msg, &now, &mut state));
        }

        tick(&mut state, &now, &(now - last_tick));
        last_tick = now;
    }
}

fn main() {
    env_logger::init().unwrap();
    run_server().unwrap();
}
