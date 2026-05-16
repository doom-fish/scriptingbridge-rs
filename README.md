# scriptingbridge-rs

Safe Rust bindings for Apple’s `ScriptingBridge.framework` on macOS, extended
with Swift-bridged `NSAppleEventDescriptor` and `NSAppleScript` helpers for
fully dynamic automation flows.

## Status

`0.2.0` covers six logical areas:

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
scriptingbridge-rs = "0.2"
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

## API notes

- `tell()` remains the convenience helper for zero- or one-argument string
  selectors / key paths.
- `SBObject::send_event` and `Application::send_event` expose the dynamic Apple
  event path directly when you need lower-level control.
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
