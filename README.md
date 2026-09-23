# scriptingbridge-rs

Safe Rust bindings for Apple’s `ScriptingBridge.framework` on macOS, extended
with Swift-bridged `NSAppleEventDescriptor` and `NSAppleScript` helpers for
fully dynamic automation flows.

## Status

Requires macOS 13 or later. The crate covers six logical areas:

- `SBApplication`
- `SBObject`
- `SBElementArray`
- `SBApplicationDelegate`
- `NSAppleEventDescriptor`
- `NSAppleScript`

See [`COVERAGE.md`](COVERAGE.md) for the API-by-API matrix.

## Installation

```toml
[dependencies]
scriptingbridge-rs = "0.4"
```

## Quick start

```rust
use scriptingbridge::Application;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let finder = Application::with_bundle_identifier("com.apple.finder")?;
    println!("Finder running: {}", finder.is_running());
    Ok(())
}
```

## Highlights

- `SBApplication` constructors by bundle identifier, URL, or process identifier,
  plus `launch_flags`, `send_mode`, `timeout`, delegate attachment, and
  `class_for_scripting_class`.
- `ScriptObject` / `ElementArray` wrappers for property lookups, array queries,
  and low-level Apple event sends.
- `AppleEventDescriptor` builders for lists, records, addresses, raw `AEDesc`
  round-trips, and Apple event mutation / sending.
- `AppleScript` helpers for inline source, file-backed scripts, compilation, and
  execution.

## Automation permission (TCC)

Sending Apple events to another application needs the user's consent, which
macOS records under System Settings > Privacy & Security > Automation for the
responsible process (your app, or the terminal that runs your binary). The first
event to a target shows a prompt and blocks until the user answers; a denied
event fails with `errAEEventNotPermitted` (-1743), which this crate reports as
an error. Apps that use the hardened runtime need the
`com.apple.security.automation.apple-events` entitlement, sandboxed apps need an
Apple-events entitlement for each target, and every app must describe why in
`NSAppleEventsUsageDescription` in its `Info.plist`.

`Application::automation_permission(false)` reports the current state without
prompting (`Granted`, `Denied`, `RequiresConsent` or `TargetNotRunning`); pass
`true` to show the prompt, which blocks while it is on screen.

## Threading

Apple documents `NSAppleScript` as main-thread only, so create, compile and run
`AppleScript` values on the main thread. The crate does not enforce this. None
of the handle types are `Send`, so each stays on the thread that created it.

## Untrusted strings

`tell`, `object_for_key_path`, `element_array_for_key_path` and
`array_by_applying_selector*` never perform an arbitrary selector:

- Names must be ASCII identifiers. Memory-management and dispatch selectors
  (`dealloc`, `release`, `retain`, `autorelease`, `retainCount`,
  `performSelector:`, …), the `alloc`, `new`, `copy`, `mutableCopy` and `init`
  families, setters (`setX:`) and private `_` selectors are rejected.
- A performed method must exist, return an object and take object arguments,
  according to `methodSignatureForSelector:`.
- Zero-argument `tell` calls and key paths go through key-value coding, and each
  component must be a readable key of the object it is applied to. The key given
  to `valueForKey:` is checked the same way.
- Unknown keys and Objective-C exceptions become errors instead of aborting the
  process.

A command that passes these checks is still a real scripting command of the
target application: untrusted input can still ask it to do anything its
scripting dictionary allows, such as `delete`.

## Errors

Every application handle gets a default `SBApplicationDelegate`, so a failed
Apple event becomes an `Err` instead of an Objective-C exception. A delegate
installed with `set_delegate` sees the failure first: returning `Some` supplies
the operation's result, returning `None` keeps the error. `has_delegate` reports
only delegates installed through this crate's API.

## API notes

- `tell()` remains the convenience helper for zero- or one-argument string
  selectors / key paths, within the rules above.
- `SBObject::send_event` and `Application::send_event` expose the dynamic Apple
  event path directly when you need lower-level control. They pass up to eight
  parameters through a variadic Objective-C call, and the descriptors reach the
  event unchanged.
- Numbers keep their width and type in both directions (32- and 64-bit
  integers, unsigned 64-bit values, floating point, booleans). Records whose
  keys are names rather than four-character codes (for example the
  `properties` of a Finder item) are returned with the names as user record
  fields (`usrf`).
- `AppleEventDescriptor` re-exports the standard Apple event send-option bitflags
  and the `NSAppleScript` error dictionary keys.

## Examples

```bash
cargo run --example 01_finder_handle
cargo run --example 02_sbobject_properties
cargo run --example 03_sbelementarray_disks
cargo run --example 04_sbapplication_delegate
cargo run --example 05_nsappleeventdescriptor_roundtrip
cargo run --example 06_nsapplescript_run
```

## License

Licensed under either of:

- MIT license ([LICENSE-MIT](LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
