#![crate_name = "simple"]
#![feature(core, convert)]

extern crate env_logger;
#[macro_use]
extern crate log;
extern crate protobuf;
extern crate zmq;

use protobuf::core::{Message, parse_from_bytes};

use zmq::Socket;

mod shipit_protocol;
use shipit_protocol::{Request, Response};

fn send(s: &mut Socket, req: Request) -> Response {
    s.send(req.write_to_bytes().ok().unwrap().as_slice(), 0)
        .ok().unwrap();
    parse_from_bytes(s.recv_bytes(0).ok().unwrap().as_slice())
        .ok().unwrap()
}

const ADDRESS: &'static str = "tcp://localhost:1337";

fn main() {
    env_logger::init().unwrap();
    info!("Connecting to server on address {}", ADDRESS);

    let mut context = zmq::Context::new();
    let mut s = context.socket(zmq::REQ).ok().unwrap();

    s.connect("tcp://localhost:1337").unwrap();
    info!("Connected to server");

    let mut req = Request::new();
    req.mut_identify().set_name("dflemstr".to_string());

    let resp = send(&mut s, req);
    println!("Response: {:?}", resp);
    let identified = resp.get_identified();

    let mut counter = 0;
    loop {
        let mut ping = Request::new();
        ping.set_access_token(identified.get_access_token().to_string());
        ping.mut_ping().set_payload(format!("{}", counter).into_bytes());
        counter = counter + 1;
        let pong = send(&mut s, ping);
        println!("Ping response: {:?}", pong);
    }
}
