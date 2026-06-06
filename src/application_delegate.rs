use std::ffi::{c_void, CStr};
use std::ptr::NonNull;

use crate::apple_event_descriptor::AppleEventDescriptor;
use crate::ffi;
use crate::internal::bridge_error;
use crate::Result;
use doom_fish_utils::panic_safe::catch_user_panic;

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

/// Owns an `SBApplicationDelegate` callback bridge.
#[derive(Debug)]
pub struct ApplicationDelegate(NonNull<c_void>);

type DelegateCallback = dyn FnMut(&ApplicationErrorEvent) -> Option<AppleEventDescriptor> + Send;

struct CallbackState {
    callback: Box<DelegateCallback>,
}

impl ApplicationDelegate {
    /// Creates an `SBApplicationDelegate` bridge from a Rust callback.
    pub fn new<F>(callback: F) -> Result<Self>
    where
        F: FnMut(&ApplicationErrorEvent) -> Option<AppleEventDescriptor> + Send + 'static,
    {
        let state = Box::new(CallbackState {
            callback: Box::new(callback),
        });
        let context = Box::into_raw(state).cast::<c_void>();
        let mut error = std::ptr::null_mut();
        // SAFETY: We're passing a valid context pointer (from Box::into_raw) and valid
        // C function pointers (application_delegate_trampoline and
        // application_delegate_drop_trampoline). The C++ bridge will hold the context
        // and pass it back to our callbacks. If the bridge creation succeeds, it takes
        // ownership of the context and will call our drop_trampoline to free it.
        let raw = unsafe {
            ffi::application_delegate::sb_application_delegate_create(
                context,
                application_delegate_trampoline,
                application_delegate_drop_trampoline,
                &mut error,
            )
        };

        let Some(handle) = NonNull::new(raw) else {
            // SAFETY: If bridge creation failed, we need to clean up our leaked context.
            // We know context is a valid pointer because we just created it from Box::into_raw.
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
        // SAFETY: self.0 is always a valid NonNull<c_void> returned from the C++ bridge.
        // Calling sb_application_delegate_release on it is safe and will release the bridge
        // object and invoke our drop_trampoline callback to clean up the context.
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

    let mut result = std::ptr::null_mut();
    catch_user_panic("ApplicationDelegate::on_error_event", || {
        // SAFETY: context is guaranteed to be a valid, non-null CallbackState pointer by
        // the C++ bridge, because we created it in ApplicationDelegate::new and store it
        // in the Swift object's context field. The callback fires while the ApplicationDelegate
        // is alive, so the pointer is guaranteed to be valid. Access is exclusive (only we
        // dereference it per callback invocation) and never races because the caller
        // (Swift/Objective-C) is single-threaded per delegate instance.
        let state = unsafe { &mut *context.cast::<CallbackState>().as_ptr() };
        let event = ApplicationErrorEvent {
            event_class,
            event_id,
            error_domain: c_string_from_ptr(error_domain),
            error_code,
            error_message: c_string_from_ptr(error_message),
        };

        result =
            (state.callback)(&event).map_or(std::ptr::null_mut(), AppleEventDescriptor::into_raw);
    });
    result
}

unsafe extern "C" fn application_delegate_drop_trampoline(context: *mut c_void) {
    if let Some(context) = NonNull::new(context) {
        // The drop trampoline runs from the Swift delegate's `deinit` across the C ABI.
        // Dropping the user-provided closure can run arbitrary user `Drop` code, which may
        // panic; an unwind across `extern "C"` into Swift is undefined behaviour, so the
        // drop is wrapped in `catch_user_panic`.
        catch_user_panic("ApplicationDelegate::drop", || {
            // SAFETY: context is guaranteed to be a valid, non-null CallbackState pointer that
            // we leaked via Box::into_raw in ApplicationDelegate::new. We only drop it once
            // (the Swift bridge calls this exactly once per delegate), so there's no double-free.
            // No other code holds a reference to the CallbackState because it was leaked
            // exclusively for the bridge.
            unsafe { drop(Box::from_raw(context.cast::<CallbackState>().as_ptr())) };
        });
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
