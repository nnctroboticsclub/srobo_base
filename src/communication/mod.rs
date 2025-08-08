#[cfg(feature = "alloc")]
mod async_stream;

#[cfg(feature = "alloc")]
pub use async_stream::{AsyncReadableStream, AsyncSerial};

#[cfg(feature = "alloc")]
mod ffi_stream;

#[cfg(feature = "alloc")]
pub use ffi_stream::*;

#[cfg(feature = "std")]
mod serial_std;

#[cfg(feature = "std")]
pub use serial_std::*;

#[cfg(feature = "alloc")]
mod sync_stream;
#[cfg(feature = "alloc")]
pub use sync_stream::*;

#[cfg(feature = "alloc")]
mod write_stream;

#[cfg(feature = "alloc")]
pub use write_stream::WritableStream;
