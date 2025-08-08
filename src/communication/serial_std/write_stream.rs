use core::fmt::Debug;

use crate::communication::{serial_std::SerialDeviceRequest, SerialDeviceRef, WritableStream};

pub struct SDevWritableStream<E> {
    serial: SerialDeviceRef,

    _e: std::marker::PhantomData<E>,
}

impl<E> SDevWritableStream<E> {
    pub fn new(serial: SerialDeviceRef) -> Self {
        SDevWritableStream {
            serial,
            _e: std::marker::PhantomData,
        }
    }
}

impl<E: Debug + Default> WritableStream for SDevWritableStream<E> {
    type Error = E;

    fn write(&mut self, data: &[u8]) -> Result<(), Self::Error> {
        self.serial
            .lock()
            .map_err(|_| E::default())?
            .req_tx
            .send(SerialDeviceRequest::Tx(data.to_vec()))
            .unwrap();
        Ok(())
    }
}
