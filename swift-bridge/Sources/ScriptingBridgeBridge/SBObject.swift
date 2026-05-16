import Foundation
import ScriptingBridge

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
) -> (codes: [UInt32], values: [Any?])? {
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
    Array(codeBuffer),
    valueBuffer.map { sbCocoaValue(fromHandle: $0) })
}

func sbInvokeSendEvent(
  on object: AnyObject,
  eventClass: UInt32,
  eventID: UInt32,
  parameterCodes: UnsafePointer<UInt32>?,
  parameterValues: UnsafePointer<UnsafeMutableRawPointer?>?,
  parameterCount: Int64,
  errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Any? {
  guard let parameters = sbSendEventValues(
    parameterCodes: parameterCodes,
    parameterValues: parameterValues,
    parameterCount: parameterCount,
    errorOut: errorOut)
  else {
    return nil
  }

  let selector = NSSelectorFromString("sendEvent:id:parameters:")
  let imp = object.method(for: selector)
  let codes = parameters.codes
  let values = parameters.values

  switch parameters.codes.count {
  case 0:
    typealias SendEvent = @convention(c) (AnyObject, Selector, UInt32, UInt32, UInt32) -> AnyObject?
    let function = unsafeBitCast(imp, to: SendEvent.self)
    return function(object, selector, eventClass, eventID, 0)
  case 1:
    typealias SendEvent = @convention(c) (AnyObject, Selector, UInt32, UInt32, UInt32, AnyObject?, UInt32) -> AnyObject?
    let function = unsafeBitCast(imp, to: SendEvent.self)
    return function(object, selector, eventClass, eventID, codes[0], values[0] as AnyObject?, 0)
  case 2:
    typealias SendEvent = @convention(c) (AnyObject, Selector, UInt32, UInt32, UInt32, AnyObject?, UInt32, AnyObject?, UInt32) -> AnyObject?
    let function = unsafeBitCast(imp, to: SendEvent.self)
    return function(
      object,
      selector,
      eventClass,
      eventID,
      codes[0],
      values[0] as AnyObject?,
      codes[1],
      values[1] as AnyObject?,
      0)
  case 3:
    typealias SendEvent = @convention(c) (AnyObject, Selector, UInt32, UInt32, UInt32, AnyObject?, UInt32, AnyObject?, UInt32, AnyObject?, UInt32) -> AnyObject?
    let function = unsafeBitCast(imp, to: SendEvent.self)
    return function(
      object,
      selector,
      eventClass,
      eventID,
      codes[0],
      values[0] as AnyObject?,
      codes[1],
      values[1] as AnyObject?,
      codes[2],
      values[2] as AnyObject?,
      0)
  case 4:
    typealias SendEvent = @convention(c) (AnyObject, Selector, UInt32, UInt32, UInt32, AnyObject?, UInt32, AnyObject?, UInt32, AnyObject?, UInt32, AnyObject?, UInt32) -> AnyObject?
    let function = unsafeBitCast(imp, to: SendEvent.self)
    return function(
      object,
      selector,
      eventClass,
      eventID,
      codes[0],
      values[0] as AnyObject?,
      codes[1],
      values[1] as AnyObject?,
      codes[2],
      values[2] as AnyObject?,
      codes[3],
      values[3] as AnyObject?,
      0)
  case 5:
    typealias SendEvent = @convention(c) (AnyObject, Selector, UInt32, UInt32, UInt32, AnyObject?, UInt32, AnyObject?, UInt32, AnyObject?, UInt32, AnyObject?, UInt32, AnyObject?, UInt32) -> AnyObject?
    let function = unsafeBitCast(imp, to: SendEvent.self)
    return function(
      object,
      selector,
      eventClass,
      eventID,
      codes[0],
      values[0] as AnyObject?,
      codes[1],
      values[1] as AnyObject?,
      codes[2],
      values[2] as AnyObject?,
      codes[3],
      values[3] as AnyObject?,
      codes[4],
      values[4] as AnyObject?,
      0)
  case 6:
    typealias SendEvent = @convention(c) (AnyObject, Selector, UInt32, UInt32, UInt32, AnyObject?, UInt32, AnyObject?, UInt32, AnyObject?, UInt32, AnyObject?, UInt32, AnyObject?, UInt32, AnyObject?, UInt32) -> AnyObject?
    let function = unsafeBitCast(imp, to: SendEvent.self)
    return function(
      object,
      selector,
      eventClass,
      eventID,
      codes[0],
      values[0] as AnyObject?,
      codes[1],
      values[1] as AnyObject?,
      codes[2],
      values[2] as AnyObject?,
      codes[3],
      values[3] as AnyObject?,
      codes[4],
      values[4] as AnyObject?,
      codes[5],
      values[5] as AnyObject?,
      0)
  case 7:
    typealias SendEvent = @convention(c) (AnyObject, Selector, UInt32, UInt32, UInt32, AnyObject?, UInt32, AnyObject?, UInt32, AnyObject?, UInt32, AnyObject?, UInt32, AnyObject?, UInt32, AnyObject?, UInt32, AnyObject?, UInt32) -> AnyObject?
    let function = unsafeBitCast(imp, to: SendEvent.self)
    return function(
      object,
      selector,
      eventClass,
      eventID,
      codes[0],
      values[0] as AnyObject?,
      codes[1],
      values[1] as AnyObject?,
      codes[2],
      values[2] as AnyObject?,
      codes[3],
      values[3] as AnyObject?,
      codes[4],
      values[4] as AnyObject?,
      codes[5],
      values[5] as AnyObject?,
      codes[6],
      values[6] as AnyObject?,
      0)
  case 8:
    typealias SendEvent = @convention(c) (AnyObject, Selector, UInt32, UInt32, UInt32, AnyObject?, UInt32, AnyObject?, UInt32, AnyObject?, UInt32, AnyObject?, UInt32, AnyObject?, UInt32, AnyObject?, UInt32, AnyObject?, UInt32, AnyObject?, UInt32) -> AnyObject?
    let function = unsafeBitCast(imp, to: SendEvent.self)
    return function(
      object,
      selector,
      eventClass,
      eventID,
      codes[0],
      values[0] as AnyObject?,
      codes[1],
      values[1] as AnyObject?,
      codes[2],
      values[2] as AnyObject?,
      codes[3],
      values[3] as AnyObject?,
      codes[4],
      values[4] as AnyObject?,
      codes[5],
      values[5] as AnyObject?,
      codes[6],
      values[6] as AnyObject?,
      codes[7],
      values[7] as AnyObject?,
      0)
  default:
    sbSetError(errorOut, "unreachable Apple event parameter arity")
    return nil
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
  return sbDescriptorHandle(from: handle.object.get())
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
  guard let value = handle.object.get() else {
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
  return sbRetain(SBRSObjectHandle(object: handle.object.property(withCode: code)))
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
  return sbRetain(
    SBRSObjectHandle(
      object: handle.object.property(with: scriptingClass.scriptingClass, code: code)))
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
  return sbRetain(SBRSElementArrayHandle(array: handle.object.elementArray(withCode: code)))
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
  let result = sbInvokeSendEvent(
    on: handle.object,
    eventClass: eventClass,
    eventID: eventID,
    parameterCodes: parameterCodes,
    parameterValues: parameterValues,
    parameterCount: parameterCount,
    errorOut: errorOut)

  return sbDescriptorHandle(from: result)
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
  handle.object.setTo(sbCocoaValue(fromHandle: valueHandle))
  return true
}

@_cdecl("sb_object_release")
public func sb_object_release(_ rawHandle: UnsafeMutableRawPointer?) {
  guard let rawHandle else {
    return
  }

  sbRelease(rawHandle)
}
