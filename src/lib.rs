#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(
    clippy::doc_markdown,
    clippy::missing_const_for_fn,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::redundant_pub_crate,
    clippy::similar_names,
    clippy::use_self
)]
#![doc = include_str!("../README.md")]

#[cfg(not(target_os = "macos"))]
compile_error!("scriptingbridge only supports macOS");

mod apple_event_descriptor;
mod apple_script;
mod application;
mod application_delegate;
mod element_array;
mod error;
mod ffi;
mod internal;
mod object;

pub use apple_event_descriptor::{
    four_char_code, AEEventClass, AEEventID, AEKeyword, AEReturnID, AETransactionID,
    APPLE_EVENT_SEND_ALWAYS_INTERACT, APPLE_EVENT_SEND_CAN_INTERACT,
    APPLE_EVENT_SEND_CAN_SWITCH_LAYER, APPLE_EVENT_SEND_DEFAULT_OPTIONS,
    APPLE_EVENT_SEND_DONT_ANNOTATE, APPLE_EVENT_SEND_DONT_EXECUTE,
    APPLE_EVENT_SEND_DONT_RECORD, APPLE_EVENT_SEND_NEVER_INTERACT,
    APPLE_EVENT_SEND_NO_REPLY, APPLE_EVENT_SEND_QUEUE_REPLY,
    APPLE_EVENT_SEND_WAIT_FOR_REPLY, AppleEventDescriptor, AppleEventSendOptions, DescType,
    OSType, RawAppleEventDescriptor,
};
pub use apple_script::{
    AppleScript, APPLE_SCRIPT_ERROR_APP_NAME_KEY, APPLE_SCRIPT_ERROR_BRIEF_MESSAGE_KEY,
    APPLE_SCRIPT_ERROR_MESSAGE_KEY, APPLE_SCRIPT_ERROR_NUMBER_KEY,
    APPLE_SCRIPT_ERROR_RANGE_KEY,
};
pub use application::{Application, LaunchFlags, ScriptingClass, SendMode};
pub use application_delegate::{ApplicationDelegate, ApplicationErrorEvent};
pub use element_array::ElementArray;
pub use error::{Result, ScriptingBridgeError};
pub use object::{EventParameter, Property, ScriptObject};
