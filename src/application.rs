use std::ffi::{c_char, c_void};
use std::ptr::NonNull;

use crate::apple_event_descriptor::{AEEventClass, AEEventID, AppleEventDescriptor};
use crate::application_delegate::ApplicationDelegate;
use crate::element_array::ElementArray;
use crate::ffi;
use crate::internal::{bool_result, c_string, optional_handle, required_handle, take_c_string};
use crate::object::{EventParameter, ScriptObject};
use crate::Result;

pub type LaunchFlags = u32;
pub type SendMode = i32;

#[derive(Debug)]
pub struct Application(NonNull<c_void>);

#[derive(Debug)]
pub struct ScriptingClass(NonNull<c_void>);

impl Application {
    pub fn with_bundle_identifier(bundle_identifier: &str) -> Result<Self> {
        create_application(
            bundle_identifier,
            "sb_application_create_with_bundle_identifier",
            |value, error| unsafe {
                ffi::application::sb_application_create_with_bundle_identifier(
                    value.as_ptr(),
                    error,
                )
            },
        )
    }

    pub fn with_url(url: &str) -> Result<Self> {
        create_application(
            url,
            "sb_application_create_with_url",
            |value, error| unsafe {
                ffi::application::sb_application_create_with_url(value.as_ptr(), error)
            },
        )
    }

    pub fn with_process_identifier(process_identifier: i32) -> Result<Self> {
        let mut error = std::ptr::null_mut();
        let raw = unsafe {
            ffi::application::sb_application_create_with_process_identifier(
                process_identifier,
                &mut error,
            )
        };
        required_handle(
            raw,
            "sb_application_create_with_process_identifier",
            error,
            Self,
        )
    }

    pub fn shared_with_bundle_identifier(bundle_identifier: &str) -> Result<Self> {
        create_application(
            bundle_identifier,
            "sb_application_shared_with_bundle_identifier",
            |value, error| unsafe {
                ffi::application::sb_application_shared_with_bundle_identifier(
                    value.as_ptr(),
                    error,
                )
            },
        )
    }

    pub fn shared_with_url(url: &str) -> Result<Self> {
        create_application(
            url,
            "sb_application_shared_with_url",
            |value, error| unsafe {
                ffi::application::sb_application_shared_with_url(value.as_ptr(), error)
            },
        )
    }

    pub fn shared_with_process_identifier(process_identifier: i32) -> Result<Self> {
        let mut error = std::ptr::null_mut();
        let raw = unsafe {
            ffi::application::sb_application_shared_with_process_identifier(
                process_identifier,
                &mut error,
            )
        };
        required_handle(
            raw,
            "sb_application_shared_with_process_identifier",
            error,
            Self,
        )
    }

    pub fn as_object(&self) -> Result<ScriptObject> {
        let raw = unsafe { ffi::application::sb_application_copy_object(self.0.as_ptr()) };
        required_handle(
            raw,
            "sb_application_copy_object",
            std::ptr::null_mut(),
            ScriptObject::from_raw,
        )
    }

    pub fn class_for_scripting_class(&self, class_name: &str) -> Result<Option<ScriptingClass>> {
        let class_name = c_string(class_name, "sb_application_class_for_scripting_class")?;
        let mut error = std::ptr::null_mut();
        let raw = unsafe {
            ffi::application::sb_application_class_for_scripting_class(
                self.0.as_ptr(),
                class_name.as_ptr(),
                &mut error,
            )
        };
        optional_handle(
            raw,
            "sb_application_class_for_scripting_class",
            error,
            ScriptingClass,
        )
    }

    pub fn is_running(&self) -> bool {
        unsafe { ffi::application::sb_application_is_running(self.0.as_ptr()) }
    }

    pub fn process_identifier(&self) -> Option<i32> {
        let process_identifier =
            unsafe { ffi::application::sb_application_process_identifier(self.0.as_ptr()) };
        (process_identifier >= 0).then_some(process_identifier)
    }

    pub fn launch(&self) -> Result<()> {
        let mut error = std::ptr::null_mut();
        let ok = unsafe { ffi::application::sb_application_launch(self.0.as_ptr(), &mut error) };
        bool_result(ok, "sb_application_launch", error)
    }

    pub fn activate(&self) -> Result<()> {
        let mut error = std::ptr::null_mut();
        let ok = unsafe { ffi::application::sb_application_activate(self.0.as_ptr(), &mut error) };
        bool_result(ok, "sb_application_activate", error)
    }

    pub fn quit(&self) -> Result<()> {
        let mut error = std::ptr::null_mut();
        let ok = unsafe { ffi::application::sb_application_quit(self.0.as_ptr(), &mut error) };
        bool_result(ok, "sb_application_quit", error)
    }

    pub fn terminate(&self) -> Result<()> {
        let mut error = std::ptr::null_mut();
        let ok = unsafe { ffi::application::sb_application_terminate(self.0.as_ptr(), &mut error) };
        bool_result(ok, "sb_application_terminate", error)
    }

    pub fn launch_flags(&self) -> LaunchFlags {
        unsafe { ffi::application::sb_application_get_launch_flags(self.0.as_ptr()) }
    }

    pub fn set_launch_flags(&self, launch_flags: LaunchFlags) -> Result<()> {
        let mut error = std::ptr::null_mut();
        let ok = unsafe {
            ffi::application::sb_application_set_launch_flags(
                self.0.as_ptr(),
                launch_flags,
                &mut error,
            )
        };
        bool_result(ok, "sb_application_set_launch_flags", error)
    }

    pub fn send_mode(&self) -> SendMode {
        unsafe { ffi::application::sb_application_get_send_mode(self.0.as_ptr()) }
    }

    pub fn set_send_mode(&self, send_mode: SendMode) -> Result<()> {
        let mut error = std::ptr::null_mut();
        let ok = unsafe {
            ffi::application::sb_application_set_send_mode(self.0.as_ptr(), send_mode, &mut error)
        };
        bool_result(ok, "sb_application_set_send_mode", error)
    }

    pub fn timeout(&self) -> i64 {
        unsafe { ffi::application::sb_application_get_timeout(self.0.as_ptr()) }
    }

    pub fn set_timeout(&self, timeout: i64) -> Result<()> {
        let mut error = std::ptr::null_mut();
        let ok = unsafe {
            ffi::application::sb_application_set_timeout(self.0.as_ptr(), timeout, &mut error)
        };
        bool_result(ok, "sb_application_set_timeout", error)
    }

    pub fn set_delegate(&self, delegate: Option<&ApplicationDelegate>) -> Result<()> {
        let mut error = std::ptr::null_mut();
        let ok = unsafe {
            ffi::application::sb_application_set_delegate(
                self.0.as_ptr(),
                delegate.map_or(std::ptr::null_mut(), ApplicationDelegate::as_ptr),
                &mut error,
            )
        };
        bool_result(ok, "sb_application_set_delegate", error)
    }

    pub fn has_delegate(&self) -> bool {
        unsafe { ffi::application::sb_application_has_delegate(self.0.as_ptr()) }
    }

    pub fn tell(&self, command: &str, args: &[&str]) -> Result<Option<String>> {
        if args.len() > 1 {
            return Err(crate::ScriptingBridgeError::new(
                "sb_application_tell",
                "only zero or one tell() arguments are supported",
            ));
        }

        let command = c_string(command, "sb_application_tell")?;
        let argument = args
            .first()
            .map(|value| c_string(value, "sb_application_tell"))
            .transpose()?;
        let mut error = std::ptr::null_mut();
        let raw = unsafe {
            ffi::application::sb_application_tell(
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
                Err(crate::internal::bridge_error("sb_application_tell", error))
            }
        } else {
            Ok(Some(take_c_string(raw)))
        }
    }

    pub fn send_event(
        &self,
        event_class: AEEventClass,
        event_id: AEEventID,
        parameters: &[EventParameter<'_>],
    ) -> Result<Option<AppleEventDescriptor>> {
        let codes = parameter_codes(parameters);
        let values = parameter_values(parameters);
        let count = i64::try_from(parameters.len()).map_err(|_| {
            crate::ScriptingBridgeError::new(
                "sb_application_send_event",
                "too many Apple event parameters",
            )
        })?;
        let mut error = std::ptr::null_mut();
        let raw = unsafe {
            ffi::application::sb_application_send_event(
                self.0.as_ptr(),
                event_class,
                event_id,
                codes.as_ptr(),
                values.as_ptr(),
                count,
                &mut error,
            )
        };
        optional_handle(
            raw,
            "sb_application_send_event",
            error,
            AppleEventDescriptor::from_raw,
        )
    }

    pub fn object_for_key_path(&self, key_path: &str) -> Result<Option<ScriptObject>> {
        let key_path = c_string(key_path, "sb_application_object_for_key_path")?;
        let mut error = std::ptr::null_mut();
        let raw = unsafe {
            ffi::application::sb_application_object_for_key_path(
                self.0.as_ptr(),
                key_path.as_ptr(),
                &mut error,
            )
        };
        optional_handle(
            raw,
            "sb_application_object_for_key_path",
            error,
            ScriptObject::from_raw,
        )
    }

    pub fn element_array_for_key_path(&self, key_path: &str) -> Result<Option<ElementArray>> {
        let key_path = c_string(key_path, "sb_application_element_array_for_key_path")?;
        let mut error = std::ptr::null_mut();
        let raw = unsafe {
            ffi::application::sb_application_element_array_for_key_path(
                self.0.as_ptr(),
                key_path.as_ptr(),
                &mut error,
            )
        };
        optional_handle(
            raw,
            "sb_application_element_array_for_key_path",
            error,
            ElementArray::from_raw,
        )
    }
}

impl ScriptingClass {
    pub fn name(&self) -> Option<String> {
        let raw = unsafe { ffi::application::sb_scripting_class_name(self.0.as_ptr()) };
        crate::internal::take_optional_c_string(raw)
    }

    pub(crate) fn as_ptr(&self) -> *mut c_void {
        self.0.as_ptr()
    }
}

impl Drop for Application {
    fn drop(&mut self) {
        unsafe { ffi::application::sb_application_release(self.0.as_ptr()) };
    }
}

impl Drop for ScriptingClass {
    fn drop(&mut self) {
        unsafe { ffi::application::sb_scripting_class_release(self.0.as_ptr()) };
    }
}

fn create_application(
    value: &str,
    function: &'static str,
    create: impl FnOnce(&std::ffi::CString, *mut *mut c_char) -> *mut c_void,
) -> Result<Application> {
    let value = c_string(value, function)?;
    let mut error = std::ptr::null_mut();
    let raw = create(&value, &mut error);
    required_handle(raw, function, error, Application)
}

fn parameter_codes(parameters: &[EventParameter<'_>]) -> Vec<u32> {
    parameters.iter().map(|parameter| parameter.code).collect()
}

fn parameter_values(parameters: &[EventParameter<'_>]) -> Vec<*mut c_void> {
    parameters
        .iter()
        .map(|parameter| parameter.value.as_ptr())
        .collect()
}
