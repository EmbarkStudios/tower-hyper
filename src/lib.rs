//! A tower hyper bridge library that provides utilties
//! to use hyper with a tower middleware stack.
//!
//! # Overview
//!
//! This library is comprised of client and server modules. Currently, only
//! the client portion is done and working. The server side is blocked partially
//! by hypers use of its own Service and MakeService traits.

pub mod client;
pub mod retries;
pub mod util;

mod body;

pub use body::Body;
