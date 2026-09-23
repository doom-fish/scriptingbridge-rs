import Foundation
import ScriptingBridge
import ScriptingBridgeObjCBridge

final class SBRSObjectHandle: NSObject {
  let object: SBObject

  init(object: SBObject) {
    self.object = object
  }
}

private func sbSendEventValues(
  parameterCodes: UnsafePointer<UInt32>?,
  parameterValues: UnsafePointer<UnsafeMutableRawPointer?>?,
  parameterCount: Int64,
  errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> (codes: [NSNumber], values: [NSAppleEventDescriptor])? {
  guard parameterCount >= 0 else {
    sbSetError(errorOut, "parameter count cannot be negative")
    return nil
  }

  let parameterCount = Int(parameterCount)
  guard parameterCount == 0 || (parameterCodes != nil && parameterValues != nil) else {
    sbSetError(errorOut, "missing Apple event parameter codes or values")
    return nil
  }

  guard parameterCount <= 8 else {
    sbSetError(errorOut, "sendEvent currently supports up to 8 code/value pairs")
    return nil
  }

  guard let parameterCodes, let parameterValues else {
    return ([], [])
  }

  let codeBuffer = UnsafeBufferPointer(start: parameterCodes, count: parameterCount)
  let valueBuffer = UnsafeBufferPointer(start: parameterValues, count: parameterCount)
  return (
    codeBuffer.map { NSNumber(value: $0) },
    valueBuffer.map { sbDescriptor(fromHandle: $0) ?? NSAppleEventDescriptor.null() })
}

func sbInvokeSendEvent(
  on object: AnyObject,
  eventClass: UInt32,
  eventID: UInt32,
  parameterCodes: UnsafePointer<UInt32>?,
  parameterValues: UnsafePointer<UnsafeMutableRawPointer?>?,
  parameterCount: Int64,
  errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> (succeeded: Bool, value: AnyObject?) {
  guard let parameters = sbSendEventValues(
    parameterCodes: parameterCodes,
    parameterValues: parameterValues,
    parameterCount: parameterCount,
    errorOut: errorOut)
  else {
    return (false, nil)
  }

  return sbRun(errorOut) {
    SBRSSendEvent(object, eventClass, eventID, parameters.codes, parameters.values, &$0, &$1)
  }
}

@_cdecl("sb_object_create")
public func sb_object_create(
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
  sbRetain(SBRSObjectHandle(object: SBObject()))
}

@_cdecl("sb_object_create_with_properties")
public func sb_object_create_with_properties(
  _ names: UnsafePointer<UnsafePointer<CChar>?>?,
  _ values: UnsafePointer<UnsafeMutableRawPointer?>?,
  _ count: Int64,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
  guard let properties = sbPropertyDictionary(names: names, values: values, count: count, errorOut: errorOut) else {
    return nil
  }

  return sbRetain(SBRSObjectHandle(object: SBObject(properties: properties)))
}

@_cdecl("sb_object_create_with_data")
public func sb_object_create_with_data(
  _ dataHandle: UnsafeMutableRawPointer?,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
  guard let data = sbCocoaValue(fromHandle: dataHandle) else {
    sbSetError(errorOut, "missing SBObject data value")
    return nil
  }

  return sbRetain(SBRSObjectHandle(object: SBObject(data: data as AnyObject)))
}

@_cdecl("sb_object_create_with_element_code")
public func sb_object_create_with_element_code(
  _ elementCode: UInt32,
  _ names: UnsafePointer<UnsafePointer<CChar>?>?,
  _ values: UnsafePointer<UnsafeMutableRawPointer?>?,
  _ count: Int64,
  _ dataHandle: UnsafeMutableRawPointer?,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
  let properties = sbPropertyDictionary(names: names, values: values, count: count, errorOut: errorOut)
  guard errorOut?.pointee == nil else {
    return nil
  }

  return sbRetain(
    SBRSObjectHandle(
      object: SBObject(
        elementCode: elementCode,
        properties: properties,
        data: sbCocoaValue(fromHandle: dataHandle))))
}

@_cdecl("sb_object_get")
public func sb_object_get(
  _ rawHandle: UnsafeMutableRawPointer?,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
  guard let rawHandle else {
    sbSetError(errorOut, "missing SBObject handle")
    return nil
  }

  let handle: SBRSObjectHandle = sbBorrow(rawHandle)
  let (succeeded, value) = sbRun(errorOut) {
    SBRSInvoke(handle.object, "get", nil, false, .object, &$0, &$1)
  }
  return succeeded ? sbDescriptorHandle(from: value) : nil
}

@_cdecl("sb_object_description")
public func sb_object_description(_ rawHandle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
  guard let rawHandle else {
    return nil
  }

  let handle: SBRSObjectHandle = sbBorrow(rawHandle)
  return sbCString(handle.object.description)
}

@_cdecl("sb_object_get_description")
public func sb_object_get_description(_ rawHandle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
  guard let rawHandle else {
    return nil
  }

  let handle: SBRSObjectHandle = sbBorrow(rawHandle)
  let (succeeded, value) = sbRun(nil) {
    SBRSInvoke(handle.object, "get", nil, false, .object, &$0, &$1)
  }
  guard succeeded, let value else {
    return nil
  }

  return sbCString(String(describing: value))
}

@_cdecl("sb_object_last_error_description")
public func sb_object_last_error_description(_ rawHandle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
  guard let rawHandle else {
    return nil
  }

  let handle: SBRSObjectHandle = sbBorrow(rawHandle)
  guard let error = handle.object.lastError() else {
    return nil
  }

  return sbCString(sbNSErrorMessage(error as NSError))
}

@_cdecl("sb_object_property_with_code")
public func sb_object_property_with_code(
  _ rawHandle: UnsafeMutableRawPointer?,
  _ code: UInt32,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
  guard let rawHandle else {
    sbSetError(errorOut, "missing SBObject handle")
    return nil
  }

  let handle: SBRSObjectHandle = sbBorrow(rawHandle)
  let (succeeded, value) = sbRun(errorOut) { SBRSPropertyWithCode(handle.object, code, &$0, &$1) }
  guard succeeded, let property = value as? SBObject else {
    return nil
  }
  return sbRetain(SBRSObjectHandle(object: property))
}

@_cdecl("sb_object_property_with_class")
public func sb_object_property_with_class(
  _ rawHandle: UnsafeMutableRawPointer?,
  _ classHandle: UnsafeMutableRawPointer?,
  _ code: UInt32,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
  guard let rawHandle else {
    sbSetError(errorOut, "missing SBObject handle")
    return nil
  }
  guard let classHandle else {
    sbSetError(errorOut, "missing scripting class handle")
    return nil
  }

  let handle: SBRSObjectHandle = sbBorrow(rawHandle)
  let scriptingClass: SBRSScriptingClassHandle = sbBorrow(classHandle)
  let (succeeded, value) = sbRun(errorOut) {
    SBRSPropertyWithClassAndCode(handle.object, scriptingClass.scriptingClass, code, &$0, &$1)
  }
  guard succeeded, let property = value as? SBObject else {
    return nil
  }
  return sbRetain(SBRSObjectHandle(object: property))
}

@_cdecl("sb_object_element_array_with_code")
public func sb_object_element_array_with_code(
  _ rawHandle: UnsafeMutableRawPointer?,
  _ code: UInt32,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
  guard let rawHandle else {
    sbSetError(errorOut, "missing SBObject handle")
    return nil
  }

  let handle: SBRSObjectHandle = sbBorrow(rawHandle)
  let (succeeded, value) = sbRun(errorOut) { SBRSElementArrayWithCode(handle.object, code, &$0, &$1) }
  guard succeeded, let array = value as? SBElementArray else {
    return nil
  }
  return sbRetain(SBRSElementArrayHandle(array: array))
}

@_cdecl("sb_object_send_event")
public func sb_object_send_event(
  _ rawHandle: UnsafeMutableRawPointer?,
  _ eventClass: UInt32,
  _ eventID: UInt32,
  _ parameterCodes: UnsafePointer<UInt32>?,
  _ parameterValues: UnsafePointer<UnsafeMutableRawPointer?>?,
  _ parameterCount: Int64,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
  guard let rawHandle else {
    sbSetError(errorOut, "missing SBObject handle")
    return nil
  }

  let handle: SBRSObjectHandle = sbBorrow(rawHandle)
  let (succeeded, result) = sbInvokeSendEvent(
    on: handle.object,
    eventClass: eventClass,
    eventID: eventID,
    parameterCodes: parameterCodes,
    parameterValues: parameterValues,
    parameterCount: parameterCount,
    errorOut: errorOut)

  return succeeded ? sbDescriptorHandle(from: result) : nil
}

@_cdecl("sb_object_set_to")
public func sb_object_set_to(
  _ rawHandle: UnsafeMutableRawPointer?,
  _ valueHandle: UnsafeMutableRawPointer?,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
  guard let rawHandle else {
    sbSetError(errorOut, "missing SBObject handle")
    return false
  }

  let handle: SBRSObjectHandle = sbBorrow(rawHandle)
  let value = sbCocoaValue(fromHandle: valueHandle)
  return sbRun(errorOut) {
    SBRSInvoke(handle.object, "setTo:", value, true, .void, &$0, &$1)
  }.succeeded
}

@_cdecl("sb_object_release")
public func sb_object_release(_ rawHandle: UnsafeMutableRawPointer?) {
  guard let rawHandle else {
    return
  }

  sbRelease(rawHandle)
}
