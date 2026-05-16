use std::ffi::{c_char, c_void, CString};
use std::ptr::NonNull;

use crate::apple_event_descriptor::{AEEventClass, AEEventID, AppleEventDescriptor, DescType};
use crate::application::ScriptingClass;
use crate::element_array::ElementArray;
use crate::ffi;
use crate::internal::{c_string, optional_handle, required_handle, take_optional_c_string};
use crate::Result;

#[derive(Clone, Copy, Debug)]
pub struct Property<'a> {
    pub name: &'a str,
    pub value: &'a AppleEventDescriptor,
}

impl<'a> Property<'a> {
    pub const fn new(name: &'a str, value: &'a AppleEventDescriptor) -> Self {
        Self { name, value }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct EventParameter<'a> {
    pub code: DescType,
    pub value: &'a AppleEventDescriptor,
}

impl<'a> EventParameter<'a> {
    pub const fn new(code: DescType, value: &'a AppleEventDescriptor) -> Self {
        Self { code, value }
    }
}

#[derive(Debug)]
pub struct ScriptObject(pub(crate) NonNull<c_void>);

struct PropertyBuffers {
    _names: Vec<CString>,
    name_ptrs: Vec<*const c_char>,
    values: Vec<*mut c_void>,
}

impl ScriptObject {
    pub fn new() -> Result<Self> {
        let mut error = std::ptr::null_mut();
        let raw = unsafe { ffi::object::sb_object_create(&mut error) };
        required_handle(raw, "sb_object_create", error, Self)
    }

    pub fn with_properties(properties: &[Property<'_>]) -> Result<Self> {
        let buffers = property_buffers(properties, "sb_object_create_with_properties")?;
        let count = i64::try_from(properties.len()).map_err(|_| {
            crate::ScriptingBridgeError::new("sb_object_create_with_properties", "too many properties")
        })?;
        let mut error = std::ptr::null_mut();
        let raw = unsafe {
            ffi::object::sb_object_create_with_properties(
                buffers.name_ptrs.as_ptr(),
                buffers.values.as_ptr(),
                count,
                &mut error,
            )
        };
        required_handle(raw, "sb_object_create_with_properties", error, Self::from_raw)
    }

    pub fn with_data(data: &AppleEventDescriptor) -> Result<Self> {
        let mut error = std::ptr::null_mut();
        let raw = unsafe { ffi::object::sb_object_create_with_data(data.as_ptr(), &mut error) };
        required_handle(raw, "sb_object_create_with_data", error, Self::from_raw)
    }

    pub fn with_element_code(
        element_code: DescType,
        properties: &[Property<'_>],
        data: Option<&AppleEventDescriptor>,
    ) -> Result<Self> {
        let buffers = property_buffers(properties, "sb_object_create_with_element_code")?;
        let count = i64::try_from(properties.len()).map_err(|_| {
            crate::ScriptingBridgeError::new("sb_object_create_with_element_code", "too many properties")
        })?;
        let mut error = std::ptr::null_mut();
        let raw = unsafe {
            ffi::object::sb_object_create_with_element_code(
                element_code,
                buffers.name_ptrs.as_ptr(),
                buffers.values.as_ptr(),
                count,
                data.map_or(std::ptr::null_mut(), AppleEventDescriptor::as_ptr),
                &mut error,
            )
        };
        required_handle(raw, "sb_object_create_with_element_code", error, Self::from_raw)
    }

    pub fn get(&self) -> Result<Option<AppleEventDescriptor>> {
        let mut error = std::ptr::null_mut();
        let raw = unsafe { ffi::object::sb_object_get(self.0.as_ptr(), &mut error) };
        optional_handle(raw, "sb_object_get", error, AppleEventDescriptor::from_raw)
    }

    pub fn description(&self) -> Option<String> {
        let raw = unsafe { ffi::object::sb_object_description(self.0.as_ptr()) };
        take_optional_c_string(raw)
    }

    pub fn get_description(&self) -> Option<String> {
        let raw = unsafe { ffi::object::sb_object_get_description(self.0.as_ptr()) };
        take_optional_c_string(raw)
    }

    pub fn last_error_description(&self) -> Option<String> {
        let raw = unsafe { ffi::object::sb_object_last_error_description(self.0.as_ptr()) };
        take_optional_c_string(raw)
    }

    pub fn property_with_code(&self, code: DescType) -> Result<Option<Self>> {
        let mut error = std::ptr::null_mut();
        let raw = unsafe { ffi::object::sb_object_property_with_code(self.0.as_ptr(), code, &mut error) };
        optional_handle(raw, "sb_object_property_with_code", error, Self::from_raw)
    }

    pub fn property_with_class(
        &self,
        class: &ScriptingClass,
        code: DescType,
    ) -> Result<Option<Self>> {
        let mut error = std::ptr::null_mut();
        let raw = unsafe {
            ffi::object::sb_object_property_with_class(
                self.0.as_ptr(),
                class.as_ptr(),
                code,
                &mut error,
            )
        };
        optional_handle(raw, "sb_object_property_with_class", error, Self::from_raw)
    }

    pub fn element_array_with_code(&self, code: DescType) -> Result<Option<ElementArray>> {
        let mut error = std::ptr::null_mut();
        let raw = unsafe { ffi::object::sb_object_element_array_with_code(self.0.as_ptr(), code, &mut error) };
        optional_handle(raw, "sb_object_element_array_with_code", error, ElementArray::from_raw)
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
            crate::ScriptingBridgeError::new("sb_object_send_event", "too many Apple event parameters")
        })?;
        let mut error = std::ptr::null_mut();
        let raw = unsafe {
            ffi::object::sb_object_send_event(
                self.0.as_ptr(),
                event_class,
                event_id,
                codes.as_ptr(),
                values.as_ptr(),
                count,
                &mut error,
            )
        };
        optional_handle(raw, "sb_object_send_event", error, AppleEventDescriptor::from_raw)
    }

    pub fn set_to(&self, value: Option<&AppleEventDescriptor>) -> Result<()> {
        let mut error = std::ptr::null_mut();
        let ok = unsafe {
            ffi::object::sb_object_set_to(
                self.0.as_ptr(),
                value.map_or(std::ptr::null_mut(), AppleEventDescriptor::as_ptr),
                &mut error,
            )
        };
        crate::internal::bool_result(ok, "sb_object_set_to", error)
    }

    pub(crate) fn from_raw(handle: NonNull<c_void>) -> Self {
        Self(handle)
    }
}

impl Drop for ScriptObject {
    fn drop(&mut self) {
        unsafe { ffi::object::sb_object_release(self.0.as_ptr()) };
    }
}

fn property_buffers(properties: &[Property<'_>], function: &'static str) -> Result<PropertyBuffers> {
    let names = properties
        .iter()
        .map(|property| c_string(property.name, function))
        .collect::<Result<Vec<_>>>()?;
    let name_ptrs = names.iter().map(|name| name.as_ptr()).collect::<Vec<_>>();
    let values = properties
        .iter()
        .map(|property| property.value.as_ptr())
        .collect::<Vec<_>>();

    Ok(PropertyBuffers {
        _names: names,
        name_ptrs,
        values,
    })
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
