# scriptingbridge-rs coverage audit (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 35
VERIFIED: 35
GAPS: 0
EXEMPT: 0
COVERAGE_PCT: 100.00%

Scope: public ScriptingBridge declarations in `SBApplication.h`, `SBObject.h`, and `SBElementArray.h`. The crate's `NSAppleEventDescriptor` / `NSAppleScript` helpers are Foundation wrappers and are out of scope for this audit. Some Rust APIs are intentionally lossy but still make the underlying ScriptingBridge symbol reachable (for example `ScriptObject::last_error_description` and `Application::has_delegate`).

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| `SBApplication` | interface | `SBApplication.h` | `Application` |
| `-initWithBundleIdentifier:` | instance method | `SBApplication.h` | `Application::with_bundle_identifier` |
| `-initWithURL:` | instance method | `SBApplication.h` | `Application::with_url` |
| `-initWithProcessIdentifier:` | instance method | `SBApplication.h` | `Application::with_process_identifier` |
| `+applicationWithBundleIdentifier:` | class method | `SBApplication.h` | `Application::shared_with_bundle_identifier` |
| `+applicationWithURL:` | class method | `SBApplication.h` | `Application::shared_with_url` |
| `+applicationWithProcessIdentifier:` | class method | `SBApplication.h` | `Application::shared_with_process_identifier` |
| `-classForScriptingClass:` | instance method | `SBApplication.h` | `Application::class_for_scripting_class`, `ScriptingClass` |
| `running` | property | `SBApplication.h` | `Application::is_running` |
| `-activate` | instance method | `SBApplication.h` | `Application::activate` |
| `delegate` | property | `SBApplication.h` | `Application::set_delegate`, `Application::has_delegate` |
| `launchFlags` | property | `SBApplication.h` | `Application::launch_flags`, `Application::set_launch_flags` |
| `sendMode` | property | `SBApplication.h` | `Application::send_mode`, `Application::set_send_mode` |
| `timeout` | property | `SBApplication.h` | `Application::timeout`, `Application::set_timeout` |
| `SBApplicationDelegate` | protocol | `SBApplication.h` | `ApplicationDelegate` |
| `-eventDidFail:withError:` | protocol method | `SBApplication.h` | `ApplicationDelegate::new` callback bridge |
| `SBObject` | interface | `SBObject.h` | `ScriptObject` |
| `-init` | instance method | `SBObject.h` | `ScriptObject::new` |
| `-initWithProperties:` | instance method | `SBObject.h` | `ScriptObject::with_properties` |
| `-initWithData:` | instance method | `SBObject.h` | `ScriptObject::with_data` |
| `-get` | instance method | `SBObject.h` | `ScriptObject::get` |
| `-lastError` | property | `SBObject.h` | `ScriptObject::last_error_description` |
| `-initWithElementCode:properties:data:` | instance method | `SBObject.h` | `ScriptObject::with_element_code` |
| `-propertyWithCode:` | instance method | `SBObject.h` | `ScriptObject::property_with_code` |
| `-propertyWithClass:code:` | instance method | `SBObject.h` | `ScriptObject::property_with_class` |
| `-elementArrayWithCode:` | instance method | `SBObject.h` | `ScriptObject::element_array_with_code` |
| `-sendEvent:id:parameters:` | instance method | `SBObject.h` | `ScriptObject::send_event`, `Application::send_event` |
| `-setTo:` | instance method | `SBObject.h` | `ScriptObject::set_to` |
| `SBElementArray` | interface | `SBElementArray.h` | `ElementArray` |
| `-objectWithName:` | instance method | `SBElementArray.h` | `ElementArray::object_with_name` |
| `-objectWithID:` | instance method | `SBElementArray.h` | `ElementArray::object_with_id` |
| `-objectAtLocation:` | instance method | `SBElementArray.h` | `ElementArray::object_at_location` |
| `-arrayByApplyingSelector:` | instance method | `SBElementArray.h` | `ElementArray::array_by_applying_selector` |
| `-arrayByApplyingSelector:withObject:` | instance method | `SBElementArray.h` | `ElementArray::array_by_applying_selector_with_object` |
| `-get` | instance method | `SBElementArray.h` | `ElementArray::get` |

## 🔴 GAPS
| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |

_None._

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |

_None._
