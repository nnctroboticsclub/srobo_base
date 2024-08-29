#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod communication;
pub mod parser;
pub mod time;
pub mod utils;
