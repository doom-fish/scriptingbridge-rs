use std::ffi::{c_char, c_void};

unsafe extern "C" {
    pub fn sb_application_create_with_bundle_identifier(
        bundle_identifier: *const c_char,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn sb_application_create_with_url(
        url: *const c_char,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn sb_application_create_with_process_identifier(
        process_identifier: i32,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;

    pub fn sb_application_shared_with_bundle_identifier(
        bundle_identifier: *const c_char,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn sb_application_shared_with_url(
        url: *const c_char,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn sb_application_shared_with_process_identifier(
        process_identifier: i32,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;

    pub fn sb_application_copy_object(handle: *mut c_void) -> *mut c_void;
    pub fn sb_application_class_for_scripting_class(
        handle: *mut c_void,
        class_name: *const c_char,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn sb_scripting_class_name(handle: *mut c_void) -> *mut c_char;
    pub fn sb_scripting_class_release(handle: *mut c_void);

    pub fn sb_application_is_running(handle: *mut c_void) -> bool;
    pub fn sb_application_process_identifier(handle: *mut c_void) -> i32;
    pub fn sb_application_launch(handle: *mut c_void, error_out: *mut *mut c_char) -> bool;
    pub fn sb_application_activate(handle: *mut c_void, error_out: *mut *mut c_char) -> bool;
    pub fn sb_application_quit(handle: *mut c_void, error_out: *mut *mut c_char) -> bool;
    pub fn sb_application_terminate(handle: *mut c_void, error_out: *mut *mut c_char) -> bool;

    pub fn sb_application_get_launch_flags(handle: *mut c_void) -> u32;
    pub fn sb_application_set_launch_flags(
        handle: *mut c_void,
        launch_flags: u32,
        error_out: *mut *mut c_char,
    ) -> bool;
    pub fn sb_application_get_send_mode(handle: *mut c_void) -> i32;
    pub fn sb_application_set_send_mode(
        handle: *mut c_void,
        send_mode: i32,
        error_out: *mut *mut c_char,
    ) -> bool;
    pub fn sb_application_get_timeout(handle: *mut c_void) -> i64;
    pub fn sb_application_set_timeout(
        handle: *mut c_void,
        timeout: i64,
        error_out: *mut *mut c_char,
    ) -> bool;

    pub fn sb_application_set_delegate(
        handle: *mut c_void,
        delegate: *mut c_void,
        error_out: *mut *mut c_char,
    ) -> bool;
    pub fn sb_application_has_delegate(handle: *mut c_void) -> bool;

    pub fn sb_application_tell(
        handle: *mut c_void,
        command: *const c_char,
        argument: *const c_char,
        error_out: *mut *mut c_char,
    ) -> *mut c_char;
    pub fn sb_application_send_event(
        handle: *mut c_void,
        event_class: u32,
        event_id: u32,
        parameter_codes: *const u32,
        parameter_values: *const *mut c_void,
        parameter_count: i64,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
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
}
