#![crate_name = "simple"]

extern crate capnp;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate shipit;
extern crate zmq;

use shipit::comm;
use shipit::protocol::response;

const ADDRESS: &'static str = "tcp://localhost:1337";

fn main() {
    env_logger::init().unwrap();
    info!("Connecting to server on address {}", ADDRESS);

    let mut context = zmq::Context::new();
    let mut sender = comm::Sender::new();
    let mut receiver = comm::Receiver::new();
    let mut s = context.socket(zmq::REQ).ok().unwrap();

    s.connect("tcp://localhost:1337").unwrap();
    info!("Connected to server");

    sender.send_request(&mut s, |r| {
        r.init_msg().init_identify().set_name("dflemstr");
    }).unwrap();

    let received = receiver.recv_response(&mut s).unwrap();
    let resp = received.get_root().unwrap();
    let token: String = match resp.get_msg().which() {
        Ok(response::msg::Identified(data)) => {
            let identified = data.unwrap();
            identified.get_access_token().unwrap().to_string()
        },
        Ok(response::msg::Error(data)) => {
            let error = data.unwrap();
            panic!("Error when sending identify: {}",
                   error.get_msg().unwrap());
        }
        _ => panic!("Unsupported response for identify"),
    };

    info!("Access token: {}", token);

    let mut counter = 0;
    loop {
        let payload = format!("{}", counter).into_bytes();

        sender.send_request(&mut s, |r| {
            let mut authed_msg = r.init_msg().init_authed();
            authed_msg.set_access_token(&token);
            authed_msg.init_msg().init_ping().set_payload(&payload);
        }).unwrap();

        let received = receiver.recv_response(&mut s).unwrap();
        let resp = received.get_root().unwrap();
        match resp.get_msg().which() {
            Ok(response::msg::Pong(data)) => {
                let pong = data.unwrap();
                let received_payload =
                    String::from_utf8(pong.get_payload().unwrap().to_vec()).unwrap();
                debug!("Received payload {:?}", received_payload);
            },
            _ => panic!("Unsupported response for ping"),
        };

        counter = counter + 1;
    }
}
