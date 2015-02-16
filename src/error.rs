extern crate protobuf;
extern crate zmq;

use std::error::FromError;

pub enum Error {
    Protobuf(protobuf::error::ProtobufError),
    ZMQ(zmq::Error),
    UnknownRequest,
    Unauthorized,
}

impl FromError<zmq::Error> for Error {
    fn from_error(e: zmq::Error) -> Self {
        Error::ZMQ(e)
    }
}

impl FromError<protobuf::error::ProtobufError> for Error {
    fn from_error(e: protobuf::error::ProtobufError) -> Self {
        Error::Protobuf(e)
    }
}
