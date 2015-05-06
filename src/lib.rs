extern crate capnp;
extern crate zmq;

pub mod protocol {
    pub use ::shipit_capnp::*;
}

pub mod comm;
pub mod error;

mod shipit_capnp {
    include!(concat!(env!("OUT_DIR"), "/shipit_capnp.rs"));
}
