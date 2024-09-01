#[cfg(feature = "alloc")]
mod async_serial;

#[cfg(feature = "alloc")]
pub use async_serial::*;

#[cfg(feature = "std")]
mod serial_std;

#[cfg(feature = "std")]
pub use serial_std::*;

#[cfg(feature = "alloc")]
mod ffi_stream;

#[cfg(feature = "alloc")]
pub use ffi_stream::*;
