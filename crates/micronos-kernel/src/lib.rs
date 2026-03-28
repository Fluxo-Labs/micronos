#![no_std]
#![doc = include_str!("../README.md")]

extern crate alloc;

pub mod boot;
pub mod events;
pub mod kernel;
pub mod posix;
pub mod scheduler;
pub mod syscall;

pub use boot::BootSequence;
pub use kernel::MicronKernel;
