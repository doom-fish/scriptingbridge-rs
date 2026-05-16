import CoreServices
import Foundation

final class SBRSAppleEventDescriptorHandle: NSObject {
  let descriptor: NSAppleEventDescriptor

  init(descriptor: NSAppleEventDescriptor) {
    self.descriptor = descriptor
  }
}

final class SBRSRawAEDescHandle: NSObject {
  var descriptor: AEDesc

  init(descriptor: AEDesc) {
    self.descriptor = descriptor
  }

  deinit {
    _ = AEDisposeDesc(&descriptor)
  }
}

@_cdecl("sb_apple_event_descriptor_null")
public func sb_apple_event_descriptor_null() -> UnsafeMutableRawPointer? {
  sbRetain(SBRSAppleEventDescriptorHandle(descriptor: NSAppleEventDescriptor.null()))
}

@_cdecl("sb_apple_event_descriptor_create_with_descriptor_type_bytes")
public func sb_apple_event_descriptor_create_with_descriptor_type_bytes(
  _ descriptorType: UInt32,
  _ bytes: UnsafePointer<UInt8>?,
  _ length: Int64,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
  guard length >= 0 else {
    sbSetError(errorOut, "descriptor byte length cannot be negative")
    return nil
  }

  let descriptor = NSAppleEventDescriptor(
    descriptorType: descriptorType,
    bytes: bytes,
    length: Int(length))
  return descriptor.map { sbRetain(SBRSAppleEventDescriptorHandle(descriptor: $0)) }
}

@_cdecl("sb_apple_event_descriptor_create_with_descriptor_type_data")
public func sb_apple_event_descriptor_create_with_descriptor_type_data(
  _ descriptorType: UInt32,
  _ bytes: UnsafePointer<UInt8>?,
  _ length: Int64,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
  guard length >= 0 else {
    sbSetError(errorOut, "descriptor data length cannot be negative")
    return nil
  }

  let data = bytes.map { Data(bytes: $0, count: Int(length)) }
  let descriptor = NSAppleEventDescriptor(descriptorType: descriptorType, data: data)
  return descriptor.map { sbRetain(SBRSAppleEventDescriptorHandle(descriptor: $0)) }
}

@_cdecl("sb_apple_event_descriptor_create_with_boolean")
public func sb_apple_event_descriptor_create_with_boolean(_ value: Bool) -> UnsafeMutableRawPointer? {
  sbRetain(SBRSAppleEventDescriptorHandle(descriptor: NSAppleEventDescriptor(boolean: value)))
}

@_cdecl("sb_apple_event_descriptor_create_with_enum_code")
public func sb_apple_event_descriptor_create_with_enum_code(_ value: UInt32) -> UnsafeMutableRawPointer? {
  sbRetain(SBRSAppleEventDescriptorHandle(descriptor: NSAppleEventDescriptor(enumCode: value)))
}

@_cdecl("sb_apple_event_descriptor_create_with_int32")
public func sb_apple_event_descriptor_create_with_int32(_ value: Int32) -> UnsafeMutableRawPointer? {
  sbRetain(SBRSAppleEventDescriptorHandle(descriptor: NSAppleEventDescriptor(int32: value)))
}

@_cdecl("sb_apple_event_descriptor_create_with_double")
public func sb_apple_event_descriptor_create_with_double(_ value: Double) -> UnsafeMutableRawPointer? {
  sbRetain(SBRSAppleEventDescriptorHandle(descriptor: NSAppleEventDescriptor(double: value)))
}

@_cdecl("sb_apple_event_descriptor_create_with_type_code")
public func sb_apple_event_descriptor_create_with_type_code(_ value: UInt32) -> UnsafeMutableRawPointer? {
  sbRetain(SBRSAppleEventDescriptorHandle(descriptor: NSAppleEventDescriptor(typeCode: value)))
}

@_cdecl("sb_apple_event_descriptor_create_with_string")
public func sb_apple_event_descriptor_create_with_string(_ value: UnsafePointer<CChar>?) -> UnsafeMutableRawPointer? {
  guard let value else {
    return nil
  }

  return sbRetain(
    SBRSAppleEventDescriptorHandle(
      descriptor: NSAppleEventDescriptor(string: String(cString: value))))
}

@_cdecl("sb_apple_event_descriptor_create_with_date")
public func sb_apple_event_descriptor_create_with_date(_ value: Double) -> UnsafeMutableRawPointer? {
  sbRetain(
    SBRSAppleEventDescriptorHandle(
      descriptor: NSAppleEventDescriptor(date: Date(timeIntervalSince1970: value))))
}

@_cdecl("sb_apple_event_descriptor_create_with_file_url")
public func sb_apple_event_descriptor_create_with_file_url(
  _ value: UnsafePointer<CChar>?,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
  guard let value else {
    sbSetError(errorOut, "missing file URL string")
    return nil
  }

  let rawURL = String(cString: value)
  guard let url = sbURL(from: rawURL), url.isFileURL else {
    sbSetError(errorOut, "\(rawURL) is not a valid file URL or file path")
    return nil
  }

  return sbRetain(SBRSAppleEventDescriptorHandle(descriptor: NSAppleEventDescriptor(fileURL: url)))
}

@_cdecl("sb_apple_event_descriptor_create_apple_event")
public func sb_apple_event_descriptor_create_apple_event(
  _ eventClass: UInt32,
  _ eventID: UInt32,
  _ targetDescriptorHandle: UnsafeMutableRawPointer?,
  _ returnID: Int16,
  _ transactionID: Int32,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
  let descriptor = NSAppleEventDescriptor(
    eventClass: eventClass,
    eventID: eventID,
    targetDescriptor: sbDescriptor(fromHandle: targetDescriptorHandle),
    returnID: returnID,
    transactionID: transactionID)
  return sbRetain(SBRSAppleEventDescriptorHandle(descriptor: descriptor))
}

@_cdecl("sb_apple_event_descriptor_create_list")
public func sb_apple_event_descriptor_create_list() -> UnsafeMutableRawPointer? {
  sbRetain(SBRSAppleEventDescriptorHandle(descriptor: NSAppleEventDescriptor(listDescriptor: ())))
}

@_cdecl("sb_apple_event_descriptor_create_record")
public func sb_apple_event_descriptor_create_record() -> UnsafeMutableRawPointer? {
  sbRetain(SBRSAppleEventDescriptorHandle(descriptor: NSAppleEventDescriptor(recordDescriptor: ())))
}

@_cdecl("sb_apple_event_descriptor_current_process")
public func sb_apple_event_descriptor_current_process() -> UnsafeMutableRawPointer? {
  sbRetain(SBRSAppleEventDescriptorHandle(descriptor: NSAppleEventDescriptor.currentProcess()))
}

@_cdecl("sb_apple_event_descriptor_create_with_process_identifier")
public func sb_apple_event_descriptor_create_with_process_identifier(_ value: Int32) -> UnsafeMutableRawPointer? {
  sbRetain(
    SBRSAppleEventDescriptorHandle(
      descriptor: NSAppleEventDescriptor(processIdentifier: value)))
}

@_cdecl("sb_apple_event_descriptor_create_with_bundle_identifier")
public func sb_apple_event_descriptor_create_with_bundle_identifier(
  _ value: UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer? {
  guard let value else {
    return nil
  }

  return sbRetain(
    SBRSAppleEventDescriptorHandle(
      descriptor: NSAppleEventDescriptor(bundleIdentifier: String(cString: value))))
}

@_cdecl("sb_apple_event_descriptor_create_with_application_url")
public func sb_apple_event_descriptor_create_with_application_url(
  _ value: UnsafePointer<CChar>?,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
  guard let value else {
    sbSetError(errorOut, "missing application URL string")
    return nil
  }

  let rawURL = String(cString: value)
  guard let url = sbURL(from: rawURL) else {
    sbSetError(errorOut, "could not parse application URL \(rawURL)")
    return nil
  }

  return sbRetain(
    SBRSAppleEventDescriptorHandle(
      descriptor: NSAppleEventDescriptor(applicationURL: url)))
}

@_cdecl("sb_apple_event_descriptor_copy_aedesc")
public func sb_apple_event_descriptor_copy_aedesc(
  _ rawHandle: UnsafeMutableRawPointer?,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
  guard let rawHandle else {
    sbSetError(errorOut, "missing NSAppleEventDescriptor handle")
    return nil
  }

  let handle: SBRSAppleEventDescriptorHandle = sbBorrow(rawHandle)
  guard let sourceDescriptor = handle.descriptor.aeDesc else {
    sbSetError(errorOut, "descriptor does not expose an AEDesc")
    return nil
  }

  var copiedDescriptor = AEDesc()
  let status = AEDuplicateDesc(sourceDescriptor, &copiedDescriptor)
  if status != noErr {
    sbSetError(errorOut, sbOSStatusMessage(OSStatus(status)))
    return nil
  }

  return sbRetain(SBRSRawAEDescHandle(descriptor: copiedDescriptor))
}

@_cdecl("sb_apple_event_descriptor_create_with_aedesc_no_copy")
public func sb_apple_event_descriptor_create_with_aedesc_no_copy(
  _ rawDescriptorHandle: UnsafeMutableRawPointer?,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
  guard let rawDescriptorHandle else {
    sbSetError(errorOut, "missing raw AEDesc handle")
    return nil
  }

  let rawHandle: SBRSRawAEDescHandle = sbBorrow(rawDescriptorHandle)
  var copiedDescriptor = AEDesc()
  let status = AEDuplicateDesc(&rawHandle.descriptor, &copiedDescriptor)
  if status != noErr {
    sbSetError(errorOut, sbOSStatusMessage(OSStatus(status)))
    return nil
  }

  rawHandle.descriptor = AEDesc()
  sbRelease(rawDescriptorHandle)
  let descriptor = NSAppleEventDescriptor(aeDescNoCopy: &copiedDescriptor)
  return sbRetain(SBRSAppleEventDescriptorHandle(descriptor: descriptor))
}

@_cdecl("sb_aedesc_descriptor_type")
public func sb_aedesc_descriptor_type(_ rawDescriptorHandle: UnsafeMutableRawPointer?) -> UInt32 {
  guard let rawDescriptorHandle else {
    return 0
  }

  let rawHandle: SBRSRawAEDescHandle = sbBorrow(rawDescriptorHandle)
  return rawHandle.descriptor.descriptorType
}

@_cdecl("sb_aedesc_release")
public func sb_aedesc_release(_ rawDescriptorHandle: UnsafeMutableRawPointer?) {
  guard let rawDescriptorHandle else {
    return
  }

  sbRelease(rawDescriptorHandle)
}

@_cdecl("sb_apple_event_descriptor_descriptor_type")
public func sb_apple_event_descriptor_descriptor_type(_ rawHandle: UnsafeMutableRawPointer?) -> UInt32 {
  guard let rawHandle else {
    return 0
  }

  let handle: SBRSAppleEventDescriptorHandle = sbBorrow(rawHandle)
  return handle.descriptor.descriptorType
}

@_cdecl("sb_apple_event_descriptor_copy_data")
public func sb_apple_event_descriptor_copy_data(
  _ rawHandle: UnsafeMutableRawPointer?,
  _ lengthOut: UnsafeMutablePointer<Int64>?
) -> UnsafeMutablePointer<UInt8>? {
  guard let rawHandle else {
    lengthOut?.pointee = 0
    return nil
  }

  let handle: SBRSAppleEventDescriptorHandle = sbBorrow(rawHandle)
  let data = handle.descriptor.data
  lengthOut?.pointee = Int64(data.count)
  return sbCopyDataBuffer(data)?.assumingMemoryBound(to: UInt8.self)
}

@_cdecl("sb_apple_event_descriptor_boolean_value")
public func sb_apple_event_descriptor_boolean_value(_ rawHandle: UnsafeMutableRawPointer?) -> Bool {
  guard let rawHandle else {
    return false
  }

  let handle: SBRSAppleEventDescriptorHandle = sbBorrow(rawHandle)
  return handle.descriptor.booleanValue
}

@_cdecl("sb_apple_event_descriptor_enum_code_value")
public func sb_apple_event_descriptor_enum_code_value(_ rawHandle: UnsafeMutableRawPointer?) -> UInt32 {
  guard let rawHandle else {
    return 0
  }

  let handle: SBRSAppleEventDescriptorHandle = sbBorrow(rawHandle)
  return handle.descriptor.enumCodeValue
}

@_cdecl("sb_apple_event_descriptor_int32_value")
public func sb_apple_event_descriptor_int32_value(_ rawHandle: UnsafeMutableRawPointer?) -> Int32 {
  guard let rawHandle else {
    return 0
  }

  let handle: SBRSAppleEventDescriptorHandle = sbBorrow(rawHandle)
  return handle.descriptor.int32Value
}

@_cdecl("sb_apple_event_descriptor_double_value")
public func sb_apple_event_descriptor_double_value(_ rawHandle: UnsafeMutableRawPointer?) -> Double {
  guard let rawHandle else {
    return 0
  }

  let handle: SBRSAppleEventDescriptorHandle = sbBorrow(rawHandle)
  return handle.descriptor.doubleValue
}

@_cdecl("sb_apple_event_descriptor_type_code_value")
public func sb_apple_event_descriptor_type_code_value(_ rawHandle: UnsafeMutableRawPointer?) -> UInt32 {
  guard let rawHandle else {
    return 0
  }

  let handle: SBRSAppleEventDescriptorHandle = sbBorrow(rawHandle)
  return handle.descriptor.typeCodeValue
}

@_cdecl("sb_apple_event_descriptor_string_value")
public func sb_apple_event_descriptor_string_value(_ rawHandle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
  guard let rawHandle else {
    return nil
  }

  let handle: SBRSAppleEventDescriptorHandle = sbBorrow(rawHandle)
  return handle.descriptor.stringValue.flatMap(sbCString)
}

@_cdecl("sb_apple_event_descriptor_date_value")
public func sb_apple_event_descriptor_date_value(_ rawHandle: UnsafeMutableRawPointer?) -> Double {
  guard let rawHandle else {
    return .nan
  }

  let handle: SBRSAppleEventDescriptorHandle = sbBorrow(rawHandle)
  return handle.descriptor.dateValue?.timeIntervalSince1970 ?? .nan
}

@_cdecl("sb_apple_event_descriptor_file_url_value")
public func sb_apple_event_descriptor_file_url_value(_ rawHandle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
  guard let rawHandle else {
    return nil
  }

  let handle: SBRSAppleEventDescriptorHandle = sbBorrow(rawHandle)
  guard let absoluteString = handle.descriptor.fileURLValue?.absoluteString else {
    return nil
  }

  return sbCString(absoluteString)
}

@_cdecl("sb_apple_event_descriptor_event_class")
public func sb_apple_event_descriptor_event_class(_ rawHandle: UnsafeMutableRawPointer?) -> UInt32 {
  guard let rawHandle else {
    return 0
  }

  let handle: SBRSAppleEventDescriptorHandle = sbBorrow(rawHandle)
  return handle.descriptor.eventClass
}

@_cdecl("sb_apple_event_descriptor_event_id")
public func sb_apple_event_descriptor_event_id(_ rawHandle: UnsafeMutableRawPointer?) -> UInt32 {
  guard let rawHandle else {
    return 0
  }

  let handle: SBRSAppleEventDescriptorHandle = sbBorrow(rawHandle)
  return handle.descriptor.eventID
}

@_cdecl("sb_apple_event_descriptor_return_id")
public func sb_apple_event_descriptor_return_id(_ rawHandle: UnsafeMutableRawPointer?) -> Int16 {
  guard let rawHandle else {
    return 0
  }

  let handle: SBRSAppleEventDescriptorHandle = sbBorrow(rawHandle)
  return handle.descriptor.returnID
}

@_cdecl("sb_apple_event_descriptor_transaction_id")
public func sb_apple_event_descriptor_transaction_id(_ rawHandle: UnsafeMutableRawPointer?) -> Int32 {
  guard let rawHandle else {
    return 0
  }

  let handle: SBRSAppleEventDescriptorHandle = sbBorrow(rawHandle)
  return handle.descriptor.transactionID
}

@_cdecl("sb_apple_event_descriptor_set_param_descriptor")
public func sb_apple_event_descriptor_set_param_descriptor(
  _ rawHandle: UnsafeMutableRawPointer?,
  _ descriptorHandle: UnsafeMutableRawPointer?,
  _ keyword: UInt32,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
  guard let rawHandle else {
    sbSetError(errorOut, "missing NSAppleEventDescriptor handle")
    return false
  }
  guard let descriptor = sbDescriptor(fromHandle: descriptorHandle) else {
    sbSetError(errorOut, "missing parameter descriptor")
    return false
  }

  let handle: SBRSAppleEventDescriptorHandle = sbBorrow(rawHandle)
  handle.descriptor.setParam(descriptor, forKeyword: keyword)
  return true
}

@_cdecl("sb_apple_event_descriptor_param_descriptor_for_keyword")
public func sb_apple_event_descriptor_param_descriptor_for_keyword(
  _ rawHandle: UnsafeMutableRawPointer?,
  _ keyword: UInt32,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
  guard let rawHandle else {
    sbSetError(errorOut, "missing NSAppleEventDescriptor handle")
    return nil
  }

  let handle: SBRSAppleEventDescriptorHandle = sbBorrow(rawHandle)
  return handle.descriptor.paramDescriptor(forKeyword: keyword).map {
    sbRetain(SBRSAppleEventDescriptorHandle(descriptor: $0))
  }
}

@_cdecl("sb_apple_event_descriptor_remove_param_descriptor")
public func sb_apple_event_descriptor_remove_param_descriptor(
  _ rawHandle: UnsafeMutableRawPointer?,
  _ keyword: UInt32,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
  guard let rawHandle else {
    sbSetError(errorOut, "missing NSAppleEventDescriptor handle")
    return false
  }

  let handle: SBRSAppleEventDescriptorHandle = sbBorrow(rawHandle)
  handle.descriptor.removeParamDescriptor(withKeyword: keyword)
  return true
}

@_cdecl("sb_apple_event_descriptor_set_attribute_descriptor")
public func sb_apple_event_descriptor_set_attribute_descriptor(
  _ rawHandle: UnsafeMutableRawPointer?,
  _ descriptorHandle: UnsafeMutableRawPointer?,
  _ keyword: UInt32,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
  guard let rawHandle else {
    sbSetError(errorOut, "missing NSAppleEventDescriptor handle")
    return false
  }
  guard let descriptor = sbDescriptor(fromHandle: descriptorHandle) else {
    sbSetError(errorOut, "missing attribute descriptor")
    return false
  }

  let handle: SBRSAppleEventDescriptorHandle = sbBorrow(rawHandle)
  handle.descriptor.setAttribute(descriptor, forKeyword: keyword)
  return true
}

@_cdecl("sb_apple_event_descriptor_attribute_descriptor_for_keyword")
public func sb_apple_event_descriptor_attribute_descriptor_for_keyword(
  _ rawHandle: UnsafeMutableRawPointer?,
  _ keyword: UInt32,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
  guard let rawHandle else {
    sbSetError(errorOut, "missing NSAppleEventDescriptor handle")
    return nil
  }

  let handle: SBRSAppleEventDescriptorHandle = sbBorrow(rawHandle)
  return handle.descriptor.attributeDescriptor(forKeyword: keyword).map {
    sbRetain(SBRSAppleEventDescriptorHandle(descriptor: $0))
  }
}

@_cdecl("sb_apple_event_descriptor_send_event")
public func sb_apple_event_descriptor_send_event(
  _ rawHandle: UnsafeMutableRawPointer?,
  _ sendOptions: UInt64,
  _ timeout: Double,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
  guard let rawHandle else {
    sbSetError(errorOut, "missing NSAppleEventDescriptor handle")
    return nil
  }

  let handle: SBRSAppleEventDescriptorHandle = sbBorrow(rawHandle)
  do {
    let reply = try handle.descriptor.sendEvent(
      options: NSAppleEventDescriptor.SendOptions(rawValue: UInt(sendOptions)),
      timeout: timeout)
    return sbRetain(SBRSAppleEventDescriptorHandle(descriptor: reply))
  } catch {
    sbSetError(errorOut, sbNSErrorMessage(error as NSError))
    return nil
  }
}

@_cdecl("sb_apple_event_descriptor_is_record_descriptor")
public func sb_apple_event_descriptor_is_record_descriptor(_ rawHandle: UnsafeMutableRawPointer?) -> Bool {
  guard let rawHandle else {
    return false
  }

  let handle: SBRSAppleEventDescriptorHandle = sbBorrow(rawHandle)
  return handle.descriptor.isRecordDescriptor
}

@_cdecl("sb_apple_event_descriptor_number_of_items")
public func sb_apple_event_descriptor_number_of_items(_ rawHandle: UnsafeMutableRawPointer?) -> Int64 {
  guard let rawHandle else {
    return 0
  }

  let handle: SBRSAppleEventDescriptorHandle = sbBorrow(rawHandle)
  return Int64(handle.descriptor.numberOfItems)
}

@_cdecl("sb_apple_event_descriptor_insert_descriptor")
public func sb_apple_event_descriptor_insert_descriptor(
  _ rawHandle: UnsafeMutableRawPointer?,
  _ descriptorHandle: UnsafeMutableRawPointer?,
  _ index: Int64,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
  guard let rawHandle else {
    sbSetError(errorOut, "missing NSAppleEventDescriptor handle")
    return false
  }
  guard let descriptor = sbDescriptor(fromHandle: descriptorHandle) else {
    sbSetError(errorOut, "missing descriptor to insert")
    return false
  }
  guard index >= 0 else {
    sbSetError(errorOut, "descriptor index cannot be negative")
    return false
  }

  let handle: SBRSAppleEventDescriptorHandle = sbBorrow(rawHandle)
  handle.descriptor.insert(descriptor, at: Int(index))
  return true
}

@_cdecl("sb_apple_event_descriptor_descriptor_at_index")
public func sb_apple_event_descriptor_descriptor_at_index(
  _ rawHandle: UnsafeMutableRawPointer?,
  _ index: Int64,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
  guard let rawHandle else {
    sbSetError(errorOut, "missing NSAppleEventDescriptor handle")
    return nil
  }
  guard index >= 0 else {
    sbSetError(errorOut, "descriptor index cannot be negative")
    return nil
  }

  let handle: SBRSAppleEventDescriptorHandle = sbBorrow(rawHandle)
  return handle.descriptor.atIndex(Int(index)).map {
    sbRetain(SBRSAppleEventDescriptorHandle(descriptor: $0))
  }
}

@_cdecl("sb_apple_event_descriptor_remove_descriptor_at_index")
public func sb_apple_event_descriptor_remove_descriptor_at_index(
  _ rawHandle: UnsafeMutableRawPointer?,
  _ index: Int64,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
  guard let rawHandle else {
    sbSetError(errorOut, "missing NSAppleEventDescriptor handle")
    return false
  }
  guard index >= 0 else {
    sbSetError(errorOut, "descriptor index cannot be negative")
    return false
  }

  let handle: SBRSAppleEventDescriptorHandle = sbBorrow(rawHandle)
  handle.descriptor.remove(at: Int(index))
  return true
}

@_cdecl("sb_apple_event_descriptor_set_descriptor")
public func sb_apple_event_descriptor_set_descriptor(
  _ rawHandle: UnsafeMutableRawPointer?,
  _ descriptorHandle: UnsafeMutableRawPointer?,
  _ keyword: UInt32,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
  guard let rawHandle else {
    sbSetError(errorOut, "missing NSAppleEventDescriptor handle")
    return false
  }
  guard let descriptor = sbDescriptor(fromHandle: descriptorHandle) else {
    sbSetError(errorOut, "missing record descriptor")
    return false
  }

  let handle: SBRSAppleEventDescriptorHandle = sbBorrow(rawHandle)
  handle.descriptor.setDescriptor(descriptor, forKeyword: keyword)
  return true
}

@_cdecl("sb_apple_event_descriptor_descriptor_for_keyword")
public func sb_apple_event_descriptor_descriptor_for_keyword(
  _ rawHandle: UnsafeMutableRawPointer?,
  _ keyword: UInt32,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
  guard let rawHandle else {
    sbSetError(errorOut, "missing NSAppleEventDescriptor handle")
    return nil
  }

  let handle: SBRSAppleEventDescriptorHandle = sbBorrow(rawHandle)
  return handle.descriptor.forKeyword(keyword).map {
    sbRetain(SBRSAppleEventDescriptorHandle(descriptor: $0))
  }
}

@_cdecl("sb_apple_event_descriptor_remove_descriptor_for_keyword")
public func sb_apple_event_descriptor_remove_descriptor_for_keyword(
  _ rawHandle: UnsafeMutableRawPointer?,
  _ keyword: UInt32,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
  guard let rawHandle else {
    sbSetError(errorOut, "missing NSAppleEventDescriptor handle")
    return false
  }

  let handle: SBRSAppleEventDescriptorHandle = sbBorrow(rawHandle)
  handle.descriptor.remove(withKeyword: keyword)
  return true
}

@_cdecl("sb_apple_event_descriptor_keyword_for_descriptor_at_index")
public func sb_apple_event_descriptor_keyword_for_descriptor_at_index(
  _ rawHandle: UnsafeMutableRawPointer?,
  _ index: Int64,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UInt32 {
  guard let rawHandle else {
    sbSetError(errorOut, "missing NSAppleEventDescriptor handle")
    return 0
  }
  guard index >= 0 else {
    sbSetError(errorOut, "descriptor index cannot be negative")
    return 0
  }

  let handle: SBRSAppleEventDescriptorHandle = sbBorrow(rawHandle)
  return handle.descriptor.keywordForDescriptor(at: Int(index))
}

@_cdecl("sb_apple_event_descriptor_coerce_to_descriptor_type")
public func sb_apple_event_descriptor_coerce_to_descriptor_type(
  _ rawHandle: UnsafeMutableRawPointer?,
  _ descriptorType: UInt32,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
  guard let rawHandle else {
    sbSetError(errorOut, "missing NSAppleEventDescriptor handle")
    return nil
  }

  let handle: SBRSAppleEventDescriptorHandle = sbBorrow(rawHandle)
  return handle.descriptor.coerce(toDescriptorType: descriptorType).map {
    sbRetain(SBRSAppleEventDescriptorHandle(descriptor: $0))
  }
}

@_cdecl("sb_apple_event_descriptor_release")
public func sb_apple_event_descriptor_release(_ rawHandle: UnsafeMutableRawPointer?) {
  guard let rawHandle else {
    return
  }

  sbRelease(rawHandle)
}
