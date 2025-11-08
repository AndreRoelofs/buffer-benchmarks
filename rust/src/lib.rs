mod constants;

capnp::generated_code!(pub mod message_capnp);

pub mod capnps;
pub mod flatbuffers;
pub mod prost;
pub mod protobuf;
