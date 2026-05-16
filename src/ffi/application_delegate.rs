use std::ffi::{c_char, c_void};

pub type SbApplicationDelegateCallback = unsafe extern "C" fn(
    context: *mut c_void,
    event_class: u32,
    event_id: u32,
    error_domain: *const c_char,
    error_code: i64,
    error_message: *const c_char,
) -> *mut c_void;

pub type SbApplicationDelegateDrop = unsafe extern "C" fn(context: *mut c_void);

unsafe extern "C" {
    pub fn sb_application_delegate_create(
        context: *mut c_void,
        callback: SbApplicationDelegateCallback,
        drop_context: SbApplicationDelegateDrop,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn sb_application_delegate_release(handle: *mut c_void);
}
