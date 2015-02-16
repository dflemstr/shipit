#![crate_name = "simple"]
#![feature(core)]

extern crate protobuf;
extern crate zmq;

use protobuf::core::{Message, parse_from_bytes};

use zmq::Socket;

mod shipit_protocol;
use shipit_protocol::{Request, Response};

fn send(s: &mut Socket, req: Request) -> Response {
    s.send(req.write_to_bytes().ok().unwrap().as_slice(), 0)
        .ok().unwrap();
    parse_from_bytes(&s.recv_bytes(0).ok().unwrap().as_slice())
        .ok().unwrap()
}

fn main() {
    println!("Connecting to server...");

    let mut context = zmq::Context::new();
    let mut s = context.socket(zmq::REQ).ok().unwrap();

    assert!(s.connect("tcp://localhost:1337").is_ok());

    let mut req = Request::new();
    req.mut_identify().set_name("dflemstr".to_string());

    let resp = send(&mut s, req);

    println!("Response: {:?}", resp);
}
