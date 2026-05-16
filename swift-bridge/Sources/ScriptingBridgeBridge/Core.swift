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

func sbFourCharCode(_ string: String) -> UInt32? {
  let bytes = Array(string.utf8)
  guard bytes.count == 4 else {
    return nil
  }

  return bytes.reduce(0) { partial, byte in
    (partial << 8) | UInt32(byte)
  }
}

func sbFourCharString(_ code: UInt32) -> String {
  let bytes = [
    UInt8((code >> 24) & 0xff),
    UInt8((code >> 16) & 0xff),
    UInt8((code >> 8) & 0xff),
    UInt8(code & 0xff),
  ]
  return String(bytes: bytes, encoding: .macOSRoman) ?? "????"
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

  if let dictionary = value as? [String: Any] {
    guard dictionary.keys.allSatisfy({ sbFourCharCode($0) != nil }) else {
      return NSAppleEventDescriptor(string: String(describing: dictionary))
    }

    let descriptor = NSAppleEventDescriptor(recordDescriptor: ())
    for key in dictionary.keys.sorted() {
      guard let keyword = sbFourCharCode(key) else {
        continue
      }
      descriptor.setDescriptor(
        sbDescriptor(from: dictionary[key]) ?? NSAppleEventDescriptor.null(),
        forKeyword: keyword)
    }
    return descriptor
  }

  if let number = value as? NSNumber {
    if CFGetTypeID(number) == CFBooleanGetTypeID() {
      return NSAppleEventDescriptor(boolean: number.boolValue)
    }

    let objcType = String(cString: number.objCType)
    if objcType == "f" || objcType == "d" {
      return NSAppleEventDescriptor(double: number.doubleValue)
    }

    return NSAppleEventDescriptor(int32: number.int32Value)
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

  if descriptor.descriptorType == DescType(typeNull) {
    return nil
  }

  if descriptor.isRecordDescriptor {
    var record: [String: Any] = [:]
    for index in 1...descriptor.numberOfItems {
      let keyword = descriptor.keywordForDescriptor(at: index)
      record[sbFourCharString(keyword)] = sbCocoaValue(from: descriptor.atIndex(index)) ?? NSNull()
    }
    return record
  }

  if descriptor.descriptorType == DescType(typeAEList) {
    return (1...descriptor.numberOfItems).map {
      sbCocoaValue(from: descriptor.atIndex($0)) ?? NSNull()
    }
  }

  switch descriptor.descriptorType {
  case DescType(typeBoolean):
    return NSNumber(value: descriptor.booleanValue)
  case DescType(typeIEEE32BitFloatingPoint), DescType(typeIEEE64BitFloatingPoint):
    return NSNumber(value: descriptor.doubleValue)
  case DescType(typeEnumerated):
    return NSNumber(value: descriptor.enumCodeValue)
  case DescType(typeType):
    return NSNumber(value: descriptor.typeCodeValue)
  case DescType(typeFileURL):
    return descriptor.fileURLValue
  default:
    if let dateValue = descriptor.dateValue {
      return dateValue
    }
    if let stringValue = descriptor.stringValue {
      return stringValue
    }
    return NSNumber(value: descriptor.int32Value)
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
