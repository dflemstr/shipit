use protobuf;
use zmq;

use error::Error;
use protocol::{Request, Response};

pub struct Sender;

pub struct Receiver;

impl Sender {
    pub fn new() -> Self {
        Sender
    }

    pub fn send_request(&mut self,
                        s: &mut zmq::Socket,
                        request: &Request) -> Result<(), Error> {
        send_raw(s, request)
    }

    pub fn send_response(&mut self,
                         s: &mut zmq::Socket,
                         response: &Response) -> Result<(), Error> {
        send_raw(s, response)
    }
}

impl Receiver {
    pub fn new() -> Self {
        Receiver
    }

    pub fn recv_request(&mut self,
                        s: &mut zmq::Socket) -> Result<Request, Error> {
        recv_raw(s)
    }

    pub fn recv_response(&mut self,
                         s: &mut zmq::Socket)-> Result<Response, Error> {
        recv_raw(s)
    }
}

#[inline]
fn recv_raw<A>(socket: &mut zmq::Socket) -> Result<A, Error>
    where A: protobuf::MessageStatic {
    let message = try!(socket.recv_bytes(0));
    let entity = try!(protobuf::parse_from_bytes(&(*message)));
    Ok(entity)
}

#[inline]
fn send_raw<A>(socket: &mut zmq::Socket, entity: &A) -> Result<(), Error>
    where A: protobuf::Message {
    let size = entity.compute_size() as usize;
    let mut message =
        try!(unsafe { zmq::Message::with_capacity_unallocated(size) });

    // limit lifetime of mut message borrow
    {
        let mut data: &mut [u8] = &mut *message;
        let mut out = protobuf::CodedOutputStream::new(&mut data);
        try!(entity.write_to_with_cached_sizes(&mut out));
    }

    try!(socket.send(&message, 0));
    Ok(())
}
