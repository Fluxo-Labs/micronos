#![no_std]
#![doc = include_str!("../README.md")]

extern crate alloc;

pub mod error;
pub mod traits;
pub mod types;

pub use error::{Error, Result};
pub use types::*;
