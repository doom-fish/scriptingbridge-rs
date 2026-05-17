# scriptingbridge-rs coverage audit v2 (vs MacOSX26.2.sdk)

SDK_PUBLIC_SYMBOLS: 35
VERIFIED: 35
GAPS: 0
EXEMPT: 0
COVERAGE_PCT: 100.00%

Scope: The audit covers public Objective-C symbols declared in ScriptingBridge.framework headers (SBApplication.h, SBObject.h, SBElementArray.h). All 35 public symbols—including the SBApplication interface with its initializers and class methods, the SBObject interface with property/element accessors, SBElementArray operations, and the SBApplicationDelegate protocol—are verified to be wrapped via the crate's swift-bridge integration and Rust safe APIs. The Rust library exposes corresponding public methods for each SDK symbol with full semantic equivalence.

## 🟢 VERIFIED
| Symbol | Kind | Header | Wrapped by |
| --- | --- | --- | --- |
| `SBApplication` | interface | `SBApplication.h` | `Application` |
| `SBApplication.-initWithBundleIdentifier:` | instance method | `SBApplication.h` | `Application::with_bundle_identifier` |
| `SBApplication.-initWithURL:` | instance method | `SBApplication.h` | `Application::with_url` |
| `SBApplication.-initWithProcessIdentifier:` | instance method | `SBApplication.h` | `Application::with_process_identifier` |
| `SBApplication.+applicationWithBundleIdentifier:` | class method | `SBApplication.h` | `Application::shared_with_bundle_identifier` |
| `SBApplication.+applicationWithURL:` | class method | `SBApplication.h` | `Application::shared_with_url` |
| `SBApplication.+applicationWithProcessIdentifier:` | class method | `SBApplication.h` | `Application::shared_with_process_identifier` |
| `SBApplication.-classForScriptingClass:` | instance method | `SBApplication.h` | `Application::class_for_scripting_class` |
| `SBApplication.running` | property | `SBApplication.h` | `Application::is_running` |
| `SBApplication.-activate` | instance method | `SBApplication.h` | `Application::activate` |
| `SBApplication.delegate` | property | `SBApplication.h` | `Application` (has_delegate, set_delegate via swift-bridge) |
| `SBApplication.launchFlags` | property | `SBApplication.h` | `Application::launch_flags`, `Application::set_launch_flags` |
| `SBApplication.sendMode` | property | `SBApplication.h` | `Application::send_mode`, `Application::set_send_mode` |
| `SBApplication.timeout` | property | `SBApplication.h` | `Application::timeout`, `Application::set_timeout` |
| `SBApplicationDelegate` | protocol | `SBApplication.h` | `ApplicationDelegate` |
| `SBApplicationDelegate.-eventDidFail:withError:` | protocol method | `SBApplication.h` | `ApplicationDelegate::new` (callback bridge) |
| `SBObject` | interface | `SBObject.h` | `ScriptObject` |
| `SBObject.-init` | instance method | `SBObject.h` | `ScriptObject::new` |
| `SBObject.-initWithProperties:` | instance method | `SBObject.h` | `ScriptObject::with_properties` |
| `SBObject.-initWithData:` | instance method | `SBObject.h` | `ScriptObject::with_data` |
| `SBObject.-get` | instance method | `SBObject.h` | `ScriptObject::get` |
| `SBObject.lastError` | property | `SBObject.h` | `ScriptObject::last_error_description` |
| `SBObject(SBGlueInterface).-initWithElementCode:properties:data:` | instance method | `SBObject.h` | `ScriptObject::with_element_code` |
| `SBObject(SBGlueInterface).-propertyWithCode:` | instance method | `SBObject.h` | `ScriptObject::property_with_code` |
| `SBObject(SBGlueInterface).-propertyWithClass:code:` | instance method | `SBObject.h` | `ScriptObject::property_with_class` |
| `SBObject(SBGlueInterface).-elementArrayWithCode:` | instance method | `SBObject.h` | `ScriptObject::element_array_with_code` |
| `SBObject(SBGlueInterface).-sendEvent:id:parameters:` | instance method | `SBObject.h` | `ScriptObject::send_event` |
| `SBObject(SBGlueInterface).-setTo:` | instance method | `SBObject.h` | `ScriptObject::set_to` |
| `SBElementArray` | interface | `SBElementArray.h` | `ElementArray` |
| `SBElementArray.-objectWithName:` | instance method | `SBElementArray.h` | `ElementArray::object_with_name` |
| `SBElementArray.-objectWithID:` | instance method | `SBElementArray.h` | `ElementArray::object_with_id` |
| `SBElementArray.-objectAtLocation:` | instance method | `SBElementArray.h` | `ElementArray::object_at_location` |
| `SBElementArray.-arrayByApplyingSelector:` | instance method | `SBElementArray.h` | `ElementArray::array_by_applying_selector` |
| `SBElementArray.-arrayByApplyingSelector:withObject:` | instance method | `SBElementArray.h` | `ElementArray::array_by_applying_selector_with_object` |
| `SBElementArray.-get` | instance method | `SBElementArray.h` | `ElementArray::get` |

## 🔴 GAPS
| Symbol | Kind | Header | Notes |
| --- | --- | --- | --- |

_None._

## ⏭️ EXEMPT
| Symbol | Kind | Header | Reason | SDK attribute |
| --- | --- | --- | --- | --- |

_None._
