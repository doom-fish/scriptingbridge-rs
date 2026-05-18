use std::ffi::c_void;
use std::ptr::NonNull;

use crate::ffi;
use crate::internal::{
    bool_result, c_string, optional_handle, required_handle, take_bytes, take_optional_c_string,
};
use crate::Result;

/// Mirrors the Apple Event Manager descriptor type used by `NSAppleEventDescriptor`.
pub type DescType = u32;
/// Re-exports the Core Foundation `OSType` used by Scripting Bridge and Apple events.
pub use apple_cf::raw::OSType;
/// Mirrors the Apple event keyword type used by `NSAppleEventDescriptor`.
pub type AEKeyword = u32;
/// Mirrors the Apple event class code used by `NSAppleEventDescriptor`.
pub type AEEventClass = u32;
/// Mirrors the Apple event ID code used by `NSAppleEventDescriptor`.
pub type AEEventID = u32;
/// Mirrors the Apple event return ID used by `NSAppleEventDescriptor`.
pub type AEReturnID = i16;
/// Mirrors the Apple event transaction ID used by `NSAppleEventDescriptor`.
pub type AETransactionID = i32;
/// Mirrors the Apple Event Manager send-option bitfield used by `NSAppleEventDescriptor`.
pub type AppleEventSendOptions = u64;

/// Matches the Apple Event Manager `kAENoReply` send option.
pub const APPLE_EVENT_SEND_NO_REPLY: AppleEventSendOptions = 0x0000_0001;
/// Matches the Apple Event Manager `kAEQueueReply` send option.
pub const APPLE_EVENT_SEND_QUEUE_REPLY: AppleEventSendOptions = 0x0000_0002;
/// Matches the Apple Event Manager `kAEWaitReply` send option.
pub const APPLE_EVENT_SEND_WAIT_FOR_REPLY: AppleEventSendOptions = 0x0000_0003;
/// Matches the Apple Event Manager `kAENeverInteract` send option.
pub const APPLE_EVENT_SEND_NEVER_INTERACT: AppleEventSendOptions = 0x0000_0010;
/// Matches the Apple Event Manager `kAECanInteract` send option.
pub const APPLE_EVENT_SEND_CAN_INTERACT: AppleEventSendOptions = 0x0000_0020;
/// Matches the Apple Event Manager `kAEAlwaysInteract` send option.
pub const APPLE_EVENT_SEND_ALWAYS_INTERACT: AppleEventSendOptions = 0x0000_0030;
/// Matches the Apple Event Manager `kAECanSwitchLayer` send option.
pub const APPLE_EVENT_SEND_CAN_SWITCH_LAYER: AppleEventSendOptions = 0x0000_0040;
/// Matches the Apple Event Manager `kAEDontRecord` send option.
pub const APPLE_EVENT_SEND_DONT_RECORD: AppleEventSendOptions = 0x0000_1000;
/// Matches the Apple Event Manager `kAEDontExecute` send option.
pub const APPLE_EVENT_SEND_DONT_EXECUTE: AppleEventSendOptions = 0x0000_2000;
/// Matches the Apple Event Manager `kAEDontAnnotate` send option.
pub const APPLE_EVENT_SEND_DONT_ANNOTATE: AppleEventSendOptions = 0x0001_0000;
/// Uses the default wait-for-reply and can-interact send options.
pub const APPLE_EVENT_SEND_DEFAULT_OPTIONS: AppleEventSendOptions =
    APPLE_EVENT_SEND_WAIT_FOR_REPLY | APPLE_EVENT_SEND_CAN_INTERACT;

/// Wraps an `NSAppleEventDescriptor` instance.
#[derive(Debug)]
pub struct AppleEventDescriptor(NonNull<c_void>);

/// Owns a raw `AEDesc` copied from `NSAppleEventDescriptor`.
#[derive(Debug)]
pub struct RawAppleEventDescriptor(NonNull<c_void>);

impl AppleEventDescriptor {
    /// Creates the `NSAppleEventDescriptor` null descriptor.
    pub fn null() -> Result<Self> {
        // SAFETY: sb_apple_event_descriptor_null returns a valid handle or null.
        // No error pointer is needed for this simple factory function.
        let raw = unsafe { ffi::apple_event_descriptor::sb_apple_event_descriptor_null() };
        required_handle(
            raw,
            "sb_apple_event_descriptor_null",
            std::ptr::null_mut(),
            Self::from_raw,
        )
    }

    /// Creates an `NSAppleEventDescriptor` from raw descriptor bytes.
    pub fn with_descriptor_type_and_bytes(descriptor_type: DescType, bytes: &[u8]) -> Result<Self> {
        descriptor_from_bytes(
            descriptor_type,
            bytes,
            "sb_apple_event_descriptor_create_with_descriptor_type_bytes",
            |descriptor_type, pointer, length, error| unsafe {
                ffi::apple_event_descriptor::sb_apple_event_descriptor_create_with_descriptor_type_bytes(
                    descriptor_type,
                    pointer,
                    length,
                    error,
                )
            },
        )
    }

    /// Creates an `NSAppleEventDescriptor` from raw descriptor data.
    pub fn with_descriptor_type_and_data(descriptor_type: DescType, data: &[u8]) -> Result<Self> {
        descriptor_from_bytes(
            descriptor_type,
            data,
            "sb_apple_event_descriptor_create_with_descriptor_type_data",
            |descriptor_type, pointer, length, error| unsafe {
                ffi::apple_event_descriptor::sb_apple_event_descriptor_create_with_descriptor_type_data(
                    descriptor_type,
                    pointer,
                    length,
                    error,
                )
            },
        )
    }

    /// Creates an `NSAppleEventDescriptor` boolean descriptor.
    pub fn with_boolean(value: bool) -> Result<Self> {
        let raw = unsafe {
            ffi::apple_event_descriptor::sb_apple_event_descriptor_create_with_boolean(value)
        };
        required_handle(
            raw,
            "sb_apple_event_descriptor_create_with_boolean",
            std::ptr::null_mut(),
            Self::from_raw,
        )
    }

    /// Creates an `NSAppleEventDescriptor` enum-code descriptor.
    pub fn with_enum_code(value: OSType) -> Result<Self> {
        let raw = unsafe {
            ffi::apple_event_descriptor::sb_apple_event_descriptor_create_with_enum_code(value)
        };
        required_handle(
            raw,
            "sb_apple_event_descriptor_create_with_enum_code",
            std::ptr::null_mut(),
            Self::from_raw,
        )
    }

    /// Creates an `NSAppleEventDescriptor` 32-bit integer descriptor.
    pub fn with_int32(value: i32) -> Result<Self> {
        let raw = unsafe {
            ffi::apple_event_descriptor::sb_apple_event_descriptor_create_with_int32(value)
        };
        required_handle(
            raw,
            "sb_apple_event_descriptor_create_with_int32",
            std::ptr::null_mut(),
            Self::from_raw,
        )
    }

    /// Creates an `NSAppleEventDescriptor` floating-point descriptor.
    pub fn with_double(value: f64) -> Result<Self> {
        let raw = unsafe {
            ffi::apple_event_descriptor::sb_apple_event_descriptor_create_with_double(value)
        };
        required_handle(
            raw,
            "sb_apple_event_descriptor_create_with_double",
            std::ptr::null_mut(),
            Self::from_raw,
        )
    }

    /// Creates an `NSAppleEventDescriptor` type-code descriptor.
    pub fn with_type_code(value: OSType) -> Result<Self> {
        let raw = unsafe {
            ffi::apple_event_descriptor::sb_apple_event_descriptor_create_with_type_code(value)
        };
        required_handle(
            raw,
            "sb_apple_event_descriptor_create_with_type_code",
            std::ptr::null_mut(),
            Self::from_raw,
        )
    }

    /// Creates an `NSAppleEventDescriptor` string descriptor.
    pub fn with_string(value: &str) -> Result<Self> {
        let value = c_string(value, "sb_apple_event_descriptor_create_with_string")?;
        let raw = unsafe {
            ffi::apple_event_descriptor::sb_apple_event_descriptor_create_with_string(
                value.as_ptr(),
            )
        };
        required_handle(
            raw,
            "sb_apple_event_descriptor_create_with_string",
            std::ptr::null_mut(),
            Self::from_raw,
        )
    }

    /// Creates an `NSAppleEventDescriptor` date descriptor from a timestamp.
    pub fn with_date(timestamp_seconds: f64) -> Result<Self> {
        let raw = unsafe {
            ffi::apple_event_descriptor::sb_apple_event_descriptor_create_with_date(
                timestamp_seconds,
            )
        };
        required_handle(
            raw,
            "sb_apple_event_descriptor_create_with_date",
            std::ptr::null_mut(),
            Self::from_raw,
        )
    }

    /// Creates an `NSAppleEventDescriptor` file URL descriptor.
    pub fn with_file_url(path_or_url: &str) -> Result<Self> {
        let value = c_string(
            path_or_url,
            "sb_apple_event_descriptor_create_with_file_url",
        )?;
        let mut error = std::ptr::null_mut();
        let raw = unsafe {
            ffi::apple_event_descriptor::sb_apple_event_descriptor_create_with_file_url(
                value.as_ptr(),
                &mut error,
            )
        };
        required_handle(
            raw,
            "sb_apple_event_descriptor_create_with_file_url",
            error,
            Self::from_raw,
        )
    }

    /// Creates an `NSAppleEventDescriptor` Apple event header.
    pub fn apple_event(
        event_class: AEEventClass,
        event_id: AEEventID,
        target_descriptor: Option<&Self>,
        return_id: AEReturnID,
        transaction_id: AETransactionID,
    ) -> Result<Self> {
        let mut error = std::ptr::null_mut();
        let raw = unsafe {
            ffi::apple_event_descriptor::sb_apple_event_descriptor_create_apple_event(
                event_class,
                event_id,
                target_descriptor.map_or(std::ptr::null_mut(), Self::as_ptr),
                return_id,
                transaction_id,
                &mut error,
            )
        };
        required_handle(
            raw,
            "sb_apple_event_descriptor_create_apple_event",
            error,
            Self::from_raw,
        )
    }

    /// Creates an `NSAppleEventDescriptor` list descriptor.
    pub fn list() -> Result<Self> {
        let raw = unsafe { ffi::apple_event_descriptor::sb_apple_event_descriptor_create_list() };
        required_handle(
            raw,
            "sb_apple_event_descriptor_create_list",
            std::ptr::null_mut(),
            Self::from_raw,
        )
    }

    /// Creates an `NSAppleEventDescriptor` record descriptor.
    pub fn record() -> Result<Self> {
        let raw = unsafe { ffi::apple_event_descriptor::sb_apple_event_descriptor_create_record() };
        required_handle(
            raw,
            "sb_apple_event_descriptor_create_record",
            std::ptr::null_mut(),
            Self::from_raw,
        )
    }

    /// Creates the current-process target descriptor used by `NSAppleEventDescriptor`.
    pub fn current_process() -> Result<Self> {
        let raw =
            unsafe { ffi::apple_event_descriptor::sb_apple_event_descriptor_current_process() };
        required_handle(
            raw,
            "sb_apple_event_descriptor_current_process",
            std::ptr::null_mut(),
            Self::from_raw,
        )
    }

    /// Creates a process target `NSAppleEventDescriptor`.
    pub fn with_process_identifier(process_identifier: i32) -> Result<Self> {
        let raw = unsafe {
            ffi::apple_event_descriptor::sb_apple_event_descriptor_create_with_process_identifier(
                process_identifier,
            )
        };
        required_handle(
            raw,
            "sb_apple_event_descriptor_create_with_process_identifier",
            std::ptr::null_mut(),
            Self::from_raw,
        )
    }

    /// Creates an application target `NSAppleEventDescriptor` from a bundle identifier.
    pub fn with_bundle_identifier(bundle_identifier: &str) -> Result<Self> {
        let bundle_identifier = c_string(
            bundle_identifier,
            "sb_apple_event_descriptor_create_with_bundle_identifier",
        )?;
        let raw = unsafe {
            ffi::apple_event_descriptor::sb_apple_event_descriptor_create_with_bundle_identifier(
                bundle_identifier.as_ptr(),
            )
        };
        required_handle(
            raw,
            "sb_apple_event_descriptor_create_with_bundle_identifier",
            std::ptr::null_mut(),
            Self::from_raw,
        )
    }

    /// Creates an application target `NSAppleEventDescriptor` from a URL.
    pub fn with_application_url(url: &str) -> Result<Self> {
        let url = c_string(url, "sb_apple_event_descriptor_create_with_application_url")?;
        let mut error = std::ptr::null_mut();
        let raw = unsafe {
            ffi::apple_event_descriptor::sb_apple_event_descriptor_create_with_application_url(
                url.as_ptr(),
                &mut error,
            )
        };
        required_handle(
            raw,
            "sb_apple_event_descriptor_create_with_application_url",
            error,
            Self::from_raw,
        )
    }

    /// Copies this `NSAppleEventDescriptor` into an owned raw `AEDesc`.
    pub fn to_raw_aedesc(&self) -> Result<RawAppleEventDescriptor> {
        let mut error = std::ptr::null_mut();
        let raw = unsafe {
            ffi::apple_event_descriptor::sb_apple_event_descriptor_copy_aedesc(
                self.0.as_ptr(),
                &mut error,
            )
        };
        required_handle(
            raw,
            "sb_apple_event_descriptor_copy_aedesc",
            error,
            RawAppleEventDescriptor::from_raw,
        )
    }

    /// Wraps an owned raw `AEDesc` as an `NSAppleEventDescriptor` without copying.
    pub fn from_raw_aedesc_no_copy(raw_descriptor: RawAppleEventDescriptor) -> Result<Self> {
        let raw_descriptor = raw_descriptor.into_raw();
        let mut error = std::ptr::null_mut();
        let raw = unsafe {
            ffi::apple_event_descriptor::sb_apple_event_descriptor_create_with_aedesc_no_copy(
                raw_descriptor,
                &mut error,
            )
        };
        required_handle(
            raw,
            "sb_apple_event_descriptor_create_with_aedesc_no_copy",
            error,
            Self::from_raw,
        )
    }

    /// Returns the descriptor type reported by `NSAppleEventDescriptor`.
    pub fn descriptor_type(&self) -> DescType {
        unsafe {
            ffi::apple_event_descriptor::sb_apple_event_descriptor_descriptor_type(self.0.as_ptr())
        }
    }

    /// Returns the raw data payload held by `NSAppleEventDescriptor`.
    pub fn data(&self) -> Vec<u8> {
        let mut length = 0_i64;
        let raw = unsafe {
            ffi::apple_event_descriptor::sb_apple_event_descriptor_copy_data(
                self.0.as_ptr(),
                &mut length,
            )
        };
        take_bytes(raw, usize::try_from(length).unwrap_or_default())
    }

    /// Returns the boolean value stored in this `NSAppleEventDescriptor`.
    pub fn boolean_value(&self) -> bool {
        unsafe {
            ffi::apple_event_descriptor::sb_apple_event_descriptor_boolean_value(self.0.as_ptr())
        }
    }

    /// Returns the enum `OSType` stored in this `NSAppleEventDescriptor`.
    pub fn enum_code_value(&self) -> OSType {
        unsafe {
            ffi::apple_event_descriptor::sb_apple_event_descriptor_enum_code_value(self.0.as_ptr())
        }
    }

    /// Returns the 32-bit integer stored in this `NSAppleEventDescriptor`.
    pub fn int32_value(&self) -> i32 {
        unsafe {
            ffi::apple_event_descriptor::sb_apple_event_descriptor_int32_value(self.0.as_ptr())
        }
    }

    /// Returns the floating-point value stored in this `NSAppleEventDescriptor`.
    pub fn double_value(&self) -> f64 {
        unsafe {
            ffi::apple_event_descriptor::sb_apple_event_descriptor_double_value(self.0.as_ptr())
        }
    }

    /// Returns the type-code `OSType` stored in this `NSAppleEventDescriptor`.
    pub fn type_code_value(&self) -> OSType {
        unsafe {
            ffi::apple_event_descriptor::sb_apple_event_descriptor_type_code_value(self.0.as_ptr())
        }
    }

    /// Returns the string value stored in this `NSAppleEventDescriptor`.
    pub fn string_value(&self) -> Option<String> {
        let raw = unsafe {
            ffi::apple_event_descriptor::sb_apple_event_descriptor_string_value(self.0.as_ptr())
        };
        take_optional_c_string(raw)
    }

    /// Returns the date value stored in this `NSAppleEventDescriptor`.
    pub fn date_value(&self) -> Option<f64> {
        let value = unsafe {
            ffi::apple_event_descriptor::sb_apple_event_descriptor_date_value(self.0.as_ptr())
        };
        (!value.is_nan()).then_some(value)
    }

    /// Returns the file URL value stored in this `NSAppleEventDescriptor`.
    pub fn file_url_value(&self) -> Option<String> {
        let raw = unsafe {
            ffi::apple_event_descriptor::sb_apple_event_descriptor_file_url_value(self.0.as_ptr())
        };
        take_optional_c_string(raw)
    }

    /// Returns the Apple event class stored in this `NSAppleEventDescriptor`.
    pub fn event_class(&self) -> AEEventClass {
        unsafe {
            ffi::apple_event_descriptor::sb_apple_event_descriptor_event_class(self.0.as_ptr())
        }
    }

    /// Returns the Apple event ID stored in this `NSAppleEventDescriptor`.
    pub fn event_id(&self) -> AEEventID {
        unsafe { ffi::apple_event_descriptor::sb_apple_event_descriptor_event_id(self.0.as_ptr()) }
    }

    /// Returns the Apple event return ID stored in this `NSAppleEventDescriptor`.
    pub fn return_id(&self) -> AEReturnID {
        unsafe { ffi::apple_event_descriptor::sb_apple_event_descriptor_return_id(self.0.as_ptr()) }
    }

    /// Returns the Apple event transaction ID stored in this `NSAppleEventDescriptor`.
    pub fn transaction_id(&self) -> AETransactionID {
        unsafe {
            ffi::apple_event_descriptor::sb_apple_event_descriptor_transaction_id(self.0.as_ptr())
        }
    }

    /// Sets an Apple event parameter on this `NSAppleEventDescriptor`.
    pub fn set_param_descriptor(&self, descriptor: &Self, keyword: AEKeyword) -> Result<()> {
        descriptor_mutation(
            self,
            descriptor,
            keyword,
            "sb_apple_event_descriptor_set_param_descriptor",
            |handle, descriptor, keyword, error| unsafe {
                ffi::apple_event_descriptor::sb_apple_event_descriptor_set_param_descriptor(
                    handle, descriptor, keyword, error,
                )
            },
        )
    }

    /// Looks up an Apple event parameter by keyword on this `NSAppleEventDescriptor`.
    pub fn param_descriptor_for_keyword(&self, keyword: AEKeyword) -> Result<Option<Self>> {
        descriptor_lookup(
            self,
            keyword,
            "sb_apple_event_descriptor_param_descriptor_for_keyword",
            |handle, keyword, error| unsafe {
                ffi::apple_event_descriptor::sb_apple_event_descriptor_param_descriptor_for_keyword(
                    handle, keyword, error,
                )
            },
        )
    }

    /// Removes an Apple event parameter by keyword from this `NSAppleEventDescriptor`.
    pub fn remove_param_descriptor(&self, keyword: AEKeyword) -> Result<()> {
        descriptor_keyword_bool(
            self,
            keyword,
            "sb_apple_event_descriptor_remove_param_descriptor",
            |handle, keyword, error| unsafe {
                ffi::apple_event_descriptor::sb_apple_event_descriptor_remove_param_descriptor(
                    handle, keyword, error,
                )
            },
        )
    }

    /// Sets an Apple event attribute on this `NSAppleEventDescriptor`.
    pub fn set_attribute_descriptor(&self, descriptor: &Self, keyword: AEKeyword) -> Result<()> {
        descriptor_mutation(
            self,
            descriptor,
            keyword,
            "sb_apple_event_descriptor_set_attribute_descriptor",
            |handle, descriptor, keyword, error| unsafe {
                ffi::apple_event_descriptor::sb_apple_event_descriptor_set_attribute_descriptor(
                    handle, descriptor, keyword, error,
                )
            },
        )
    }

    /// Looks up an Apple event attribute by keyword on this `NSAppleEventDescriptor`.
    pub fn attribute_descriptor_for_keyword(&self, keyword: AEKeyword) -> Result<Option<Self>> {
        descriptor_lookup(
            self,
            keyword,
            "sb_apple_event_descriptor_attribute_descriptor_for_keyword",
            |handle, keyword, error| unsafe {
                ffi::apple_event_descriptor::sb_apple_event_descriptor_attribute_descriptor_for_keyword(
                    handle, keyword, error,
                )
            },
        )
    }

    /// Sends this Apple event descriptor and returns any reply descriptor.
    pub fn send_event(
        &self,
        send_options: AppleEventSendOptions,
        timeout: f64,
    ) -> Result<Option<Self>> {
        let mut error = std::ptr::null_mut();
        let raw = unsafe {
            ffi::apple_event_descriptor::sb_apple_event_descriptor_send_event(
                self.0.as_ptr(),
                send_options,
                timeout,
                &mut error,
            )
        };
        optional_handle(
            raw,
            "sb_apple_event_descriptor_send_event",
            error,
            Self::from_raw,
        )
    }

    /// Returns whether this `NSAppleEventDescriptor` is a record descriptor.
    pub fn is_record_descriptor(&self) -> bool {
        unsafe {
            ffi::apple_event_descriptor::sb_apple_event_descriptor_is_record_descriptor(
                self.0.as_ptr(),
            )
        }
    }

    /// Returns the item count reported by this `NSAppleEventDescriptor`.
    pub fn number_of_items(&self) -> usize {
        usize::try_from(unsafe {
            ffi::apple_event_descriptor::sb_apple_event_descriptor_number_of_items(self.0.as_ptr())
        })
        .unwrap_or_default()
    }

    /// Inserts a child descriptor into this list-style `NSAppleEventDescriptor`.
    pub fn insert_descriptor(&self, descriptor: &Self, index: usize) -> Result<()> {
        let index = i64::try_from(index).map_err(|_| {
            crate::ScriptingBridgeError::new(
                "sb_apple_event_descriptor_insert_descriptor",
                "descriptor index exceeds i64",
            )
        })?;
        let mut error = std::ptr::null_mut();
        let ok = unsafe {
            ffi::apple_event_descriptor::sb_apple_event_descriptor_insert_descriptor(
                self.0.as_ptr(),
                descriptor.0.as_ptr(),
                index,
                &mut error,
            )
        };
        bool_result(ok, "sb_apple_event_descriptor_insert_descriptor", error)
    }

    /// Returns the child descriptor at the given index from this `NSAppleEventDescriptor`.
    pub fn descriptor_at_index(&self, index: usize) -> Result<Option<Self>> {
        let index = i64::try_from(index).map_err(|_| {
            crate::ScriptingBridgeError::new(
                "sb_apple_event_descriptor_descriptor_at_index",
                "descriptor index exceeds i64",
            )
        })?;
        let mut error = std::ptr::null_mut();
        let raw = unsafe {
            ffi::apple_event_descriptor::sb_apple_event_descriptor_descriptor_at_index(
                self.0.as_ptr(),
                index,
                &mut error,
            )
        };
        optional_handle(
            raw,
            "sb_apple_event_descriptor_descriptor_at_index",
            error,
            Self::from_raw,
        )
    }

    /// Removes the child descriptor at the given index from this `NSAppleEventDescriptor`.
    pub fn remove_descriptor_at_index(&self, index: usize) -> Result<()> {
        let index = i64::try_from(index).map_err(|_| {
            crate::ScriptingBridgeError::new(
                "sb_apple_event_descriptor_remove_descriptor_at_index",
                "descriptor index exceeds i64",
            )
        })?;
        let mut error = std::ptr::null_mut();
        let ok = unsafe {
            ffi::apple_event_descriptor::sb_apple_event_descriptor_remove_descriptor_at_index(
                self.0.as_ptr(),
                index,
                &mut error,
            )
        };
        bool_result(
            ok,
            "sb_apple_event_descriptor_remove_descriptor_at_index",
            error,
        )
    }

    /// Sets a record item on this `NSAppleEventDescriptor` by keyword.
    pub fn set_descriptor(&self, descriptor: &Self, keyword: AEKeyword) -> Result<()> {
        descriptor_mutation(
            self,
            descriptor,
            keyword,
            "sb_apple_event_descriptor_set_descriptor",
            |handle, descriptor, keyword, error| unsafe {
                ffi::apple_event_descriptor::sb_apple_event_descriptor_set_descriptor(
                    handle, descriptor, keyword, error,
                )
            },
        )
    }

    /// Looks up a record item on this `NSAppleEventDescriptor` by keyword.
    pub fn descriptor_for_keyword(&self, keyword: AEKeyword) -> Result<Option<Self>> {
        descriptor_lookup(
            self,
            keyword,
            "sb_apple_event_descriptor_descriptor_for_keyword",
            |handle, keyword, error| unsafe {
                ffi::apple_event_descriptor::sb_apple_event_descriptor_descriptor_for_keyword(
                    handle, keyword, error,
                )
            },
        )
    }

    /// Removes a record item from this `NSAppleEventDescriptor` by keyword.
    pub fn remove_descriptor_for_keyword(&self, keyword: AEKeyword) -> Result<()> {
        descriptor_keyword_bool(
            self,
            keyword,
            "sb_apple_event_descriptor_remove_descriptor_for_keyword",
            |handle, keyword, error| unsafe {
                ffi::apple_event_descriptor::sb_apple_event_descriptor_remove_descriptor_for_keyword(
                    handle, keyword, error,
                )
            },
        )
    }

    /// Returns the record keyword for the descriptor at the given index.
    pub fn keyword_for_descriptor_at_index(&self, index: usize) -> Result<AEKeyword> {
        let index = i64::try_from(index).map_err(|_| {
            crate::ScriptingBridgeError::new(
                "sb_apple_event_descriptor_keyword_for_descriptor_at_index",
                "descriptor index exceeds i64",
            )
        })?;
        let mut error = std::ptr::null_mut();
        let keyword = unsafe {
            ffi::apple_event_descriptor::sb_apple_event_descriptor_keyword_for_descriptor_at_index(
                self.0.as_ptr(),
                index,
                &mut error,
            )
        };
        if error.is_null() {
            Ok(keyword)
        } else {
            Err(crate::internal::bridge_error(
                "sb_apple_event_descriptor_keyword_for_descriptor_at_index",
                error,
            ))
        }
    }

    /// Coerces this `NSAppleEventDescriptor` to another descriptor type.
    pub fn coerce_to_descriptor_type(&self, descriptor_type: DescType) -> Result<Option<Self>> {
        let mut error = std::ptr::null_mut();
        let raw = unsafe {
            ffi::apple_event_descriptor::sb_apple_event_descriptor_coerce_to_descriptor_type(
                self.0.as_ptr(),
                descriptor_type,
                &mut error,
            )
        };
        optional_handle(
            raw,
            "sb_apple_event_descriptor_coerce_to_descriptor_type",
            error,
            Self::from_raw,
        )
    }

    pub(crate) fn from_raw(handle: NonNull<c_void>) -> Self {
        Self(handle)
    }

    pub(crate) fn as_ptr(&self) -> *mut c_void {
        self.0.as_ptr()
    }

    pub(crate) fn into_raw(self) -> *mut c_void {
        let pointer = self.0.as_ptr();
        std::mem::forget(self);
        pointer
    }
}

impl RawAppleEventDescriptor {
    /// Returns the descriptor type stored in this raw `AEDesc`.
    pub fn descriptor_type(&self) -> DescType {
        unsafe { ffi::apple_event_descriptor::sb_aedesc_descriptor_type(self.0.as_ptr()) }
    }

    fn from_raw(handle: NonNull<c_void>) -> Self {
        Self(handle)
    }

    fn into_raw(self) -> *mut c_void {
        let pointer = self.0.as_ptr();
        std::mem::forget(self);
        pointer
    }
}

impl Drop for AppleEventDescriptor {
    fn drop(&mut self) {
        unsafe { ffi::apple_event_descriptor::sb_apple_event_descriptor_release(self.0.as_ptr()) };
    }
}

impl Drop for RawAppleEventDescriptor {
    fn drop(&mut self) {
        unsafe { ffi::apple_event_descriptor::sb_aedesc_release(self.0.as_ptr()) };
    }
}

/// Builds a four-character Apple event code in `OSType` form.
pub const fn four_char_code(bytes: [u8; 4]) -> u32 {
    u32::from_be_bytes(bytes)
}

fn descriptor_from_bytes(
    descriptor_type: DescType,
    bytes: &[u8],
    function: &'static str,
    create: impl FnOnce(DescType, *const u8, i64, *mut *mut i8) -> *mut c_void,
) -> Result<AppleEventDescriptor> {
    let length = i64::try_from(bytes.len())
        .map_err(|_| crate::ScriptingBridgeError::new(function, "buffer length exceeds i64"))?;
    let mut error = std::ptr::null_mut();
    let raw = create(descriptor_type, bytes.as_ptr(), length, &mut error);
    required_handle(raw, function, error, AppleEventDescriptor::from_raw)
}

fn descriptor_mutation(
    handle: &AppleEventDescriptor,
    descriptor: &AppleEventDescriptor,
    keyword: AEKeyword,
    function: &'static str,
    mutate: impl FnOnce(*mut c_void, *mut c_void, AEKeyword, *mut *mut i8) -> bool,
) -> Result<()> {
    let mut error = std::ptr::null_mut();
    let ok = mutate(
        handle.0.as_ptr(),
        descriptor.0.as_ptr(),
        keyword,
        &mut error,
    );
    bool_result(ok, function, error)
}

fn descriptor_lookup(
    handle: &AppleEventDescriptor,
    keyword: AEKeyword,
    function: &'static str,
    lookup: impl FnOnce(*mut c_void, AEKeyword, *mut *mut i8) -> *mut c_void,
) -> Result<Option<AppleEventDescriptor>> {
    let mut error = std::ptr::null_mut();
    let raw = lookup(handle.0.as_ptr(), keyword, &mut error);
    optional_handle(raw, function, error, AppleEventDescriptor::from_raw)
}

fn descriptor_keyword_bool(
    handle: &AppleEventDescriptor,
    keyword: AEKeyword,
    function: &'static str,
    call: impl FnOnce(*mut c_void, AEKeyword, *mut *mut i8) -> bool,
) -> Result<()> {
    let mut error = std::ptr::null_mut();
    let ok = call(handle.0.as_ptr(), keyword, &mut error);
    bool_result(ok, function, error)
}
