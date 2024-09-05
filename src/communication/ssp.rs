extern crate alloc;

use alloc::vec::Vec;

use super::AsyncReadableStream;

pub struct SerialServiceProtocolRx<S: AsyncReadableStream> {
    upper_stream_rx: S,
    services: Vec<Service>,
}

struct Service(u8);

impl Service {
    fn on_data(&self) {}
}
