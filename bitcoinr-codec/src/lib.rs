//! `bitcoinr-codec` crate enables you to decode/encode bitcoin p2p network messge into/from
//! Rust style structures.
//! Decodec message is `Message` struct which consists `network_type` and `command` field.
//! Command is main enums which represents each message command.
//! Some command contains payload such as VersionPayload.
//!
//! Each command payload contains almost all field defined in bitcoin protocol.
//! It means, you can specify any fields of each message even `user_agent` or `addr_trans_port`.

extern crate futures;
extern crate tokio_core;
extern crate tokio_io;
extern crate bytes;
extern crate chrono;
extern crate sha2;

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;


pub mod message;
pub mod net;

pub mod error;


pub use message::{MsgCodec, Message, Command};
pub use net::NetworkType;
