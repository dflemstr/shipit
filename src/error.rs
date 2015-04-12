extern crate protobuf;
extern crate zmq;

use std::error;
use std::convert::From;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::io;

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    Protobuf(protobuf::error::ProtobufError),
    ZMQ(zmq::Error),
    UnknownRequest,
    Unauthorized(UnauthReason),
}

#[derive(Debug)]
pub enum UnauthReason {
    NoTokenSpecified,
    NoSuchToken,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            &Error::IO(ref e) => write!(f, "IO error: {}", e),
            &Error::Protobuf(ref e) => write!(f, "protobuf error: {}", e),
            &Error::ZMQ(ref e) => write!(f, "ZMQ error: {}", e),
            &Error::UnknownRequest => write!(f, "unknown request"),
            &Error::Unauthorized(UnauthReason::NoTokenSpecified) =>
                write!(f, "unauthorized: no token specified"),
            &Error::Unauthorized(UnauthReason::NoSuchToken) =>
                write!(f, "unauthorized: no such token"),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self {
            &Error::IO(_) => "IO error",
            &Error::Protobuf(_) => "protobuf error",
            &Error::ZMQ(_) => "ZMQ error",
            &Error::UnknownRequest => "unknown request",
            &Error::Unauthorized(UnauthReason::NoTokenSpecified) =>
                "unauthorized: no token specified",
            &Error::Unauthorized(UnauthReason::NoSuchToken) =>
                "unauthorized: no such token",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            &Error::IO(ref e) => Option::Some(e),
            &Error::Protobuf(ref e) => Option::Some(e),
            &Error::ZMQ(ref e) => Option::Some(e),
            _ => Option::None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IO(e)
    }
}

impl From<zmq::Error> for Error {
    fn from(e: zmq::Error) -> Self {
        Error::ZMQ(e)
    }
}

impl From<protobuf::error::ProtobufError> for Error {
    fn from(e: protobuf::error::ProtobufError) -> Self {
        Error::Protobuf(e)
    }
}
