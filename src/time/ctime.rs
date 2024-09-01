use super::TimeImpl;

#[repr(C)]
pub struct CTime {
    now: Option<extern "C" fn() -> f32>,
    sleep: Option<extern "C" fn(f32)>,
}

impl TimeImpl for CTime {
    fn now(&self) -> core::time::Duration {
        if let Some(now) = self.now {
            let now = now();
            core::time::Duration::from_secs_f32(now)
        } else {
            core::time::Duration::from_secs(0)
        }
    }

    fn sleep(&self, duration: core::time::Duration) {
        if let Some(sleep) = self.sleep {
            let duration = duration.as_secs_f32();
            sleep(duration);
        }
    }
}

impl CTime {
    pub fn new() -> Self {
        CTime {
            now: None,
            sleep: None,
        }
    }
}

#[no_mangle]
pub extern "C" fn __ffi_ctime_set_now(instance: *mut CTime, now: extern "C" fn() -> f32) {
    let instance = unsafe { &mut *instance };
    instance.now = Some(now);
}

#[no_mangle]
pub extern "C" fn __ffi_ctime_set_sleep(instance: *mut CTime, sleep: extern "C" fn(f32)) {
    let instance = unsafe { &mut *instance };
    instance.sleep = Some(sleep);
}
