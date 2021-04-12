#![allow(unused_imports)]
#![allow(unreachable_code)]
#![allow(unused_mut)]

#[macro_use]
extern crate serde_json;

pub(crate) mod client;
pub(crate) use config::*;
pub use message::Message;
pub mod message;
pub mod user;
pub mod room;

mod config;
pub mod prelude;

#[cfg(test)]
mod tests;
