# Coverage

Scope audited for `0.2.0`:

- `ScriptingBridge.framework/Headers/SBApplication.h`
- `ScriptingBridge.framework/Headers/SBObject.h`
- `ScriptingBridge.framework/Headers/SBElementArray.h`
- Companion Foundation APIs explicitly required for this crate scope:
  `NSAppleEventDescriptor` and `NSAppleScript`

## SBApplication (`SBApplication.h`)

| API | Status | Notes |
| --- | --- | --- |
| `initWithBundleIdentifier:` | ✅ implemented | `Application::with_bundle_identifier` |
| `initWithURL:` | ✅ implemented | `Application::with_url` |
| `initWithProcessIdentifier:` | ✅ implemented | `Application::with_process_identifier` |
| `applicationWithBundleIdentifier:` | ✅ implemented | `Application::shared_with_bundle_identifier` |
| `applicationWithURL:` | ✅ implemented | `Application::shared_with_url` |
| `applicationWithProcessIdentifier:` | ✅ implemented | `Application::shared_with_process_identifier` |
| `classForScriptingClass:` | ✅ implemented | `Application::class_for_scripting_class` |
| `running` | ✅ implemented | `Application::is_running` |
| `activate` | ✅ implemented | `Application::activate` |
| `delegate` | ✅ implemented | `Application::set_delegate` / `has_delegate` |
| `launchFlags` | ✅ implemented | `launch_flags` / `set_launch_flags` |
| `sendMode` | ✅ implemented | `send_mode` / `set_send_mode` |
| `timeout` | ✅ implemented | `timeout` / `set_timeout` |

## SBApplicationDelegate (`SBApplication.h` informal protocol)

| API | Status | Notes |
| --- | --- | --- |
| `eventDidFail:withError:` | ✅ implemented | `ApplicationDelegate::new` callback bridge |

## SBObject (`SBObject.h`)

| API | Status | Notes |
| --- | --- | --- |
| `init` | ✅ implemented | `ScriptObject::new` |
| `initWithProperties:` | ✅ implemented | `ScriptObject::with_properties` |
| `initWithData:` | ✅ implemented | `ScriptObject::with_data` |
| `get` | ✅ implemented | `ScriptObject::get` |
| `lastError` | ✅ implemented | `last_error_description` |
| `initWithElementCode:properties:data:` | ✅ implemented | `ScriptObject::with_element_code` |
| `propertyWithCode:` | ✅ implemented | `property_with_code` |
| `propertyWithClass:code:` | ✅ implemented | `property_with_class` |
| `elementArrayWithCode:` | ✅ implemented | `element_array_with_code` |
| `sendEvent:id:parameters:` | ✅ implemented | `send_event` |
| `setTo:` | ✅ implemented | `set_to` |

## SBElementArray (`SBElementArray.h`)

| API | Status | Notes |
| --- | --- | --- |
| `objectWithName:` | ✅ implemented | `object_with_name` |
| `objectWithID:` | ✅ implemented | `object_with_id` |
| `objectAtLocation:` | ✅ implemented | `object_at_location` |
| `arrayByApplyingSelector:` | ✅ implemented | `array_by_applying_selector` |
| `arrayByApplyingSelector:withObject:` | ✅ implemented | `array_by_applying_selector_with_object` |
| `get` | ✅ implemented | `ElementArray::get` |

## NSAppleEventDescriptor (`NSAppleEventDescriptor.h`)

| API | Status | Notes |
| --- | --- | --- |
| `NSAppleEventSendOptions` constants | ✅ implemented | re-exported as `APPLE_EVENT_SEND_*` |
| `nullDescriptor` | ✅ implemented | `AppleEventDescriptor::null` |
| `descriptorWithDescriptorType:bytes:length:` | ✅ implemented | `with_descriptor_type_and_bytes` |
| `descriptorWithDescriptorType:data:` | ✅ implemented | `with_descriptor_type_and_data` |
| `descriptorWithBoolean:` | ✅ implemented | `with_boolean` |
| `descriptorWithEnumCode:` | ✅ implemented | `with_enum_code` |
| `descriptorWithInt32:` | ✅ implemented | `with_int32` |
| `descriptorWithDouble:` | ✅ implemented | `with_double` |
| `descriptorWithTypeCode:` | ✅ implemented | `with_type_code` |
| `descriptorWithString:` | ✅ implemented | `with_string` |
| `descriptorWithDate:` | ✅ implemented | `with_date` |
| `descriptorWithFileURL:` | ✅ implemented | `with_file_url` |
| `appleEventWithEventClass:eventID:targetDescriptor:returnID:transactionID:` | ✅ implemented | `apple_event` |
| `listDescriptor` | ✅ implemented | `list` |
| `recordDescriptor` | ✅ implemented | `record` |
| `currentProcessDescriptor` | ✅ implemented | `current_process` |
| `descriptorWithProcessIdentifier:` | ✅ implemented | `with_process_identifier` |
| `descriptorWithBundleIdentifier:` | ✅ implemented | `with_bundle_identifier` |
| `descriptorWithApplicationURL:` | ✅ implemented | `with_application_url` |
| `initWithAEDescNoCopy:` | ✅ implemented | `from_raw_aedesc_no_copy` |
| `initWithDescriptorType:bytes:length:` | ✅ implemented | `with_descriptor_type_and_bytes` |
| `initWithDescriptorType:data:` | ✅ implemented | `with_descriptor_type_and_data` |
| `initWithEventClass:eventID:targetDescriptor:returnID:transactionID:` | ✅ implemented | `apple_event` |
| `initListDescriptor` | ✅ implemented | `list` |
| `initRecordDescriptor` | ✅ implemented | `record` |
| `aeDesc` | ✅ implemented | `to_raw_aedesc` |
| `descriptorType` | ✅ implemented | `descriptor_type` |
| `data` | ✅ implemented | `data` |
| `booleanValue` | ✅ implemented | `boolean_value` |
| `enumCodeValue` | ✅ implemented | `enum_code_value` |
| `int32Value` | ✅ implemented | `int32_value` |
| `doubleValue` | ✅ implemented | `double_value` |
| `typeCodeValue` | ✅ implemented | `type_code_value` |
| `stringValue` | ✅ implemented | `string_value` |
| `dateValue` | ✅ implemented | `date_value` |
| `fileURLValue` | ✅ implemented | `file_url_value` |
| `eventClass` | ✅ implemented | `event_class` |
| `eventID` | ✅ implemented | `event_id` |
| `returnID` | ✅ implemented | `return_id` |
| `transactionID` | ✅ implemented | `transaction_id` |
| `setParamDescriptor:forKeyword:` | ✅ implemented | `set_param_descriptor` |
| `paramDescriptorForKeyword:` | ✅ implemented | `param_descriptor_for_keyword` |
| `removeParamDescriptorWithKeyword:` | ✅ implemented | `remove_param_descriptor` |
| `setAttributeDescriptor:forKeyword:` | ✅ implemented | `set_attribute_descriptor` |
| `attributeDescriptorForKeyword:` | ✅ implemented | `attribute_descriptor_for_keyword` |
| `sendEventWithOptions:timeout:error:` | ✅ implemented | `send_event` |
| `isRecordDescriptor` | ✅ implemented | `is_record_descriptor` |
| `numberOfItems` | ✅ implemented | `number_of_items` |
| `insertDescriptor:atIndex:` | ✅ implemented | `insert_descriptor` |
| `descriptorAtIndex:` | ✅ implemented | `descriptor_at_index` |
| `removeDescriptorAtIndex:` | ✅ implemented | `remove_descriptor_at_index` |
| `setDescriptor:forKeyword:` | ✅ implemented | `set_descriptor` |
| `descriptorForKeyword:` | ✅ implemented | `descriptor_for_keyword` |
| `removeDescriptorWithKeyword:` | ✅ implemented | `remove_descriptor_for_keyword` |
| `keywordForDescriptorAtIndex:` | ✅ implemented | `keyword_for_descriptor_at_index` |
| `coerceToDescriptorType:` | ✅ implemented | `coerce_to_descriptor_type` |

## NSAppleScript (`NSAppleScript.h`)

| API | Status | Notes |
| --- | --- | --- |
| `NSAppleScriptErrorMessage` | ✅ implemented | `APPLE_SCRIPT_ERROR_MESSAGE_KEY` |
| `NSAppleScriptErrorNumber` | ✅ implemented | `APPLE_SCRIPT_ERROR_NUMBER_KEY` |
| `NSAppleScriptErrorAppName` | ✅ implemented | `APPLE_SCRIPT_ERROR_APP_NAME_KEY` |
| `NSAppleScriptErrorBriefMessage` | ✅ implemented | `APPLE_SCRIPT_ERROR_BRIEF_MESSAGE_KEY` |
| `NSAppleScriptErrorRange` | ✅ implemented | `APPLE_SCRIPT_ERROR_RANGE_KEY` |
| `initWithContentsOfURL:error:` | ✅ implemented | `AppleScript::with_contents_of_url` |
| `initWithSource:` | ✅ implemented | `AppleScript::with_source` |
| `source` | ✅ implemented | `AppleScript::source` |
| `compiled` | ✅ implemented | `AppleScript::is_compiled` |
| `compileAndReturnError:` | ✅ implemented | `AppleScript::compile` |
| `executeAndReturnError:` | ✅ implemented | `AppleScript::execute` |
| `executeAppleEvent:error:` | ✅ implemented | `AppleScript::execute_apple_event` |
