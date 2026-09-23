use std::ffi::{c_void, CStr};
use std::ptr::NonNull;
use std::sync::{Mutex, TryLockError};

use crate::apple_event_descriptor::AppleEventDescriptor;
use crate::ffi;
use crate::internal::bridge_error;
use crate::Result;
use doom_fish_utils::callback_context::CallbackContext;

/// Carries an `SBApplicationDelegate` error callback payload.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApplicationErrorEvent {
    /// Holds the Apple event class reported by `SBApplicationDelegate`.
    pub event_class: u32,
    /// Holds the Apple event ID reported by `SBApplicationDelegate`.
    pub event_id: u32,
    /// Holds the `NSError` domain reported by `SBApplicationDelegate`.
    pub error_domain: String,
    /// Holds the `NSError` code reported by `SBApplicationDelegate`.
    pub error_code: i64,
    /// Holds the localized `NSError` message reported by `SBApplicationDelegate`.
    pub error_message: String,
}

type DelegateCallback = dyn FnMut(&ApplicationErrorEvent) -> Option<AppleEventDescriptor> + Send;
type DelegateContext = CallbackContext<Mutex<Box<DelegateCallback>>>;

/// Owns an `SBApplicationDelegate` callback bridge.
#[derive(Debug)]
pub struct ApplicationDelegate {
    handle: NonNull<c_void>,
    context: DelegateContext,
}

impl ApplicationDelegate {
    /// Creates an `SBApplicationDelegate` bridge from a Rust callback.
    pub fn new<F>(callback: F) -> Result<Self>
    where
        F: FnMut(&ApplicationErrorEvent) -> Option<AppleEventDescriptor> + Send + 'static,
    {
        let context = DelegateContext::new(Mutex::new(Box::new(callback)));
        let mut error = std::ptr::null_mut();
        let raw = unsafe {
            ffi::application_delegate::sb_application_delegate_create(
                context.as_ptr(),
                application_delegate_trampoline,
                DelegateContext::RETAIN,
                DelegateContext::RELEASE,
                &raw mut error,
            )
        };

        let Some(handle) = NonNull::new(raw) else {
            return Err(bridge_error("sb_application_delegate_create", error));
        };

        Ok(Self { handle, context })
    }

    pub(crate) fn as_ptr(&self) -> *mut c_void {
        self.handle.as_ptr()
    }
}

impl Drop for ApplicationDelegate {
    fn drop(&mut self) {
        self.context.deactivate();
        unsafe { ffi::application_delegate::sb_application_delegate_release(self.handle.as_ptr()) };
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
    let event = ApplicationErrorEvent {
        event_class,
        event_id,
        error_domain: c_string_from_ptr(error_domain),
        error_code,
        error_message: c_string_from_ptr(error_message),
    };
    unsafe {
        DelegateContext::with(context, "ApplicationDelegate::on_error_event", |callback| {
            let mut callback = match callback.try_lock() {
                Ok(callback) => callback,
                Err(TryLockError::Poisoned(poisoned)) => poisoned.into_inner(),
                Err(TryLockError::WouldBlock) => return std::ptr::null_mut(),
            };
            (callback)(&event).map_or(std::ptr::null_mut(), AppleEventDescriptor::into_raw)
        })
    }
    .unwrap_or(std::ptr::null_mut())
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
