use core::fmt::Debug;

use crate::communication::{serial_std::SerialDeviceRequest, AsyncReadableStream, SerialDeviceRef};

pub struct SDevReadableStream<E> {
    serial: SerialDeviceRef,
    _e: std::marker::PhantomData<E>,
}

impl<E> SDevReadableStream<E> {
    pub fn new(serial: SerialDeviceRef) -> Self {
        SDevReadableStream {
            serial,
            _e: std::marker::PhantomData,
        }
    }
}

impl<E: Debug + Default> AsyncReadableStream for SDevReadableStream<E> {
    type Error = E;

    fn on_data(&mut self, cb: Box<dyn Fn(&[u8])>) -> Result<(), Self::Error> {
        let converted_cb = unsafe { core::mem::transmute(cb) };

        self.serial
            .lock()
            .map_err(|_| E::default())?
            .req_tx
            .send(SerialDeviceRequest::OnRx(converted_cb))
            .unwrap();
        Ok(())
    }
}

impl<E: Default> SDevReadableStream<E> {
    pub fn on_closed(&mut self, cb: Box<dyn Fn()>) -> Result<(), E> {
        let converted_cb = unsafe { core::mem::transmute(cb) };

        self.serial
            .lock()
            .map_err(|_| E::default())?
            .req_tx
            .send(SerialDeviceRequest::OnClose(converted_cb))
            .unwrap();
        Ok(())
    }
}
