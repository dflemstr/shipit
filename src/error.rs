extern crate protobuf;
extern crate zmq;

use std::convert;
use std::error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
    IO(io::Error),
    Protobuf(protobuf::ProtobufError),
    ZMQ(zmq::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            &Error::IO(ref e) => e.fmt(f),
            &Error::Protobuf(ref e) => e.fmt(f),
            &Error::ZMQ(ref e) => e.fmt(f),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self {
            &Error::IO(_) => "IO error",
            &Error::Protobuf(_) => "Protobuf error",
            &Error::ZMQ(_) => "ZMQ error",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            &Error::IO(ref e) => Option::Some(e),
            &Error::Protobuf(ref e) => Option::Some(e),
            &Error::ZMQ(ref e) => Option::Some(e),
        }
    }
}

impl convert::From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::IO(e)
    }
}

impl convert::From<protobuf::ProtobufError> for Error {
    fn from(e: protobuf::ProtobufError) -> Error {
        Error::Protobuf(e)
    }
}

impl convert::From<zmq::Error> for Error {
    fn from(e: zmq::Error) -> Error {
        Error::ZMQ(e)
    }
}
