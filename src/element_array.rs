use std::ffi::c_void;
use std::ptr::NonNull;

use crate::apple_event_descriptor::AppleEventDescriptor;
use crate::ffi;
use crate::internal::{c_string, optional_handle, take_optional_c_string};
use crate::object::ScriptObject;
use crate::Result;

#[derive(Debug)]
pub struct ElementArray(pub(crate) NonNull<c_void>);

impl ElementArray {
    pub fn object_with_name(&self, name: &str) -> Result<Option<ScriptObject>> {
        let name = c_string(name, "sb_element_array_object_with_name")?;
        let mut error = std::ptr::null_mut();
        let raw = unsafe {
            ffi::element_array::sb_element_array_object_with_name(
                self.0.as_ptr(),
                name.as_ptr(),
                &mut error,
            )
        };
        optional_handle(
            raw,
            "sb_element_array_object_with_name",
            error,
            ScriptObject::from_raw,
        )
    }

    pub fn object_with_id(
        &self,
        identifier: &AppleEventDescriptor,
    ) -> Result<Option<ScriptObject>> {
        let mut error = std::ptr::null_mut();
        let raw = unsafe {
            ffi::element_array::sb_element_array_object_with_id(
                self.0.as_ptr(),
                identifier.as_ptr(),
                &mut error,
            )
        };
        optional_handle(
            raw,
            "sb_element_array_object_with_id",
            error,
            ScriptObject::from_raw,
        )
    }

    pub fn object_at_location(
        &self,
        location: &AppleEventDescriptor,
    ) -> Result<Option<ScriptObject>> {
        let mut error = std::ptr::null_mut();
        let raw = unsafe {
            ffi::element_array::sb_element_array_object_at_location(
                self.0.as_ptr(),
                location.as_ptr(),
                &mut error,
            )
        };
        optional_handle(
            raw,
            "sb_element_array_object_at_location",
            error,
            ScriptObject::from_raw,
        )
    }

    pub fn array_by_applying_selector(
        &self,
        selector: &str,
    ) -> Result<Option<AppleEventDescriptor>> {
        let selector = c_string(selector, "sb_element_array_array_by_applying_selector")?;
        let mut error = std::ptr::null_mut();
        let raw = unsafe {
            ffi::element_array::sb_element_array_array_by_applying_selector(
                self.0.as_ptr(),
                selector.as_ptr(),
                &mut error,
            )
        };
        optional_handle(
            raw,
            "sb_element_array_array_by_applying_selector",
            error,
            AppleEventDescriptor::from_raw,
        )
    }

    pub fn array_by_applying_selector_with_object(
        &self,
        selector: &str,
        argument: &AppleEventDescriptor,
    ) -> Result<Option<AppleEventDescriptor>> {
        let selector = c_string(
            selector,
            "sb_element_array_array_by_applying_selector_with_object",
        )?;
        let mut error = std::ptr::null_mut();
        let raw = unsafe {
            ffi::element_array::sb_element_array_array_by_applying_selector_with_object(
                self.0.as_ptr(),
                selector.as_ptr(),
                argument.as_ptr(),
                &mut error,
            )
        };
        optional_handle(
            raw,
            "sb_element_array_array_by_applying_selector_with_object",
            error,
            AppleEventDescriptor::from_raw,
        )
    }

    pub fn get(&self) -> Result<Option<AppleEventDescriptor>> {
        let mut error = std::ptr::null_mut();
        let raw = unsafe { ffi::element_array::sb_element_array_get(self.0.as_ptr(), &mut error) };
        optional_handle(
            raw,
            "sb_element_array_get",
            error,
            AppleEventDescriptor::from_raw,
        )
    }

    pub fn description(&self) -> Option<String> {
        let raw = unsafe { ffi::element_array::sb_element_array_description(self.0.as_ptr()) };
        take_optional_c_string(raw)
    }

    pub fn get_description(&self) -> Option<String> {
        let raw = unsafe { ffi::element_array::sb_element_array_get_description(self.0.as_ptr()) };
        take_optional_c_string(raw)
    }

    pub(crate) fn from_raw(handle: NonNull<c_void>) -> Self {
        Self(handle)
    }
}

impl Drop for ElementArray {
    fn drop(&mut self) {
        unsafe { ffi::element_array::sb_element_array_release(self.0.as_ptr()) };
    }
}
