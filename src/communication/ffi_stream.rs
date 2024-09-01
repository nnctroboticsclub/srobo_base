use core::ffi::c_void;

use alloc::boxed::Box;

use super::{AsyncReadableStream, WritableStream};

#[repr(C)]
pub struct CStreamRx {
    on_data: Box<dyn Fn(&[u8])>,
}

impl CStreamRx {
    pub fn new() -> Self {
        CStreamRx {
            on_data: Box::new(|_| {}),
        }
    }
}

impl AsyncReadableStream for CStreamRx {
    type Error = ();

    fn on_data(&mut self, cb: Box<dyn Fn(&[u8])>) -> Result<(), Self::Error> {
        self.on_data = cb;
        Ok(())
    }
}

#[no_mangle]
pub extern "C" fn __ffi_cstream_feed_rx(instance: *mut CStreamRx, data: *const u8, len: usize) {
    let instance = unsafe { &mut *instance };
    let data = unsafe { core::slice::from_raw_parts(data, len) };
    (instance.on_data)(data);
}

#[repr(C)]
pub struct CStreamTx {
    write: Option<extern "C" fn(instance: *const c_void, data: *const u8, len: usize) -> ()>,
}

impl CStreamTx {
    pub fn new() -> Self {
        CStreamTx { write: None }
    }
}

impl WritableStream for CStreamTx {
    type Error = ();

    fn write(&mut self, data: &[u8]) -> Result<(), Self::Error> {
        if let Some(write) = self.write {
            (write)(
                self as *const Self as *const c_void,
                data.as_ptr(),
                data.len(),
            );
        }
        Ok(())
    }
}

#[no_mangle]
pub extern "C" fn __ffi_cstream_associate_tx(
    instance: *mut CStreamTx,
    write: extern "C" fn(instance: *const c_void, data: *const u8, len: usize) -> (),
) {
    let instance = unsafe { &mut *instance };
    instance.write = Some(write);
}
