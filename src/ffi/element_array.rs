use std::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn sb_element_array_object_with_name(
        handle: *mut c_void,
        name: *const c_char,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn sb_element_array_object_with_id(
        handle: *mut c_void,
        identifier: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn sb_element_array_object_at_location(
        handle: *mut c_void,
        location: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn sb_element_array_array_by_applying_selector(
        handle: *mut c_void,
        selector: *const c_char,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn sb_element_array_array_by_applying_selector_with_object(
        handle: *mut c_void,
        selector: *const c_char,
        argument: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn sb_element_array_get(
        handle: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn sb_element_array_description(handle: *mut c_void) -> *mut c_char;
    pub fn sb_element_array_get_description(handle: *mut c_void) -> *mut c_char;
    pub fn sb_element_array_release(handle: *mut c_void);
}
