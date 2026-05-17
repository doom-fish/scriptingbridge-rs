use std::ffi::{c_char, c_void, CStr, CString};
use std::ptr::NonNull;

use crate::{ffi, Result, ScriptingBridgeError};

pub(crate) fn c_string(value: &str, function: &'static str) -> Result<CString> {
    CString::new(value).map_err(|_| {
        ScriptingBridgeError::new(function, "strings cannot contain interior NUL bytes")
    })
}

pub(crate) fn take_c_string(raw: *mut c_char) -> String {
    // SAFETY: take_c_string is only called from FFI functions that return newly-allocated
    // C strings. raw is a pointer to a null-terminated string allocated by the C++ bridge.
    // We take ownership and free it after converting to Rust String, so there's no double-free.
    let string = unsafe { CStr::from_ptr(raw) }
        .to_string_lossy()
        .into_owned();
    // SAFETY: raw is a valid, newly-allocated pointer from the C++ bridge that we must free.
    unsafe { ffi::sb_string_free(raw) };
    string
}

pub(crate) fn take_optional_c_string(raw: *mut c_char) -> Option<String> {
    (!raw.is_null()).then(|| take_c_string(raw))
}

pub(crate) fn take_bytes(raw: *mut u8, len: usize) -> Vec<u8> {
    let bytes = if len == 0 || raw.is_null() {
        Vec::new()
    } else {
        // SAFETY: raw is a pointer to a byte buffer of length len allocated by the C++ bridge.
        // We only read from it (to_vec creates a copy), so there's no lifetime issue.
        unsafe { std::slice::from_raw_parts(raw, len) }.to_vec()
    };
    // SAFETY: raw is a valid pointer allocated by the C++ bridge that we must free,
    // or null (which is safe to free). We only free it once.
    unsafe { ffi::sb_buffer_free(raw.cast()) };
    bytes
}

pub(crate) fn bridge_error(function: &'static str, error: *mut c_char) -> ScriptingBridgeError {
    if error.is_null() {
        return ScriptingBridgeError::new(function, "operation failed without an error message");
    }

    ScriptingBridgeError::new(function, take_c_string(error))
}

pub(crate) fn bool_result(ok: bool, function: &'static str, error: *mut c_char) -> Result<()> {
    if ok {
        Ok(())
    } else {
        Err(bridge_error(function, error))
    }
}

pub(crate) fn required_handle<T>(
    raw: *mut c_void,
    function: &'static str,
    error: *mut c_char,
    map: impl FnOnce(NonNull<c_void>) -> T,
) -> Result<T> {
    let handle = NonNull::new(raw).ok_or_else(|| bridge_error(function, error))?;
    Ok(map(handle))
}

pub(crate) fn optional_handle<T>(
    raw: *mut c_void,
    function: &'static str,
    error: *mut c_char,
    map: impl FnOnce(NonNull<c_void>) -> T,
) -> Result<Option<T>> {
    NonNull::new(raw).map_or_else(
        || {
            if error.is_null() {
                Ok(None)
            } else {
                Err(bridge_error(function, error))
            }
        },
        |handle| Ok(Some(map(handle))),
    )
}
