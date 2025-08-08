use core::fmt::Debug;
use core::result::Result;

use alloc::boxed::Box;

use crate::communication::WritableStream;

pub trait SyncReadableStream {
    type Error: Debug;

    fn on_data(&mut self, cb: Box<dyn Fn(&[u8]) + Send>) -> Result<(), Self::Error>;
}

pub trait SyncSerial<E: Debug> {
    type ReadStream: SyncReadableStream<Error = E>;
    type WriteStream: WritableStream<Error = E>;

    fn open(&self) -> Result<(Self::ReadStream, Self::WriteStream), E>;
}
