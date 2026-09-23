import CoreServices
import Darwin
import Foundation
import ScriptingBridge

func sbCString(_ string: String) -> UnsafeMutablePointer<CChar>? {
  string.withCString { strdup($0) }
}

func sbRetain(_ object: AnyObject) -> UnsafeMutableRawPointer {
  Unmanaged.passRetained(object).toOpaque()
}

func sbBorrow<T: AnyObject>(_ pointer: UnsafeMutableRawPointer) -> T {
  Unmanaged<T>.fromOpaque(pointer).takeUnretainedValue()
}

func sbRelease(_ pointer: UnsafeMutableRawPointer) {
  Unmanaged<AnyObject>.fromOpaque(pointer).release()
}

func sbSetError(
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
  _ message: String
) {
  errorOut?.pointee = sbCString(message)
}

func sbNSErrorMessage(_ error: NSError) -> String {
  "\(error.domain) (\(error.code)): \(error.localizedDescription)"
}

func sbOSStatusMessage(_ status: OSStatus) -> String {
  sbNSErrorMessage(NSError(domain: NSOSStatusErrorDomain, code: Int(status)))
}

func sbURL(from rawValue: String) -> URL? {
  if let url = URL(string: rawValue), url.scheme != nil {
    return url
  }

  return URL(fileURLWithPath: rawValue)
}

private let sbEventErrorKey = "fish.doom.scriptingbridge.event-error"
private let sbUserRecordFieldsKeyword: AEKeyword = 0x7573_7266

func sbRecordEventError(_ error: NSError) {
  let dictionary = Thread.current.threadDictionary
  if dictionary[sbEventErrorKey] == nil {
    dictionary[sbEventErrorKey] = error
  }
}

func sbCaptureEventError<T>(_ body: () -> T) -> (T, NSError?) {
  let dictionary = Thread.current.threadDictionary
  let outer = dictionary[sbEventErrorKey]
  dictionary.removeObject(forKey: sbEventErrorKey)
  let value = body()
  let error = dictionary[sbEventErrorKey] as? NSError
  dictionary[sbEventErrorKey] = outer
  return (value, error)
}

func sbRun(
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
  _ call: (inout AnyObject?, inout NSString?) -> Bool
) -> (succeeded: Bool, value: AnyObject?) {
  autoreleasepool {
    var value: AnyObject?
    var message: NSString?
    let (succeeded, eventError) = sbCaptureEventError { call(&value, &message) }
    guard succeeded else {
      sbSetError(errorOut, (message as String?) ?? "the Scripting Bridge call failed")
      return (false, nil)
    }
    if let eventError {
      sbSetError(errorOut, sbNSErrorMessage(eventError))
      return (false, nil)
    }
    return (true, value)
  }
}

func sbDescriptor<T>(_ type: DescType, _ value: T) -> NSAppleEventDescriptor? {
  withUnsafeBytes(of: value) { bytes in
    NSAppleEventDescriptor(descriptorType: type, bytes: bytes.baseAddress, length: bytes.count)
  }
}

func sbInteger<T: FixedWidthInteger>(_ descriptor: NSAppleEventDescriptor, as type: T.Type) -> T? {
  let data = descriptor.data
  guard data.count == MemoryLayout<T>.size else {
    return nil
  }
  return data.withUnsafeBytes { $0.loadUnaligned(as: T.self) }
}

func sbRecordKeyword(_ key: AnyHashable) -> AEKeyword? {
  guard let number = key.base as? NSNumber, CFGetTypeID(number) != CFBooleanGetTypeID(),
    !CFNumberIsFloatType(number as CFNumber)
  else {
    return nil
  }
  return AEKeyword(exactly: number.int64Value)
}

func sbCopyDataBuffer(_ data: Data) -> UnsafeMutableRawPointer? {
  let capacity = max(data.count, 1)
  guard let buffer = malloc(capacity) else {
    return nil
  }

  if !data.isEmpty {
    data.copyBytes(to: buffer.assumingMemoryBound(to: UInt8.self), count: data.count)
  }

  return buffer
}

func sbDescriptor(fromHandle rawHandle: UnsafeMutableRawPointer?) -> NSAppleEventDescriptor? {
  guard let rawHandle else {
    return nil
  }

  let handle: SBRSAppleEventDescriptorHandle = sbBorrow(rawHandle)
  return handle.descriptor
}

func sbCocoaValue(fromHandle rawHandle: UnsafeMutableRawPointer?) -> Any? {
  sbCocoaValue(from: sbDescriptor(fromHandle: rawHandle))
}

func sbDescriptorHandle(from value: Any?) -> UnsafeMutableRawPointer? {
  guard let descriptor = sbDescriptor(from: value) else {
    return nil
  }

  return sbRetain(SBRSAppleEventDescriptorHandle(descriptor: descriptor))
}

func sbDescriptor(from value: Any?) -> NSAppleEventDescriptor? {
  guard let value else {
    return nil
  }

  if value is NSNull {
    return NSAppleEventDescriptor.null()
  }

  if let descriptor = value as? NSAppleEventDescriptor {
    return descriptor
  }

  if let string = value as? String {
    return NSAppleEventDescriptor(string: string)
  }

  if let url = value as? URL {
    if url.isFileURL {
      return NSAppleEventDescriptor(fileURL: url)
    }

    return NSAppleEventDescriptor(string: url.absoluteString)
  }

  if let date = value as? Date {
    return NSAppleEventDescriptor(date: date)
  }

  if let array = value as? [Any] {
    let descriptor = NSAppleEventDescriptor(listDescriptor: ())
    for (index, item) in array.enumerated() {
      descriptor.insert(sbDescriptor(from: item) ?? NSAppleEventDescriptor.null(), at: index + 1)
    }
    return descriptor
  }

  if let dictionary = value as? [AnyHashable: Any] {
    let descriptor = NSAppleEventDescriptor(recordDescriptor: ())
    var userFields: [(String, Any)] = []
    for (key, item) in dictionary {
      if let keyword = sbRecordKeyword(key) {
        descriptor.setDescriptor(
          sbDescriptor(from: item) ?? NSAppleEventDescriptor.null(), forKeyword: keyword)
      } else {
        userFields.append((String(describing: key.base), item))
      }
    }
    if !userFields.isEmpty {
      let fields = NSAppleEventDescriptor(listDescriptor: ())
      for (key, item) in userFields.sorted(by: { $0.0 < $1.0 }) {
        fields.insert(NSAppleEventDescriptor(string: key), at: fields.numberOfItems + 1)
        fields.insert(
          sbDescriptor(from: item) ?? NSAppleEventDescriptor.null(), at: fields.numberOfItems + 1)
      }
      descriptor.setDescriptor(fields, forKeyword: sbUserRecordFieldsKeyword)
    }
    return descriptor
  }

  if let number = value as? NSNumber {
    if CFGetTypeID(number) == CFBooleanGetTypeID() {
      return NSAppleEventDescriptor(boolean: number.boolValue)
    }

    if CFNumberIsFloatType(number as CFNumber) {
      return NSAppleEventDescriptor(double: number.doubleValue)
    }

    if String(cString: number.objCType) == "Q", Int64(exactly: number.uint64Value) == nil {
      return sbDescriptor(DescType(typeUInt64), number.uint64Value)
    }

    let integer = number.int64Value
    if let small = Int32(exactly: integer) {
      return NSAppleEventDescriptor(int32: small)
    }
    return sbDescriptor(DescType(typeSInt64), integer)
  }

  if let object = value as? SBObject {
    return NSAppleEventDescriptor(string: object.description)
  }

  if let array = value as? SBElementArray {
    return NSAppleEventDescriptor(string: array.description)
  }

  return NSAppleEventDescriptor(string: String(describing: value))
}

func sbCocoaValue(from descriptor: NSAppleEventDescriptor?) -> Any? {
  guard let descriptor else {
    return nil
  }

  switch descriptor.descriptorType {
  case DescType(typeNull):
    return nil
  case DescType(typeAEList):
    return stride(from: 1, through: descriptor.numberOfItems, by: 1).map {
      sbCocoaValue(from: descriptor.atIndex($0)) ?? NSNull()
    }
  case DescType(typeBoolean), DescType(typeTrue), DescType(typeFalse):
    return NSNumber(value: descriptor.booleanValue)
  case DescType(typeSInt16), DescType(typeSInt32):
    return NSNumber(value: descriptor.int32Value)
  case DescType(typeUInt16), DescType(typeUInt32), DescType(typeSInt64):
    guard let wide = descriptor.coerce(toDescriptorType: DescType(typeSInt64)),
      let value = sbInteger(wide, as: Int64.self)
    else {
      return descriptor
    }
    return Int32(exactly: value).map { NSNumber(value: $0) } ?? descriptor
  case DescType(typeUInt64):
    guard let value = sbInteger(descriptor, as: UInt64.self) else {
      return descriptor
    }
    return Int32(exactly: value).map { NSNumber(value: $0) } ?? descriptor
  case DescType(typeIEEE32BitFloatingPoint), DescType(typeIEEE64BitFloatingPoint):
    return NSNumber(value: descriptor.doubleValue)
  case DescType(typeEnumerated):
    return NSNumber(value: descriptor.enumCodeValue)
  case DescType(typeType):
    return NSNumber(value: descriptor.typeCodeValue)
  case DescType(typeFileURL):
    return descriptor.fileURLValue
  default:
    if descriptor.isRecordDescriptor {
      return descriptor
    }
    if let dateValue = descriptor.dateValue {
      return dateValue
    }
    if let stringValue = descriptor.stringValue {
      return stringValue
    }
    return descriptor
  }
}

func sbPropertyDictionary(
  names: UnsafePointer<UnsafePointer<CChar>?>?,
  values: UnsafePointer<UnsafeMutableRawPointer?>?,
  count: Int64,
  errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> [String: Any]? {
  guard count >= 0 else {
    sbSetError(errorOut, "property count cannot be negative")
    return nil
  }

  let count = Int(count)
  guard count == 0 || (names != nil && values != nil) else {
    sbSetError(errorOut, "missing property names or values")
    return nil
  }

  guard let names, let values else {
    return [:]
  }

  let nameBuffer = UnsafeBufferPointer(start: names, count: count)
  let valueBuffer = UnsafeBufferPointer(start: values, count: count)
  var result: [String: Any] = [:]
  for index in 0..<count {
    guard let namePointer = nameBuffer[index] else {
      sbSetError(errorOut, "property name at index \(index) was null")
      return nil
    }
    result[String(cString: namePointer)] = sbCocoaValue(fromHandle: valueBuffer[index]) ?? NSNull()
  }

  return result
}

@_cdecl("sb_string_free")
public func sb_string_free(_ pointer: UnsafeMutablePointer<CChar>?) {
  free(pointer)
}

@_cdecl("sb_buffer_free")
public func sb_buffer_free(_ pointer: UnsafeMutableRawPointer?) {
  free(pointer)
}
