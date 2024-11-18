//! Implementation of the [car](https://ipld.io/specs/transport/car/) format.
//! This is a fork of https://github.com/n0-computer/iroh-car that runs in no_std environments.

#![no_std]

extern crate alloc;

mod error;
mod header;
mod reader;
mod util;
mod writer;

pub use crate::error::Error;
pub use crate::header::CarHeader;
pub use crate::reader::CarReader;
pub use crate::writer::CarWriter;
