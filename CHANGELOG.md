# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] - 2026-09-24

### Security

- `tell()`, `object_for_key_path`, `element_array_for_key_path` and `array_by_applying_selector*` performed selectors named by the caller: `dealloc`, `release` or `autorelease` freed the target (use-after-free), scalar-returning methods such as `isRunning` were read back as object pointers, and `valueForKey:` keys reached key-value coding's accessor search. Names are now validated in Rust (ASCII identifiers only; no memory-management or dispatch selectors, no `alloc`/`new`/`copy`/`mutableCopy`/`init` families, no setters, no private `_` selectors), and an Objective-C shim checks `respondsToSelector:` and `methodSignatureForSelector:` (object return, object arguments, expected count) before calling inside `@try`/`@catch`. Zero-argument `tell()` calls and key paths use key-value coding with every component checked to be a readable key.

### Fixed

- `send_event` called the variadic `sendEvent:id:parameters:` through non-variadic casts, so on arm64 every parameter was read from the wrong place. It now goes through a variadic Objective-C call with a 0 terminator, and the parameter descriptors reach the event unchanged.
- Objective-C exceptions (unknown keys, `get` on an object that is not in a container, invalid arguments) aborted the process; they are returned as errors.
- Failed Apple events raised an exception or silently returned `Ok(None)`. Every application handle now has a default `SBApplicationDelegate`, and the call that sent the event returns its error.
- Empty lists and records passed as values trapped in `1...numberOfItems` and aborted the process.
- Numbers wider than 32 bits were truncated to `int32` in both directions. 64-bit and unsigned 64-bit integers, floating-point values and booleans keep their width and type; since Scripting Bridge itself truncates `NSNumber` values, wide integers are passed as `comp`/`ucom` descriptors.
- Dictionaries whose keys are names rather than four-character codes, such as Finder's `properties`, came back as a description string; they are records with the names as user record fields (`usrf`). Records and other descriptors passed as values reach events unchanged.
- `quit()` read the result of a `void` method as an object.
- The delegate's closure could be aliased by re-entrant or concurrent callbacks; it lives in a `CallbackContext` that the Swift delegate retains and that is deactivated when the Rust handle is dropped.
- `launch()` left an unstructured `Task` running after its timeout; it uses the completion-handler API.
- Docs: the README covers Automation permission and entitlements, that `NSAppleScript` is main-thread only, and what untrusted strings can reach; the coverage files no longer claim semantic equivalence.

### Changed

- **Breaking:** `tell()` and the key-path methods reject non-identifier names and the selectors listed above, and `tell()` no longer runs `void` commands or setters (use `quit`, `activate` or `ScriptObject::set_to`).
- **Breaking:** failed Apple events return `Err` instead of `Ok(None)`. A delegate installed with `set_delegate` still sees the failure first and can return a replacement value; returning `None` keeps the error.
- **Breaking:** `has_delegate()` is `false` while only the default delegate is installed, and `set_delegate(None)` restores the default delegate.
- **Breaking:** `send_event` rejects a parameter code of 0, which would end the variadic list; more than 8 parameters remains an error.
- **Breaking:** `AppleScript::with_source` and `AppleScript::with_contents_of_url` return an error off the main thread, where Apple documents `NSAppleScript` as main-thread only. `AppleScript` is neither `Send` nor `Sync`, so its other methods and its drop run on the main thread too. The `NSAppleScript` tests moved to a `harness = false` test target that runs on the main thread.
- The tests that send Apple Events to Finder are `#[ignore]`d. They no longer activate Finder, and when run with `--ignored` they skip without Automation permission.
- `rust-version` is 1.82; `apple-cf` is required at `>=0.11, <0.12` and `doom-fish-utils` at `>=0.4.1, <0.5`.

### Added

- `Application::automation_permission` and `AutomationPermission`, which report the Automation consent state without prompting unless asked to.

## [0.3.3] - 2026-06-06

- `launch()` no longer writes the caller's error pointer after its timeout, the delegate drop trampoline contains panics from the user's closure, and the empty Swift bridge header was removed.

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
