use std::{
    ffi::{c_char, c_void, CStr, CString},
    ptr::NonNull,
};

use crate::{ffi, Result, ScriptingBridgeError};

#[derive(Debug)]
pub struct Application(NonNull<c_void>);

impl Application {
    pub fn with_bundle_identifier(bundle_identifier: &str) -> Result<Self> {
        let bundle_identifier = c_string(bundle_identifier, "sb_application_create")?;
        let mut error = std::ptr::null_mut();
        let raw = unsafe { ffi::sb_application_create(bundle_identifier.as_ptr(), &mut error) };
        NonNull::new(raw)
            .map(Self)
            .ok_or_else(|| bridge_error("sb_application_create", error))
    }

    pub fn is_running(&self) -> bool {
        unsafe { ffi::sb_application_is_running(self.0.as_ptr()) }
    }

    pub fn launch(&self) -> Result<()> {
        let mut error = std::ptr::null_mut();
        let ok = unsafe { ffi::sb_application_launch(self.0.as_ptr(), &mut error) };
        if ok {
            Ok(())
        } else {
            Err(bridge_error("sb_application_launch", error))
        }
    }

    pub fn activate(&self) -> Result<()> {
        let mut error = std::ptr::null_mut();
        let ok = unsafe { ffi::sb_application_activate(self.0.as_ptr(), &mut error) };
        if ok {
            Ok(())
        } else {
            Err(bridge_error("sb_application_activate", error))
        }
    }

    pub fn quit(&self) -> Result<()> {
        let mut error = std::ptr::null_mut();
        let ok = unsafe { ffi::sb_application_quit(self.0.as_ptr(), &mut error) };
        if ok {
            Ok(())
        } else {
            Err(bridge_error("sb_application_quit", error))
        }
    }

    pub fn terminate(&self) -> Result<()> {
        let mut error = std::ptr::null_mut();
        let ok = unsafe { ffi::sb_application_terminate(self.0.as_ptr(), &mut error) };
        if ok {
            Ok(())
        } else {
            Err(bridge_error("sb_application_terminate", error))
        }
    }

    pub fn tell(&self, command: &str, args: &[&str]) -> Result<Option<String>> {
        if args.len() > 1 {
            return Err(ScriptingBridgeError::new(
                "sb_application_tell",
                "only zero or one tell() arguments are supported in v0.1.0",
            ));
        }

        let command = c_string(command, "sb_application_tell")?;
        let argument = args
            .first()
            .map(|value| c_string(value, "sb_application_tell"))
            .transpose()?;
        let mut error = std::ptr::null_mut();
        let raw = unsafe {
            ffi::sb_application_tell(
                self.0.as_ptr(),
                command.as_ptr(),
                argument
                    .as_ref()
                    .map_or(std::ptr::null(), |value| value.as_ptr()),
                &mut error,
            )
        };
        if raw.is_null() {
            if error.is_null() {
                Ok(None)
            } else {
                Err(bridge_error("sb_application_tell", error))
            }
        } else {
            Ok(Some(take_c_string(raw)))
        }
    }

    pub fn object_for_key_path(&self, key_path: &str) -> Result<Option<ScriptObject>> {
        let key_path = c_string(key_path, "sb_application_object_for_key_path")?;
        let mut error = std::ptr::null_mut();
        let raw = unsafe {
            ffi::sb_application_object_for_key_path(self.0.as_ptr(), key_path.as_ptr(), &mut error)
        };
        NonNull::new(raw).map_or_else(
            || {
                if error.is_null() {
                    Ok(None)
                } else {
                    Err(bridge_error("sb_application_object_for_key_path", error))
                }
            },
            |raw| Ok(Some(ScriptObject(raw))),
        )
    }

    pub fn element_array_for_key_path(&self, key_path: &str) -> Result<Option<ElementArray>> {
        let key_path = c_string(key_path, "sb_application_element_array_for_key_path")?;
        let mut error = std::ptr::null_mut();
        let raw = unsafe {
            ffi::sb_application_element_array_for_key_path(
                self.0.as_ptr(),
                key_path.as_ptr(),
                &mut error,
            )
        };
        NonNull::new(raw).map_or_else(
            || {
                if error.is_null() {
                    Ok(None)
                } else {
                    Err(bridge_error(
                        "sb_application_element_array_for_key_path",
                        error,
                    ))
                }
            },
            |raw| Ok(Some(ElementArray(raw))),
        )
    }
}

impl Drop for Application {
    fn drop(&mut self) {
        unsafe { ffi::sb_application_release(self.0.as_ptr()) };
    }
}

#[derive(Debug)]
pub struct ScriptObject(NonNull<c_void>);

impl ScriptObject {
    pub fn description(&self) -> Option<String> {
        let raw = unsafe { ffi::sb_object_description(self.0.as_ptr()) };
        (!raw.is_null()).then(|| take_c_string(raw))
    }

    pub fn get_description(&self) -> Option<String> {
        let raw = unsafe { ffi::sb_object_get_description(self.0.as_ptr()) };
        (!raw.is_null()).then(|| take_c_string(raw))
    }

    pub fn last_error_description(&self) -> Option<String> {
        let raw = unsafe { ffi::sb_object_last_error_description(self.0.as_ptr()) };
        (!raw.is_null()).then(|| take_c_string(raw))
    }
}

impl Drop for ScriptObject {
    fn drop(&mut self) {
        unsafe { ffi::sb_object_release(self.0.as_ptr()) };
    }
}

#[derive(Debug)]
pub struct ElementArray(NonNull<c_void>);

impl ElementArray {
    pub fn description(&self) -> Option<String> {
        let raw = unsafe { ffi::sb_element_array_description(self.0.as_ptr()) };
        (!raw.is_null()).then(|| take_c_string(raw))
    }

    pub fn get_description(&self) -> Option<String> {
        let raw = unsafe { ffi::sb_element_array_get_description(self.0.as_ptr()) };
        (!raw.is_null()).then(|| take_c_string(raw))
    }
}

impl Drop for ElementArray {
    fn drop(&mut self) {
        unsafe { ffi::sb_element_array_release(self.0.as_ptr()) };
    }
}

fn c_string(value: &str, function: &'static str) -> Result<CString> {
    CString::new(value).map_err(|_| {
        ScriptingBridgeError::new(function, "strings cannot contain interior NUL bytes")
    })
}

fn take_c_string(raw: *mut c_char) -> String {
    let string = unsafe { CStr::from_ptr(raw) }
        .to_string_lossy()
        .into_owned();
    unsafe { ffi::sb_string_free(raw) };
    string
}

fn bridge_error(function: &'static str, error: *mut c_char) -> ScriptingBridgeError {
    if error.is_null() {
        return ScriptingBridgeError::new(function, "operation failed without an error message");
    }
    ScriptingBridgeError::new(function, take_c_string(error))
}
