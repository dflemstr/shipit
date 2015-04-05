#![feature(core, convert)]

// External stuff
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate protobuf;
extern crate rand;
extern crate time;
extern crate zmq;

mod error;
mod handler;
mod state;
mod protocol;
mod shipit_protocol;
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
use state::GameState;
use util::err_response;

const ADDRESS: &'static str = "tcp://*:1337";

fn handle_msg(msg: &zmq::Message, state: &mut GameState) -> Result<Response, Error> {
    let req = try!(protobuf::parse_from_bytes::<Request>(msg.as_slice()));
    let resp = try!(handler::handle(&req, state));
    Ok(resp)
}

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

fn evict_players(state: &mut GameState,
                 now: &SteadyTime,
                 inactivity_timeout: &Duration) {
    let mut to_evict = Vec::new();

    for (k, p) in state.players.iter() {
        if *now - p.last_seen >= *inactivity_timeout {
            info!("Evicting player {:?} due to {} timeout",
                  p.name, inactivity_timeout);
            to_evict.push(k.clone());
        }
    }

    for k in to_evict.iter() {
        state.players.remove(k);
    }
}

fn poll_req(server: &mut Socket, msg: &mut zmq::Message, state: &mut GameState) -> Result<bool, Error> {
    if try!(poll(server, msg)) {
        let resp = match handle_msg(msg, state) {
            Ok(r) => r,
            Err(Error::Protobuf(ProtobufError::WireError(msg))) =>
                err_response(Error_Kind::WIRE_ERROR, &msg),
            Err(Error::Protobuf(ProtobufError::IoError(e))) =>
                err_response(Error_Kind::IO_ERROR,
                             std::error::Error::description(&e)),
            Err(Error::UnknownRequest) =>
                err_response(
                    Error_Kind::UNKNOWN_REQUEST,
                    "This server doesn't understand the request"),
            Err(Error::Unauthorized) =>
                err_response(
                    Error_Kind::UNAUTHORIZED,
                    "You are missing or using an invalid access_token!"),
            Err(e) => {
                return Err(e);
            },
        };

        try!(respond(server, resp));
        Ok(true)
    } else {
        Ok(false)
    }
}

fn tick(state: &mut GameState, now: &SteadyTime, d: &Duration) {
    trace!("Tick, delta: {}", d);

    if *d == Duration::zero() {
        warn!("Tick of zero duration; the system timer is probably borked");
        return;
    }
}

fn run_server() -> Result<(), Error> {
    let mut ctx = zmq::Context::new();

    info!("Starting server");

    let mut state = GameState::new(1024f64, 1024f64);
    let mut server = try!(ctx.socket(zmq::REP));
    try!(server.set_rcvtimeo(Option::Some(0)));
    try!(server.set_sndhwm(16));
    try!(server.set_rcvhwm(16));

    try!(server.bind(ADDRESS));
    info!("Server started on address {}", ADDRESS);

    let handle_msg_timeout = Duration::milliseconds(1);
    let inactivity_timeout = Duration::seconds(10);
    let mut msg = try!(zmq::Message::new());

    let mut last_tick = SteadyTime::now();
    loop {
        let now = SteadyTime::now();
        evict_players(&mut state, &now, &inactivity_timeout);

        let mut has_msg = true;
        while has_msg && SteadyTime::now() < now + handle_msg_timeout {
            has_msg = try!(poll_req(&mut server, &mut msg, &mut state));
        }

        tick(&mut state, &now, &(now - last_tick));
        last_tick = now;
    }
}

fn main() {
    env_logger::init().unwrap();
    run_server().unwrap();
}
