import Foundation
import ScriptingBridge

final class SBRSElementArrayHandle: NSObject {
  let array: SBElementArray

  init(array: SBElementArray) {
    self.array = array
  }
}

@_cdecl("sb_element_array_object_with_name")
public func sb_element_array_object_with_name(
  _ rawHandle: UnsafeMutableRawPointer?,
  _ namePointer: UnsafePointer<CChar>?,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
  guard let rawHandle else {
    sbSetError(errorOut, "missing SBElementArray handle")
    return nil
  }
  guard let namePointer else {
    sbSetError(errorOut, "missing SBElementArray name")
    return nil
  }

  let handle: SBRSElementArrayHandle = sbBorrow(rawHandle)
  let object = handle.array.object(withName: String(cString: namePointer))
  guard let scriptingObject = object as? SBObject else {
    return nil
  }

  return sbRetain(SBRSObjectHandle(object: scriptingObject))
}

@_cdecl("sb_element_array_object_with_id")
public func sb_element_array_object_with_id(
  _ rawHandle: UnsafeMutableRawPointer?,
  _ identifierHandle: UnsafeMutableRawPointer?,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
  guard let rawHandle else {
    sbSetError(errorOut, "missing SBElementArray handle")
    return nil
  }
  guard let identifier = sbCocoaValue(fromHandle: identifierHandle) else {
    sbSetError(errorOut, "missing SBElementArray identifier")
    return nil
  }

  let handle: SBRSElementArrayHandle = sbBorrow(rawHandle)
  let object = handle.array.object(withID: identifier)
  guard let scriptingObject = object as? SBObject else {
    return nil
  }

  return sbRetain(SBRSObjectHandle(object: scriptingObject))
}

@_cdecl("sb_element_array_object_at_location")
public func sb_element_array_object_at_location(
  _ rawHandle: UnsafeMutableRawPointer?,
  _ locationHandle: UnsafeMutableRawPointer?,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
  guard let rawHandle else {
    sbSetError(errorOut, "missing SBElementArray handle")
    return nil
  }
  guard let location = sbCocoaValue(fromHandle: locationHandle) else {
    sbSetError(errorOut, "missing SBElementArray location")
    return nil
  }

  let handle: SBRSElementArrayHandle = sbBorrow(rawHandle)
  let object = handle.array.object(atLocation: location)
  guard let scriptingObject = object as? SBObject else {
    return nil
  }

  return sbRetain(SBRSObjectHandle(object: scriptingObject))
}

@_cdecl("sb_element_array_array_by_applying_selector")
public func sb_element_array_array_by_applying_selector(
  _ rawHandle: UnsafeMutableRawPointer?,
  _ selectorPointer: UnsafePointer<CChar>?,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
  guard let rawHandle else {
    sbSetError(errorOut, "missing SBElementArray handle")
    return nil
  }
  guard let selectorPointer else {
    sbSetError(errorOut, "missing selector string")
    return nil
  }

  let handle: SBRSElementArrayHandle = sbBorrow(rawHandle)
  let selector = NSSelectorFromString(String(cString: selectorPointer))
  return sbDescriptorHandle(from: handle.array.array(byApplying: selector))
}

@_cdecl("sb_element_array_array_by_applying_selector_with_object")
public func sb_element_array_array_by_applying_selector_with_object(
  _ rawHandle: UnsafeMutableRawPointer?,
  _ selectorPointer: UnsafePointer<CChar>?,
  _ argumentHandle: UnsafeMutableRawPointer?,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
  guard let rawHandle else {
    sbSetError(errorOut, "missing SBElementArray handle")
    return nil
  }
  guard let selectorPointer else {
    sbSetError(errorOut, "missing selector string")
    return nil
  }

  let handle: SBRSElementArrayHandle = sbBorrow(rawHandle)
  let selector = NSSelectorFromString(String(cString: selectorPointer))
  let argument = sbCocoaValue(fromHandle: argumentHandle) ?? NSNull()
  return sbDescriptorHandle(from: handle.array.array(byApplying: selector, with: argument))
}

@_cdecl("sb_element_array_get")
public func sb_element_array_get(
  _ rawHandle: UnsafeMutableRawPointer?,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
  guard let rawHandle else {
    sbSetError(errorOut, "missing SBElementArray handle")
    return nil
  }

  let handle: SBRSElementArrayHandle = sbBorrow(rawHandle)
  return sbDescriptorHandle(from: handle.array.get())
}

@_cdecl("sb_element_array_description")
public func sb_element_array_description(_ rawHandle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
  guard let rawHandle else {
    return nil
  }

  let handle: SBRSElementArrayHandle = sbBorrow(rawHandle)
  return sbCString(handle.array.description)
}

@_cdecl("sb_element_array_get_description")
public func sb_element_array_get_description(_ rawHandle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
  guard let rawHandle else {
    return nil
  }

  let handle: SBRSElementArrayHandle = sbBorrow(rawHandle)
  guard let value = handle.array.get() else {
    return nil
  }

  return sbCString(String(describing: value))
}

@_cdecl("sb_element_array_release")
public func sb_element_array_release(_ rawHandle: UnsafeMutableRawPointer?) {
  guard let rawHandle else {
    return
  }

  sbRelease(rawHandle)
}
