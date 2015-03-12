#![feature(core, io)]

// External stuff
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate protobuf;
extern crate zmq;

mod error;
mod shipit_protocol;

// Standard library
use std::result::Result;

// Other libraries
use protobuf::core::Message;
use protobuf::error::ProtobufError;

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

fn handle(i: u32, req: &Request) -> Result<Response, Error> {
    let mut resp = Response::new();

    if req.has_identify() {
        info!("Connected: {}", req.get_identify().get_name());
        resp.mut_identified().set_access_token("abc123".to_string());

        let (major, minor, patch) = zmq::version();
        let info = format!("Authenticated by worker {}, ZMQ version {}.{}.{}",
                           i, major, minor, patch);

        resp.mut_identified().set_server_info(info.to_string());
    } else {
        return Err(Error::UnknownRequest);
    }

    Ok(resp)
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

fn await_and_handle(i: u32, s: &mut Socket) -> Result<Response, Error> {
    let req = try!(await(s));
    let resp = try!(handle(i, &req));
    Ok(resp)
}

fn err_response(kind: shipit_protocol::Error_Kind, msg: String) -> Response {
    let mut r = Response::new();
    r.mut_error().set_kind(kind);
    r.mut_error().set_msg(msg);
    r
}

fn run_worker(i: u32, s: &mut Socket) {
    loop {
        let resp = match await_and_handle(i, s) {
            Ok(r) => r,
            Err(Error::Protobuf(ProtobufError::WireError(msg))) =>
                err_response(Error_Kind::WIRE_ERROR, msg),
            Err(Error::Protobuf(ProtobufError::IoError(e))) =>
                err_response(Error_Kind::IO_ERROR, e.to_string()),
            Err(Error::UnknownRequest) =>
                err_response(
                    Error_Kind::UNKNOWN_REQUEST,
                    "This server doesn't understand the request".to_string()),
            Err(Error::Unauthorized) =>
                err_response(
                    Error_Kind::UNAUTHORIZED,
                    "You are missing or using an invalid access_token!".to_string()),
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

    debug!("Starting worker pool");
    let mut workers = try!(ctx.socket(zmq::DEALER));
    try!(workers.bind("inproc://workers"));

    for i in range(0, 8) {
        let mut worker = try!(ctx.socket(zmq::REP));
        try!(worker.connect("inproc://workers"));
        try!(std::thread::Builder::new()
             .name(format!("worker-{}", i).to_string())
             .spawn(move || {
                 debug!("Starting worker {}", i);
                 run_worker(i, &mut worker);
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
