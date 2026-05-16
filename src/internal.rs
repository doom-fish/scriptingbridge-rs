use std::ffi::{c_char, c_void, CStr, CString};
use std::ptr::NonNull;

use crate::{ffi, Result, ScriptingBridgeError};

pub(crate) fn c_string(value: &str, function: &'static str) -> Result<CString> {
    CString::new(value).map_err(|_| {
        ScriptingBridgeError::new(function, "strings cannot contain interior NUL bytes")
    })
}

pub(crate) fn take_c_string(raw: *mut c_char) -> String {
    let string = unsafe { CStr::from_ptr(raw) }
        .to_string_lossy()
        .into_owned();
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
        unsafe { std::slice::from_raw_parts(raw, len) }.to_vec()
    };
    unsafe { ffi::sb_buffer_free(raw.cast()) };
    bytes
}

pub(crate) fn bridge_error(function: &'static str, error: *mut c_char) -> ScriptingBridgeError {
    if error.is_null() {
        return ScriptingBridgeError::new(function, "operation failed without an error message");
    }

    ScriptingBridgeError::new(function, take_c_string(error))
}

pub(crate) fn bool_result(
    ok: bool,
    function: &'static str,
    error: *mut c_char,
) -> Result<()> {
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
