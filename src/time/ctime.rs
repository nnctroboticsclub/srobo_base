use core::ffi::c_void;

use super::TimeImpl;

#[repr(C)]
pub struct CTime {
    now: Option<extern "C" fn(*const c_void) -> f32>,
    sleep: Option<extern "C" fn(*const c_void, f32)>,
    context: *const c_void,
}

impl TimeImpl for CTime {
    fn now(&self) -> core::time::Duration {
        if let Some(now) = self.now {
            let now = now(self.context);
            core::time::Duration::from_secs_f32(now)
        } else {
            core::time::Duration::from_secs(0)
        }
    }

    fn sleep(&self, duration: core::time::Duration) {
        if let Some(sleep) = self.sleep {
            let duration = duration.as_secs_f32();
            sleep(self.context, duration);
        }
    }
}

impl CTime {
    pub fn new() -> Self {
        CTime {
            now: None,
            sleep: None,
            context: core::ptr::null(),
        }
    }
}

#[no_mangle]
pub extern "C" fn __ffi_ctime_set_now(
    instance: *mut CTime,
    now: extern "C" fn(*const c_void) -> f32,
) {
    let instance = unsafe { &mut *instance };
    instance.now = Some(now);
}

#[no_mangle]
pub extern "C" fn __ffi_ctime_set_sleep(
    instance: *mut CTime,
    sleep: extern "C" fn(*const c_void, f32),
) {
    let instance = unsafe { &mut *instance };
    instance.sleep = Some(sleep);
}

#[no_mangle]
pub extern "C" fn __ffi_ctime_set_context(instance: *mut CTime, context: *const c_void) {
    let instance = unsafe { &mut *instance };
    instance.context = context;
}
