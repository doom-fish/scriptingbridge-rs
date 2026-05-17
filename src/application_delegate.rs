use std::ffi::{c_void, CStr};
use std::ptr::NonNull;

use crate::apple_event_descriptor::AppleEventDescriptor;
use crate::ffi;
use crate::internal::bridge_error;
use crate::Result;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApplicationErrorEvent {
    pub event_class: u32,
    pub event_id: u32,
    pub error_domain: String,
    pub error_code: i64,
    pub error_message: String,
}

#[derive(Debug)]
pub struct ApplicationDelegate(NonNull<c_void>);

type DelegateCallback = dyn FnMut(&ApplicationErrorEvent) -> Option<AppleEventDescriptor> + Send;

struct CallbackState {
    callback: Box<DelegateCallback>,
}

impl ApplicationDelegate {
    pub fn new<F>(callback: F) -> Result<Self>
    where
        F: FnMut(&ApplicationErrorEvent) -> Option<AppleEventDescriptor> + Send + 'static,
    {
        let state = Box::new(CallbackState {
            callback: Box::new(callback),
        });
        let context = Box::into_raw(state).cast::<c_void>();
        let mut error = std::ptr::null_mut();
        let raw = unsafe {
            ffi::application_delegate::sb_application_delegate_create(
                context,
                application_delegate_trampoline,
                application_delegate_drop_trampoline,
                &mut error,
            )
        };

        let Some(handle) = NonNull::new(raw) else {
            unsafe { drop(Box::from_raw(context.cast::<CallbackState>())) };
            return Err(bridge_error("sb_application_delegate_create", error));
        };

        Ok(Self(handle))
    }

    pub(crate) fn as_ptr(&self) -> *mut c_void {
        self.0.as_ptr()
    }
}

impl Drop for ApplicationDelegate {
    fn drop(&mut self) {
        unsafe { ffi::application_delegate::sb_application_delegate_release(self.0.as_ptr()) };
    }
}

unsafe extern "C" fn application_delegate_trampoline(
    context: *mut c_void,
    event_class: u32,
    event_id: u32,
    error_domain: *const i8,
    error_code: i64,
    error_message: *const i8,
) -> *mut c_void {
    let Some(context) = NonNull::new(context) else {
        return std::ptr::null_mut();
    };

    let state = unsafe { &mut *context.cast::<CallbackState>().as_ptr() };
    let event = ApplicationErrorEvent {
        event_class,
        event_id,
        error_domain: c_string_from_ptr(error_domain),
        error_code,
        error_message: c_string_from_ptr(error_message),
    };

    (state.callback)(&event).map_or(std::ptr::null_mut(), AppleEventDescriptor::into_raw)
}

unsafe extern "C" fn application_delegate_drop_trampoline(context: *mut c_void) {
    if let Some(context) = NonNull::new(context) {
        unsafe { drop(Box::from_raw(context.cast::<CallbackState>().as_ptr())) };
    }
}

fn c_string_from_ptr(pointer: *const i8) -> String {
    if pointer.is_null() {
        String::new()
    } else {
        unsafe { CStr::from_ptr(pointer) }
            .to_string_lossy()
            .into_owned()
    }
}
