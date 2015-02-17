extern crate protobuf;
extern crate zmq;

use std::error;
use std::error::FromError;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    Protobuf(protobuf::error::ProtobufError),
    ZMQ(zmq::Error),
    UnknownRequest,
    Unauthorized,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            &Error::Protobuf(ref e) => write!(f, "protobuf error: {}", e),
            &Error::ZMQ(ref e) => write!(f, "ZMQ error: {}", e),
            &Error::UnknownRequest => write!(f, "unknown request"),
            &Error::Unauthorized => write!(f, "unauthorized"),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self {
            &Error::Protobuf(_) => "protobuf error",
            &Error::ZMQ(_) => "ZMQ error",
            &Error::UnknownRequest => "unknown request",
            &Error::Unauthorized => "unauthorized",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            &Error::Protobuf(ref e) => Option::Some(e),
            &Error::ZMQ(ref e) => Option::Some(e),
            _ => Option::None,
        }
    }
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
