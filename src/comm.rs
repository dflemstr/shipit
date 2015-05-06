use std::io;
use std::marker::PhantomData;

use capnp;
use capnp::serialize::OwnedSpaceMessageReader;
use capnp::message::{DEFAULT_READER_OPTIONS,
                     MessageBuilder, MessageReader,
                     MallocMessageBuilder};
use capnp::traits::FromPointerReader;
use capnp::serialize_packed;
use zmq;

use error::Error;
use protocol::{request, response};

pub struct Sender {
    builder: MallocMessageBuilder,
    buffer: Vec<u8>,
}

pub struct Receiver;

pub struct Received<A> {
    reader: OwnedSpaceMessageReader,
    data: PhantomData<A>,
}

impl Sender {
    pub fn new() -> Self {
        Sender {
            builder: MallocMessageBuilder::new_default(),
            buffer: Vec::new(),
        }
    }

    pub fn send_request<F>(&mut self,
                           s: &mut zmq::Socket,
                           make_request: F) -> Result<(), Error>
        where F: FnOnce(request::Builder) {

        make_request(self.builder.init_root::<request::Builder>());
        try!(send_raw(s, &mut self.buffer, &mut self.builder));
        Ok(())
    }

    pub fn send_response<F>(&mut self,
                            s: &mut zmq::Socket,
                            make_response: F) -> Result<(), Error>
        where F: FnOnce(response::Builder) {

        make_response(self.builder.init_root::<response::Builder>());
        try!(send_raw(s, &mut self.buffer, &mut self.builder));
        Ok(())
    }
}

impl Receiver {
    pub fn new() -> Self {
        Receiver
    }

    pub fn recv_request<'a>(&mut self, s: &mut zmq::Socket)
                            -> Result<Received<request::Reader<'a>>, Error> {

        let reader = try!(recv_raw(s));
        Ok(Received::new(reader))
    }

    pub fn recv_response<'a>(&mut self, s: &mut zmq::Socket)
                             -> Result<Received<response::Reader<'a>>, Error> {
        let reader = try!(recv_raw(s));
        Ok(Received::new(reader))
    }
}

impl<'a, A: FromPointerReader<'a>> Received<A> {
    fn new(reader: OwnedSpaceMessageReader) -> Self {
        Received { reader: reader, data: PhantomData }
    }

    pub fn get_root(&'a self) -> Result<A, capnp::Error> {
        self.reader.get_root::<A>()
    }
}

fn recv_raw(socket: &mut zmq::Socket)
            -> Result<OwnedSpaceMessageReader, Error> {
    let message = try!(socket.recv_bytes(0));
    let mut cursor = io::Cursor::new(message);
    let reader =
        try!(serialize_packed::read_message(&mut cursor, DEFAULT_READER_OPTIONS));
    Ok(reader)
}

fn send_raw<B>(socket: &mut zmq::Socket,
               buffer: &mut Vec<u8>,
               message: &mut B) -> Result<(), Error>
    where B: capnp::message::MessageBuilder {

    buffer.clear();
    try!(serialize_packed::write_message(buffer, message));
    try!(socket.send(buffer, 0));
    Ok(())
}
