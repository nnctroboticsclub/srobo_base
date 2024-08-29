extern crate alloc;

use alloc::boxed::Box;
use core::ops::Deref;
use core::option::Option;
use core::time::Duration;

use crate::time::TimeImpl;
pub struct Swmr<T>(T);

impl<T> Swmr<T> {
    pub fn new(data: T) -> (SwmrWriter<T>, SwmrReader<T>) {
        let swmr = Box::into_raw(Box::new(Swmr(data)));

        (SwmrWriter(swmr), SwmrReader(swmr))
    }
}

pub struct SwmrWriter<T>(*mut Swmr<T>);

impl<T> SwmrWriter<T> {
    pub fn write(&self, data: T) {
        unsafe {
            (*self.0).0 = data;
        }
    }
}

#[derive(Clone, Copy)]
pub struct SwmrReader<T>(*mut Swmr<T>);

impl<T> Deref for SwmrReader<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &(*self.0).0 }
    }
}

impl<T> SwmrReader<Option<T>> {
    pub fn wait_available(&self, timeout: Duration, time: &impl TimeImpl) -> bool {
        let start = time.now();
        loop {
            if self.is_some() {
                return true;
            }

            if time.now() - start > timeout {
                return false;
            }

            time.sleep(Duration::from_millis(10));
        }
    }
}
