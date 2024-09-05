use core::fmt::Debug;
use core::result::Result;

use alloc::boxed::Box;

pub trait WritableStream {
    type Error: Debug;

    fn write(&mut self, data: &[u8]) -> Result<(), Self::Error>;
}

pub trait AsyncReadableStream {
    type Error: Debug;

    // The data which accessed callback can be not Sync and Send
    // In ARM processor, we can't atomic operation in interrupt context (Usually, callback is called in interrupt context)
    fn on_data(&mut self, cb: Box<dyn Fn(&[u8])>) -> Result<(), Self::Error>;
}

pub trait AsyncSerial<E: Debug> {
    type ReadStream: AsyncReadableStream<Error = E>;
    type WriteStream: WritableStream<Error = E>;

    fn open(&self) -> Result<(Self::ReadStream, Self::WriteStream), E>;
}
