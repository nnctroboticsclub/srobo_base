mod async_serial;

#[cfg(feature = "std")]
mod serial_std;

pub use async_serial::*;

#[cfg(feature = "std")]
pub use serial_std::*;
