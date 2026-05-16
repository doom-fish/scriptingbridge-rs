use std::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn sb_apple_event_descriptor_null() -> *mut c_void;
    pub fn sb_apple_event_descriptor_create_with_descriptor_type_bytes(
        descriptor_type: u32,
        bytes: *const u8,
        length: i64,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn sb_apple_event_descriptor_create_with_descriptor_type_data(
        descriptor_type: u32,
        bytes: *const u8,
        length: i64,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn sb_apple_event_descriptor_create_with_boolean(value: bool) -> *mut c_void;
    pub fn sb_apple_event_descriptor_create_with_enum_code(value: u32) -> *mut c_void;
    pub fn sb_apple_event_descriptor_create_with_int32(value: i32) -> *mut c_void;
    pub fn sb_apple_event_descriptor_create_with_double(value: f64) -> *mut c_void;
    pub fn sb_apple_event_descriptor_create_with_type_code(value: u32) -> *mut c_void;
    pub fn sb_apple_event_descriptor_create_with_string(value: *const c_char) -> *mut c_void;
    pub fn sb_apple_event_descriptor_create_with_date(value: f64) -> *mut c_void;
    pub fn sb_apple_event_descriptor_create_with_file_url(
        value: *const c_char,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn sb_apple_event_descriptor_create_apple_event(
        event_class: u32,
        event_id: u32,
        target_descriptor: *mut c_void,
        return_id: i16,
        transaction_id: i32,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn sb_apple_event_descriptor_create_list() -> *mut c_void;
    pub fn sb_apple_event_descriptor_create_record() -> *mut c_void;
    pub fn sb_apple_event_descriptor_current_process() -> *mut c_void;
    pub fn sb_apple_event_descriptor_create_with_process_identifier(value: i32) -> *mut c_void;
    pub fn sb_apple_event_descriptor_create_with_bundle_identifier(
        value: *const c_char,
    ) -> *mut c_void;
    pub fn sb_apple_event_descriptor_create_with_application_url(
        value: *const c_char,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn sb_apple_event_descriptor_copy_aedesc(
        handle: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn sb_apple_event_descriptor_create_with_aedesc_no_copy(
        raw_desc_handle: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn sb_aedesc_descriptor_type(raw_desc_handle: *mut c_void) -> u32;
    pub fn sb_aedesc_release(raw_desc_handle: *mut c_void);

    pub fn sb_apple_event_descriptor_descriptor_type(handle: *mut c_void) -> u32;
    pub fn sb_apple_event_descriptor_copy_data(
        handle: *mut c_void,
        length_out: *mut i64,
    ) -> *mut u8;
    pub fn sb_apple_event_descriptor_boolean_value(handle: *mut c_void) -> bool;
    pub fn sb_apple_event_descriptor_enum_code_value(handle: *mut c_void) -> u32;
    pub fn sb_apple_event_descriptor_int32_value(handle: *mut c_void) -> i32;
    pub fn sb_apple_event_descriptor_double_value(handle: *mut c_void) -> f64;
    pub fn sb_apple_event_descriptor_type_code_value(handle: *mut c_void) -> u32;
    pub fn sb_apple_event_descriptor_string_value(handle: *mut c_void) -> *mut c_char;
    pub fn sb_apple_event_descriptor_date_value(handle: *mut c_void) -> f64;
    pub fn sb_apple_event_descriptor_file_url_value(handle: *mut c_void) -> *mut c_char;
    pub fn sb_apple_event_descriptor_event_class(handle: *mut c_void) -> u32;
    pub fn sb_apple_event_descriptor_event_id(handle: *mut c_void) -> u32;
    pub fn sb_apple_event_descriptor_return_id(handle: *mut c_void) -> i16;
    pub fn sb_apple_event_descriptor_transaction_id(handle: *mut c_void) -> i32;

    pub fn sb_apple_event_descriptor_set_param_descriptor(
        handle: *mut c_void,
        descriptor: *mut c_void,
        keyword: u32,
        error_out: *mut *mut c_char,
    ) -> bool;
    pub fn sb_apple_event_descriptor_param_descriptor_for_keyword(
        handle: *mut c_void,
        keyword: u32,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn sb_apple_event_descriptor_remove_param_descriptor(
        handle: *mut c_void,
        keyword: u32,
        error_out: *mut *mut c_char,
    ) -> bool;
    pub fn sb_apple_event_descriptor_set_attribute_descriptor(
        handle: *mut c_void,
        descriptor: *mut c_void,
        keyword: u32,
        error_out: *mut *mut c_char,
    ) -> bool;
    pub fn sb_apple_event_descriptor_attribute_descriptor_for_keyword(
        handle: *mut c_void,
        keyword: u32,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn sb_apple_event_descriptor_send_event(
        handle: *mut c_void,
        send_options: u64,
        timeout: f64,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn sb_apple_event_descriptor_is_record_descriptor(handle: *mut c_void) -> bool;
    pub fn sb_apple_event_descriptor_number_of_items(handle: *mut c_void) -> i64;
    pub fn sb_apple_event_descriptor_insert_descriptor(
        handle: *mut c_void,
        descriptor: *mut c_void,
        index: i64,
        error_out: *mut *mut c_char,
    ) -> bool;
    pub fn sb_apple_event_descriptor_descriptor_at_index(
        handle: *mut c_void,
        index: i64,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn sb_apple_event_descriptor_remove_descriptor_at_index(
        handle: *mut c_void,
        index: i64,
        error_out: *mut *mut c_char,
    ) -> bool;
    pub fn sb_apple_event_descriptor_set_descriptor(
        handle: *mut c_void,
        descriptor: *mut c_void,
        keyword: u32,
        error_out: *mut *mut c_char,
    ) -> bool;
    pub fn sb_apple_event_descriptor_descriptor_for_keyword(
        handle: *mut c_void,
        keyword: u32,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn sb_apple_event_descriptor_remove_descriptor_for_keyword(
        handle: *mut c_void,
        keyword: u32,
        error_out: *mut *mut c_char,
    ) -> bool;
    pub fn sb_apple_event_descriptor_keyword_for_descriptor_at_index(
        handle: *mut c_void,
        index: i64,
        error_out: *mut *mut c_char,
    ) -> u32;
    pub fn sb_apple_event_descriptor_coerce_to_descriptor_type(
        handle: *mut c_void,
        descriptor_type: u32,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn sb_apple_event_descriptor_release(handle: *mut c_void);
}
