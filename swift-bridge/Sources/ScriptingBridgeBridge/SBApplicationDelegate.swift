import CoreServices
import Foundation
import ScriptingBridge

public typealias SBRSApplicationDelegateCallback = @convention(c) (
  UnsafeMutableRawPointer?,
  UInt32,
  UInt32,
  UnsafePointer<CChar>?,
  Int64,
  UnsafePointer<CChar>?
) -> UnsafeMutableRawPointer?

public typealias SBRSApplicationDelegateDropContext = @convention(c) (UnsafeMutableRawPointer?) -> Void

final class SBRSApplicationDelegateHandle: NSObject, SBApplicationDelegate {
  let context: UnsafeMutableRawPointer?
  let callback: SBRSApplicationDelegateCallback
  let dropContext: SBRSApplicationDelegateDropContext

  init(
    context: UnsafeMutableRawPointer?,
    callback: @escaping SBRSApplicationDelegateCallback,
    dropContext: @escaping SBRSApplicationDelegateDropContext
  ) {
    self.context = context
    self.callback = callback
    self.dropContext = dropContext
  }

  deinit {
    dropContext(context)
  }

  func eventDidFail(_ event: UnsafePointer<AppleEvent>, withError error: Error) -> Any? {
    let error = error as NSError
    var eventClass: UInt32 = 0
    var eventID: UInt32 = 0
    var eventCopy = AEDesc()
    if AEDuplicateDesc(event, &eventCopy) == noErr {
      let descriptor = NSAppleEventDescriptor(aeDescNoCopy: &eventCopy)
      eventClass = descriptor.eventClass
      eventID = descriptor.eventID
    }

    let rawResult = error.domain.withCString { domainPointer in
      error.localizedDescription.withCString { messagePointer in
        callback(context, eventClass, eventID, domainPointer, Int64(error.code), messagePointer)
      }
    }

    guard let rawResult else {
      return nil
    }

    let handle: SBRSAppleEventDescriptorHandle = sbBorrow(rawResult)
    let value = sbCocoaValue(from: handle.descriptor)
    sbRelease(rawResult)
    return value
  }
}

@_cdecl("sb_application_delegate_create")
public func sb_application_delegate_create(
  _ context: UnsafeMutableRawPointer?,
  _ callback: SBRSApplicationDelegateCallback?,
  _ dropContext: SBRSApplicationDelegateDropContext?,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
  guard let callback else {
    sbSetError(errorOut, "missing application delegate callback")
    return nil
  }
  guard let dropContext else {
    sbSetError(errorOut, "missing application delegate drop callback")
    return nil
  }

  return sbRetain(
    SBRSApplicationDelegateHandle(
      context: context,
      callback: callback,
      dropContext: dropContext))
}

@_cdecl("sb_application_delegate_release")
public func sb_application_delegate_release(_ rawHandle: UnsafeMutableRawPointer?) {
  guard let rawHandle else {
    return
  }

  sbRelease(rawHandle)
}
