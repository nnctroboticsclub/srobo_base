use core::fmt::Debug;

pub trait WritableStream {
    type Error: Debug;

    fn write(&mut self, data: &[u8]) -> Result<(), Self::Error>;
}
