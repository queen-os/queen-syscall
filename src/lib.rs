#![cfg_attr(not(test), no_std)]

#[macro_use]
extern crate num_derive;

#[cfg(target_arch = "aarch64")]
pub use self::arch::*;
pub use self::{data::*, error::*, number::*};

#[cfg(target_arch = "aarch64")]
pub mod arch;
#[cfg(target_arch = "aarch64")]
pub mod call;
pub mod data;
pub mod error;
pub mod flags;
pub mod number;
