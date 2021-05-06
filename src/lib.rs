#![cfg_attr(not(test), no_std)]
#![feature(asm)]

#[macro_use]
extern crate num_derive;

pub use self::number::*;
pub use self::arch::*;
pub use self::error::*;

pub mod number;
#[cfg(target_arch = "aarch64")]
pub mod arch;
pub mod error;