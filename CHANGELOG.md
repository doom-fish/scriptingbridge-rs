# Changelog

## [0.3.2] - 2026-05-20

- Widen `doom-fish-utils` dependency bound to `<0.4` so the 0.3.x SPSC-ring release resolves cleanly. No source changes.

## [0.3.1] - 2026-05-18

### Changed

- Added concise rustdoc comments across the public Scripting Bridge API outside the ffi layer.

## [0.3.0] - 2026-05-18

### Changed

- Added `apple-cf` (`>=0.9, <0.10`) and re-exported `OSType` from `apple_cf::raw`, removing the crate-local duplicate alias used by the Apple event helpers.

## [0.2.1] - 2025-01-09

### Fixed

- Added SAFETY: comments to all `unsafe` blocks explaining their correctness
- Added panic-safety handling to `extern "C"` callbacks in `ApplicationDelegate`
  to prevent panics from unwinding across the FFI boundary
- Added `doom-fish-utils` dependency for panic catching helpers

## [0.2.0] - 2026-05-16

### Added

- Split the Swift bridge into per-area files for `SBApplication`, `SBObject`,
  `SBElementArray`, `SBApplicationDelegate`, `NSAppleEventDescriptor`, and
  `NSAppleScript`.
- Expanded the safe Rust surface with per-area modules, owned handle types, raw
  `AEDesc` round-tripping, Apple event send-option constants, and AppleScript
  error-key constants.
- Added six numbered examples and six integration tests covering each logical
  area.
- Added `COVERAGE.md` with the surface audit for the framework and companion
  AppleScript / Apple event helpers.

## [0.1.0] - 2026-05-16

### Added

- Swift bridge for dynamic `SBApplication` construction and lifecycle control.
- `Application` wrapper covering `is_running`, `launch`, `quit`, `activate`,
  `terminate`, and `tell()`.
- `ScriptObject` and `ElementArray` wrappers for generic `SBObject` /
  `SBElementArray` values.
- `examples/01_finder_handle.rs` smoke example that opens a Finder handle and
  prints its running state.
