# Changelog

## [0.1.0] - 2026-05-16

### Added

- Swift bridge for dynamic `SBApplication` construction and lifecycle control.
- `Application` wrapper covering `is_running`, `launch`, `quit`, `activate`,
  `terminate`, and `tell()`.
- `ScriptObject` and `ElementArray` wrappers for generic `SBObject` /
  `SBElementArray` values.
- `examples/01_finder_handle.rs` smoke example that opens a Finder handle and
  prints its running state.
