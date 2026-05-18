use std::ffi::c_void;
use std::ptr::NonNull;

use crate::apple_event_descriptor::AppleEventDescriptor;
use crate::ffi;
use crate::internal::{
    bool_result, c_string, optional_handle, required_handle, take_optional_c_string,
};
use crate::Result;

/// Matches the `NSAppleScriptErrorMessage` key in `NSAppleScript` error dictionaries.
pub const APPLE_SCRIPT_ERROR_MESSAGE_KEY: &str = "NSAppleScriptErrorMessage";
/// Matches the `NSAppleScriptErrorNumber` key in `NSAppleScript` error dictionaries.
pub const APPLE_SCRIPT_ERROR_NUMBER_KEY: &str = "NSAppleScriptErrorNumber";
/// Matches the `NSAppleScriptErrorAppName` key in `NSAppleScript` error dictionaries.
pub const APPLE_SCRIPT_ERROR_APP_NAME_KEY: &str = "NSAppleScriptErrorAppName";
/// Matches the `NSAppleScriptErrorBriefMessage` key in `NSAppleScript` error dictionaries.
pub const APPLE_SCRIPT_ERROR_BRIEF_MESSAGE_KEY: &str = "NSAppleScriptErrorBriefMessage";
/// Matches the `NSAppleScriptErrorRange` key in `NSAppleScript` error dictionaries.
pub const APPLE_SCRIPT_ERROR_RANGE_KEY: &str = "NSAppleScriptErrorRange";

/// Wraps an `NSAppleScript` instance.
#[derive(Debug)]
pub struct AppleScript(NonNull<c_void>);

impl AppleScript {
    /// Creates an `NSAppleScript` from source text.
    pub fn with_source(source: &str) -> Result<Self> {
        let source = c_string(source, "sb_apple_script_create_with_source")?;
        let mut error = std::ptr::null_mut();
        let raw = unsafe {
            ffi::apple_script::sb_apple_script_create_with_source(source.as_ptr(), &mut error)
        };
        required_handle(
            raw,
            "sb_apple_script_create_with_source",
            error,
            Self::from_raw,
        )
    }

    /// Loads an `NSAppleScript` from a file path or URL.
    pub fn with_contents_of_url(path_or_url: &str) -> Result<Self> {
        let path_or_url = c_string(path_or_url, "sb_apple_script_create_with_contents_of_url")?;
        let mut error = std::ptr::null_mut();
        let raw = unsafe {
            ffi::apple_script::sb_apple_script_create_with_contents_of_url(
                path_or_url.as_ptr(),
                &mut error,
            )
        };
        required_handle(
            raw,
            "sb_apple_script_create_with_contents_of_url",
            error,
            Self::from_raw,
        )
    }

    /// Returns the source text held by this `NSAppleScript`.
    pub fn source(&self) -> Option<String> {
        let raw = unsafe { ffi::apple_script::sb_apple_script_source(self.0.as_ptr()) };
        take_optional_c_string(raw)
    }

    /// Returns whether this `NSAppleScript` has been compiled.
    pub fn is_compiled(&self) -> bool {
        unsafe { ffi::apple_script::sb_apple_script_is_compiled(self.0.as_ptr()) }
    }

    /// Compiles this `NSAppleScript`.
    pub fn compile(&self) -> Result<()> {
        let mut error = std::ptr::null_mut();
        let ok = unsafe { ffi::apple_script::sb_apple_script_compile(self.0.as_ptr(), &mut error) };
        bool_result(ok, "sb_apple_script_compile", error)
    }

    /// Executes this `NSAppleScript` and returns any result descriptor.
    pub fn execute(&self) -> Result<Option<AppleEventDescriptor>> {
        let mut error = std::ptr::null_mut();
        let raw =
            unsafe { ffi::apple_script::sb_apple_script_execute(self.0.as_ptr(), &mut error) };
        optional_handle(
            raw,
            "sb_apple_script_execute",
            error,
            AppleEventDescriptor::from_raw,
        )
    }

    /// Executes this `NSAppleScript` with an explicit Apple event.
    pub fn execute_apple_event(
        &self,
        event: &AppleEventDescriptor,
    ) -> Result<Option<AppleEventDescriptor>> {
        let mut error = std::ptr::null_mut();
        let raw = unsafe {
            ffi::apple_script::sb_apple_script_execute_apple_event(
                self.0.as_ptr(),
                event.as_ptr(),
                &mut error,
            )
        };
        optional_handle(
            raw,
            "sb_apple_script_execute_apple_event",
            error,
            AppleEventDescriptor::from_raw,
        )
    }

    fn from_raw(handle: NonNull<c_void>) -> Self {
        Self(handle)
    }
}

impl Drop for AppleScript {
    fn drop(&mut self) {
        unsafe { ffi::apple_script::sb_apple_script_release(self.0.as_ptr()) };
    }
}
