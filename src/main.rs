#![feature(core)]

extern crate protobuf;
extern crate zmq;

mod shipit_protocol;

use protobuf::core::Message;
use protobuf::error::ProtobufError;

use zmq::Socket;

use shipit_protocol::{Request, Response};

struct GameState {
    players: Vec<Player>,

    width: f64,
    height: f64,
}

impl GameState {
    fn new(w: f64, h: f64) -> Self {
        GameState { players: Vec::new(), width: w, height: h }
    }
}

struct Player {
    name: String,
    access_token: String,

    life: f64,
    energy: f64,
    score: u32,

    momentum: f64,
    angular_momentum: f64,

    x: f64,
    y: f64,
}

fn await(s: &mut Socket) -> Result<Request, ProtobufError> {
    let mut msg = zmq::Message::new().ok().unwrap();
    s.recv(&mut msg, 0).ok().unwrap();
    protobuf::parse_from_bytes(msg.as_slice())
}

fn respond(s: &mut Socket, resp: Response) {
    s.send(resp.write_to_bytes().ok().unwrap().as_slice(), 0).ok().unwrap();
}

fn handle(req: &Request, resp: &mut Response, state: &mut GameState) {
    if req.has_identify() {
        println!("Connected: {}", req.get_identify().get_name());
        resp.mut_identified().set_access_token("abc123".to_string());

        let (major, minor, patch) = zmq::version();
        let info = format!("ZMQ version: {}.{}.{}", major, minor, patch);

        resp.mut_identified().set_server_info(info.to_string());
    } else {
        resp.mut_error()
            .set_kind(shipit_protocol::Error_Kind::UNKNOWN_REQUEST);
        resp.mut_error()
            .set_msg("This server doesn't support that request".to_string());
    }
}

fn main() {
    let mut ctx = zmq::Context::new();
    let mut s = ctx.socket(zmq::REP).ok().unwrap();

    assert!(s.bind("tcp://*:1337").is_ok());
    println!("Server started");

    let mut state = GameState::new(256f64, 256f64);

    loop {
        let mut resp = Response::new();

        match await(&mut s) {
            Ok(req) => handle(&req, &mut resp, &mut state),
            Err(ProtobufError::WireError(msg)) => {
                resp.mut_error().set_kind(
                    shipit_protocol::Error_Kind::WIRE_ERROR);
                resp.mut_error().set_msg(msg);
            },
            Err(ProtobufError::IoError(e)) => {
                resp.mut_error().set_kind(
                    shipit_protocol::Error_Kind::IO_ERROR);
                resp.mut_error().set_msg(e.to_string());
            },
        }

        respond(&mut s, resp);
    }
}
