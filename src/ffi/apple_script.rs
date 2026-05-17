use std::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn sb_apple_script_create_with_source(
        source: *const c_char,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn sb_apple_script_create_with_contents_of_url(
        url: *const c_char,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn sb_apple_script_source(handle: *mut c_void) -> *mut c_char;
    pub fn sb_apple_script_is_compiled(handle: *mut c_void) -> bool;
    pub fn sb_apple_script_compile(handle: *mut c_void, error_out: *mut *mut c_char) -> bool;
    pub fn sb_apple_script_execute(handle: *mut c_void, error_out: *mut *mut c_char)
        -> *mut c_void;
    pub fn sb_apple_script_execute_apple_event(
        handle: *mut c_void,
        event: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn sb_apple_script_release(handle: *mut c_void);
}
