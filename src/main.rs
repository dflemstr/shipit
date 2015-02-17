#![feature(alloc, core, std_misc)]

extern crate protobuf;
extern crate zmq;

mod error;
mod shipit_protocol;

use std::boxed::BoxAny;
use std::option::Option;
use std::result::Result;
use std::rt::unwind::try;
use std::thread::Thread;

use protobuf::core::Message;
use protobuf::error::ProtobufError;

use zmq::Socket;

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
        println!("Connected: {}", req.get_identify().get_name());
        resp.mut_identified().set_access_token("abc123".to_string());

        let (major, minor, patch) = zmq::version();
        let info = format!("Served by worker {}, ZMQ version {}.{}.{}",
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
    unsafe {
        // This is the most annoying thing ever; the protobuf library
        // crashes on bad input.  We need to actually catch this error
        // using super unsafe code

        // If the try block doesn't set anything, this is the default
        let mut parsed = Option::None;

        // Isolate the execution
        let r = try(|| {
            parsed = Option::Some(
                protobuf::parse_from_bytes::<Request>(msg.as_slice())
                    .map_err(|e| Error::Protobuf(e)))
        });

        match r {
            Err(e) => {
                let m = e.downcast::<&str>().map(|b|*b)
                    .ok().unwrap_or("Unknown error").to_string();
                parsed = Option::Some(
                    Err(Error::Protobuf(ProtobufError::WireError(m))))
            },
            _ => {}
        }

        parsed.unwrap()
    }
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
            Err(Error::ZMQ(zmq::Error::ETERM)) => {
                println!("Context terminated! Worker {} shutting down", i);
                return ();
            },
            Err(e) => panic!("Could not handle request: {}", e),
        };

        match respond(s, resp) {
            Err(e) => panic!("Could not send reply to request: {}", e),
            Ok(()) => (),
        };
    }
}

fn run_server() -> Result<(), Error> {
    let mut ctx = zmq::Context::new();

    println!("Starting worker pool");
    let mut workers = try!(ctx.socket(zmq::DEALER));
    try!(workers.bind("inproc://workers"));

    for i in range(0, 8) {
        let mut worker = try!(ctx.socket(zmq::REP));
        println!("Starting worker {}", i);
        try!(worker.connect("inproc://workers"));
        std::thread::Builder::new()
            .name(format!("worker-{}", i).to_string())
            .spawn(move || {
                run_worker(i, &mut worker);
            });
    }

    let mut clients = try!(ctx.socket(zmq::ROUTER));
    println!("Connecting to the world");
    try!(clients.bind("tcp://*:1337"));
    let supervisor = std::thread::Builder::new()
        .name("supervisor".to_string())
        .scoped(move || {
            zmq::proxy(&mut clients, &mut workers);
        });

    println!("Server started");
    supervisor.join().ok().unwrap();
    Ok(())
}

fn main() {
    match run_server() {
        Err(e) => panic!("Server crashed: {}", e),
        Ok(()) => (),
    };
}
