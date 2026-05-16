# scriptingbridge-rs

Safe Rust bindings for Apple’s `ScriptingBridge.framework` on macOS.

`scriptingbridge-rs` focuses on the dynamic Scripting Bridge workflow you use
without generated glue headers:

- `Application::with_bundle_identifier` for `SBApplication`
- `is_running`, `launch`, `quit`, `activate`, and `terminate`
- `tell(command, args)` for simple dynamic commands / property access
- lightweight `ScriptObject` and `ElementArray` wrappers for generic results

## Status

Initial `0.1.0` coverage targets dynamic `SBApplication` control only. It does
not attempt to generate typed glue from `.sdef` files.

## Installation

```toml
[dependencies]
scriptingbridge-rs = "0.1"
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

- Swift bridge for `SBApplication` construction and app lifecycle control
- `tell()` helper for zero- or one-argument dynamic calls
- `ScriptObject` and `ElementArray` wrappers with description / `get()` helpers
- Finder smoke example that safely inspects state without sending commands

## API notes

- `tell()` supports zero or one string argument in `0.1.0`; richer selector
  shapes would require generated glue or more Objective-C runtime work.
- Generated `.sdef` / `sdp` headers are the normal path for strongly typed access;
  this crate intentionally stays on the dynamic API surface.
- `launch()` uses AppKit’s async app-opening API bridged back into sync Rust.

## Smoke example

```bash
cargo run --example 01_finder_handle
```

Expected tail output:

```text
✅ scriptingbridge Finder app handle OK
```

## License

Licensed under either of:

- MIT license ([LICENSE-MIT](LICENSE-MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
