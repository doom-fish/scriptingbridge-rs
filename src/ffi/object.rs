use std::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn sb_object_create(error_out: *mut *mut c_char) -> *mut c_void;
    pub fn sb_object_create_with_properties(
        names: *const *const c_char,
        values: *const *mut c_void,
        count: i64,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn sb_object_create_with_data(
        data: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn sb_object_create_with_element_code(
        element_code: u32,
        names: *const *const c_char,
        values: *const *mut c_void,
        count: i64,
        data: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn sb_object_get(handle: *mut c_void, error_out: *mut *mut c_char) -> *mut c_void;
    pub fn sb_object_description(handle: *mut c_void) -> *mut c_char;
    pub fn sb_object_get_description(handle: *mut c_void) -> *mut c_char;
    pub fn sb_object_last_error_description(handle: *mut c_void) -> *mut c_char;
    pub fn sb_object_property_with_code(
        handle: *mut c_void,
        code: u32,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn sb_object_property_with_class(
        handle: *mut c_void,
        class_handle: *mut c_void,
        code: u32,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn sb_object_element_array_with_code(
        handle: *mut c_void,
        code: u32,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn sb_object_send_event(
        handle: *mut c_void,
        event_class: u32,
        event_id: u32,
        parameter_codes: *const u32,
        parameter_values: *const *mut c_void,
        parameter_count: i64,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn sb_object_set_to(
        handle: *mut c_void,
        value: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> bool;
    pub fn sb_object_release(handle: *mut c_void);
}
