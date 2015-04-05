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
mod shipit_protocol;

// Standard library
use std::collections::HashMap;
use std::result::Result;
use std::thread;

// Other libraries
use protobuf::core::Message;
use protobuf::error::ProtobufError;

use rand::Rng;

use time::Duration;
use time::SteadyTime;

use zmq::Socket;

// Modules
use error::Error;

use shipit_protocol::{Request, Response, Error_Kind};

const ADDRESS: &'static str = "tcp://*:1337";

struct GameState {
    // Access token → player
    players: HashMap<String, Player>,

    width: f64,
    height: f64,
}

impl GameState {
    fn new(w: f64, h: f64) -> Self {
        GameState {
            players: HashMap::new(),

            width: w,
            height: h,
        }
    }
}

struct Player {
    name: String,
    last_seen: SteadyTime,
}

fn handle(req: &Request, state: &mut GameState) -> Result<Response, Error> {
    if req.has_identify() {
        handle_identify(req.get_identify(), state)
    } else {
        let token = try!(auth_player(state, req));

        if req.has_ping() {
            handle_ping(req.get_ping())
        } else if req.has_disconnect() {
            handle_disconnect(state, token)
        } else {
            Err(Error::UnknownRequest)
        }
    }
}

fn handle_identify(identify: &shipit_protocol::Identify,
                   state: &mut GameState) -> Result<Response, Error> {

    let name = identify.get_name();
    let is_new_player =
        state.players.values()
        .find(|p| p.name.as_slice() == name)
        .is_none();

    if is_new_player {
        info!("Connected: {:?}", name);
        let token: String =
            rand::thread_rng().gen_ascii_chars().take(16).collect();

        state.players.insert(token.clone(), Player {
            name: name.to_string(),
            last_seen: SteadyTime::now(),
        });

        let (major, minor, patch) = zmq::version();
        let info = format!("Authenticated, ZMQ version {}.{}.{}",
                           major, minor, patch);

        let mut resp = Response::new();
        resp.mut_identified().set_access_token(token);
        resp.mut_identified().set_server_info(info.to_string());
        Ok(resp)
    } else {
        Ok(err_response(
            Error_Kind::PLAYER_NAME_TAKEN,
            &format!("Player {:?} already exists!", name)))
    }
}

fn handle_ping(ping: &shipit_protocol::Ping) -> Result<Response, Error> {
    let mut resp = Response::new();
    let mut pong = shipit_protocol::Pong::new();

    if ping.has_payload() {
        pong.set_payload(ping.get_payload().to_vec());
    }
    resp.set_pong(pong);

    Ok(resp)
}

fn handle_disconnect(state: &mut GameState,
                     token: &str) -> Result<Response, Error> {
    state.players.remove(token);

    let mut resp = Response::new();
    resp.set_disconnected(shipit_protocol::Disconnected::new());
    Ok(resp)
}

fn handle_msg(msg: &zmq::Message, state: &mut GameState) -> Result<Response, Error> {
    let req = try!(protobuf::parse_from_bytes::<Request>(msg.as_slice()));
    let resp = try!(handle(&req, state));
    Ok(resp)
}

fn auth_player<'a>(state: &mut GameState, req: &'a Request) -> Result<&'a str, Error> {
    if req.has_access_token() {
        let token = req.get_access_token();
        match state.players.get_mut(token) {
            Option::Some(ref mut player) => {
                player.last_seen = SteadyTime::now();
                Ok(token)
            },
            Option::None => Err(Error::Unauthorized),
        }
    } else {
        Err(Error::Unauthorized)
    }
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

fn err_response(kind: shipit_protocol::Error_Kind, msg: &str) -> Response {
    let mut r = Response::new();
    r.mut_error().set_kind(kind);
    r.mut_error().set_msg(msg.to_string());
    r
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

fn poll_req(server: &mut Socket, msg: &mut zmq::Message, state: &mut GameState)
            -> Result<(), Error> {
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
                }
                Ok(())
            }

fn run_server() -> Result<(), Error> {
    let mut ctx = zmq::Context::new();

    info!("Starting server");

    let mut state = GameState::new(1024f64, 1024f64);
    let mut server = try!(ctx.socket(zmq::REP));
    try!(server.set_rcvtimeo(Option::Some(0)));

    try!(server.bind(ADDRESS));
    info!("Server started on address {}", ADDRESS);

    let inactivity_timeout = Duration::seconds(10);
    let mut msg = try!(zmq::Message::new());

    loop {
        let now = SteadyTime::now();
        evict_players(&mut state, &now, &inactivity_timeout);
        try!(poll_req(&mut server, &mut msg, &mut state));

        thread::sleep_ms(1);
    }
}

fn main() {
    env_logger::init().unwrap();
    run_server().unwrap();
}
