#![feature(core, io)]

// External stuff
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate protobuf;
extern crate rand;
extern crate zmq;

mod error;
mod shipit_protocol;

// Standard library
use std::result::Result;
use std::sync::{Arc,RwLock};

// Other libraries
use protobuf::core::Message;
use protobuf::error::ProtobufError;

use rand::Rng;

use zmq::Socket;

// Modules
use error::Error;

use shipit_protocol::{Request, Response, Error_Kind};

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
}

fn handle(i: u32, state_ref: &Arc<RwLock<GameState>>, req: &Request)
          -> Result<Response, Error> {
    let mut state = (*state_ref).write().unwrap();
    if req.has_identify() {
        let identify = req.get_identify();

        if player_by_name(&state.players, identify.get_name()).is_none() {
            info!("Connected: {}", identify.get_name());
            let token: String =
                rand::thread_rng().gen_ascii_chars().take(32).collect();

            state.players.push(Player {
                name: identify.get_name().to_string(),
                access_token: token.clone(),
            });

            let (major, minor, patch) = zmq::version();
            let info = format!("Authenticated by worker {}, ZMQ version {}.{}.{}",
                               i, major, minor, patch);

            let mut resp = Response::new();
            resp.mut_identified().set_access_token(token);
            resp.mut_identified().set_server_info(info.to_string());
            Ok(resp)
        } else {
            Ok(err_response(
                Error_Kind::PLAYER_NAME_TAKEN,
                &format!("Player {:?} already exists!", identify.get_name())))
        }
    } else {
        Err(Error::UnknownRequest)
    }
}

fn player_by_name<'a>(players: &'a [Player], name: &str) -> Option<&'a Player> {
    for ref player in players.iter() {
        if player.name.as_slice() == name {
            return Option::Some(player);
        }
    }
    Option::None
}

fn await(s: &mut Socket) -> Result<Request, Error> {
    let mut msg = try!(zmq::Message::new());
    try!(s.recv(&mut msg, 0));
    let req = try!(protobuf::parse_from_bytes::<Request>(msg.as_slice()));
    Ok(req)
}

fn respond(s: &mut Socket, resp: Response) -> Result<(), Error> {
    let bytes = try!(resp.write_to_bytes());
    try!(s.send(bytes.as_slice(), 0));
    Ok(())
}

fn await_and_handle(i: u32, state_ref: &Arc<RwLock<GameState>>, s: &mut Socket)
                    -> Result<Response, Error> {
    let req = try!(await(s));
    let resp = try!(handle(i, state_ref, &req));
    Ok(resp)
}

fn err_response(kind: shipit_protocol::Error_Kind, msg: &str) -> Response {
    let mut r = Response::new();
    r.mut_error().set_kind(kind);
    r.mut_error().set_msg(msg.to_string());
    r
}

fn run_worker(i: u32, state_ref: &Arc<RwLock<GameState>>, s: &mut Socket) {
    loop {
        let resp = match await_and_handle(i, state_ref, s) {
            Ok(r) => r,
            Err(Error::Protobuf(ProtobufError::WireError(msg))) =>
                err_response(Error_Kind::WIRE_ERROR, &msg),
            Err(Error::Protobuf(ProtobufError::IoError(e))) =>
                err_response(Error_Kind::IO_ERROR, e.description()),
            Err(Error::UnknownRequest) =>
                err_response(
                    Error_Kind::UNKNOWN_REQUEST,
                    "This server doesn't understand the request"),
            Err(Error::Unauthorized) =>
                err_response(
                    Error_Kind::UNAUTHORIZED,
                    "You are missing or using an invalid access_token!"),
            Err(Error::ZMQ(zmq::Error::ETERM)) => {
                error!("Context terminated! Worker {} shutting down", i);
                return ();
            },
            Err(e) => {
                error!("Worker {} crash, could not handle request: {}", i, e);
                return ();
            },
        };

        match respond(s, resp) {
            Err(e) => {
                error!("Worker {} crash, could not send reply: {}", i, e);
                return ();
            },
            Ok(()) => (),
        };
    }
}

const ADDRESS: &'static str = "tcp://*:1337";

fn run_server() -> Result<(), Error> {
    let mut ctx = zmq::Context::new();

    info!("Starting server");
    let state_ref = Arc::new(RwLock::new(GameState::new(1024f64, 1024f64)));

    debug!("Starting worker pool");
    let mut workers = try!(ctx.socket(zmq::DEALER));
    try!(workers.bind("inproc://workers"));

    for i in range(0, 8) {
        let mut worker = try!(ctx.socket(zmq::REP));
        let worker_state_ref = state_ref.clone();
        try!(worker.connect("inproc://workers"));
        try!(std::thread::Builder::new()
             .name(format!("worker-{}", i).to_string())
             .spawn(move || {
                 debug!("Starting worker {}", i);
                 run_worker(i, &worker_state_ref, &mut worker);
             }));
    }

    let mut clients = try!(ctx.socket(zmq::ROUTER));
    debug!("Connecting to the world");
    try!(clients.bind(ADDRESS));
    let supervisor =
        try!(std::thread::Builder::new()
             .name("supervisor".to_string())
             .scoped(move || {
                 debug!("Dispatching worker requests");
                 zmq::proxy(&mut clients, &mut workers).unwrap();
             }));

    info!("Server started on address {}", ADDRESS);
    supervisor.join();
    Ok(())
}

fn main() {
    env_logger::init().unwrap();
    match run_server() {
        Err(e) => error!("Server crashed: {}", e),
        Ok(()) => (),
    };
}
