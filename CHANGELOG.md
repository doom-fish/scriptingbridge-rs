# Changelog

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
