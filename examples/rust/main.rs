#![crate_name = "simple"]

extern crate env_logger;
#[macro_use]
extern crate log;
extern crate protobuf;
extern crate shipit;
extern crate zmq;

use std::env;

use shipit::comm;
use shipit::protocol::Request;

const ADDRESS: &'static str = "tcp://localhost:1337";

fn main() {
    env_logger::init().unwrap();

    let args = env::args().collect::<Vec<String>>();
    if args.len() < 1 {
        error!("No user name specified (first CLI argument)");
        return;
    }
    let name = &args[1];
    info!("You are {:?}", name);

    info!("Connecting to server on address {}", ADDRESS);

    let mut context = zmq::Context::new();
    let mut sender = comm::Sender::new();
    let mut receiver = comm::Receiver::new();
    let mut s = context.socket(zmq::REQ).ok().unwrap();

    s.connect("tcp://localhost:1337").unwrap();
    info!("Connected to server");

    let mut req = Request::new();
    req.mut_identify().set_name(name.to_string());
    sender.send_request(&mut s, &req).unwrap();

    let resp = receiver.recv_response(&mut s).unwrap();
    if !resp.has_identified() || !resp.get_identified().has_access_token() {
        panic!("Unsupported response for identify: {:?}", resp);
    }
    let token: String = resp.get_identified().get_access_token().to_string();

    info!("Access token: {}", token);

    let mut counter = 0;
    loop {
        let payload = format!("{}", counter).into_bytes();

        let mut ping_req = Request::new();
        {
            let mut authed = ping_req.mut_authed();
            authed.set_access_token(token.clone());
            authed.mut_ping().set_payload(payload);
        }
        sender.send_request(&mut s, &ping_req).unwrap();

        let ping_resp = receiver.recv_response(&mut s).unwrap();
        if !ping_resp.has_pong() {
            panic!("Unsupported response for ping: {:?}", ping_resp);
        }
        let received_payload =
            String::from_utf8(ping_resp.get_pong().get_payload().to_vec()).unwrap();
        debug!("Received payload {:?}", received_payload);
        counter = counter + 1;
    }
}
