use std::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn sb_string_free(pointer: *mut c_char);

    pub fn sb_application_create(
        bundle_identifier: *const c_char,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn sb_application_is_running(handle: *mut c_void) -> bool;
    pub fn sb_application_launch(handle: *mut c_void, error_out: *mut *mut c_char) -> bool;
    pub fn sb_application_activate(handle: *mut c_void, error_out: *mut *mut c_char) -> bool;
    pub fn sb_application_quit(handle: *mut c_void, error_out: *mut *mut c_char) -> bool;
    pub fn sb_application_terminate(handle: *mut c_void, error_out: *mut *mut c_char) -> bool;
    pub fn sb_application_tell(
        handle: *mut c_void,
        command: *const c_char,
        argument: *const c_char,
        error_out: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn sb_application_object_for_key_path(
        handle: *mut c_void,
        key_path: *const c_char,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn sb_application_element_array_for_key_path(
        handle: *mut c_void,
        key_path: *const c_char,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn sb_application_release(handle: *mut c_void);

    pub fn sb_object_description(handle: *mut c_void) -> *mut c_char;
    pub fn sb_object_get_description(handle: *mut c_void) -> *mut c_char;
    pub fn sb_object_last_error_description(handle: *mut c_void) -> *mut c_char;
    pub fn sb_object_release(handle: *mut c_void);

    pub fn sb_element_array_description(handle: *mut c_void) -> *mut c_char;
    pub fn sb_element_array_get_description(handle: *mut c_void) -> *mut c_char;
    pub fn sb_element_array_release(handle: *mut c_void);
}
