use std::ffi::{c_char, c_void};

pub mod apple_event_descriptor;
pub mod apple_script;
pub mod application;
pub mod application_delegate;
pub mod element_array;
pub mod object;

unsafe extern "C" {
    pub fn sb_string_free(pointer: *mut c_char);
    pub fn sb_buffer_free(pointer: *mut c_void);
}
