import AppKit
import CoreServices
import Foundation
import ScriptingBridge

final class SBRSApplicationHandle: NSObject {
  let application: SBApplication
  let bundleIdentifier: String?
  let url: URL?
  let processIdentifier: pid_t?

  init(
    application: SBApplication,
    bundleIdentifier: String? = nil,
    url: URL? = nil,
    processIdentifier: pid_t? = nil
  ) {
    self.application = application
    self.bundleIdentifier = bundleIdentifier
    self.url = url
    self.processIdentifier = processIdentifier
  }
}

private let sbLSLaunchDontSwitch: UInt32 = 0x00000200
private let sbLSLaunchNewInstance: UInt32 = 0x00080000

final class SBRSScriptingClassHandle: NSObject {
  let scriptingClass: AnyClass

  init(scriptingClass: AnyClass) {
    self.scriptingClass = scriptingClass
  }
}

private func sbRetainedApplicationHandle(
  application: SBApplication,
  bundleIdentifier: String? = nil,
  url: URL? = nil,
  processIdentifier: pid_t? = nil
) -> UnsafeMutableRawPointer {
  sbRetain(
    SBRSApplicationHandle(
      application: application,
      bundleIdentifier: bundleIdentifier,
      url: url,
      processIdentifier: processIdentifier))
}

private func sbApplicationURL(for handle: SBRSApplicationHandle) -> URL? {
  if let url = handle.url {
    return url
  }

  if let bundleIdentifier = handle.bundleIdentifier {
    return NSWorkspace.shared.urlForApplication(withBundleIdentifier: bundleIdentifier)
  }

  if let processIdentifier = handle.processIdentifier {
    return NSRunningApplication(processIdentifier: processIdentifier)?.bundleURL
  }

  return nil
}

private func sbRunningApplication(for handle: SBRSApplicationHandle) -> NSRunningApplication? {
  if let processIdentifier = handle.processIdentifier {
    return NSRunningApplication(processIdentifier: processIdentifier)
  }

  if let bundleIdentifier = handle.bundleIdentifier {
    return NSRunningApplication.runningApplications(withBundleIdentifier: bundleIdentifier).first
  }

  if let url = handle.url {
    return NSWorkspace.shared.runningApplications.first { $0.bundleURL == url }
  }

  return nil
}

private func sbSharedApplication(
  selectorName: String,
  argument: AnyObject,
  bundleIdentifier: String? = nil,
  url: URL? = nil,
  processIdentifier: pid_t? = nil,
  errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
  let applicationClass: AnyObject = SBApplication.self
  let selector = NSSelectorFromString(selectorName)
  guard applicationClass.responds(to: selector) else {
    sbSetError(errorOut, "SBApplication does not respond to \(selectorName)")
    return nil
  }

  guard let application = applicationClass.perform(selector, with: argument)?.takeUnretainedValue() as? SBApplication else {
    sbSetError(errorOut, "could not create shared SBApplication using \(selectorName)")
    return nil
  }

  return sbRetainedApplicationHandle(
    application: application,
    bundleIdentifier: bundleIdentifier,
    url: url,
    processIdentifier: processIdentifier)
}

private func sbPerform(
  application: SBApplication,
  command: String,
  argument: String?,
  errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Any? {
  let selectorName = argument == nil ? command : "\(command):"
  let selector = NSSelectorFromString(selectorName)

  if application.responds(to: selector) {
    if let argument {
      return application.perform(selector, with: argument as NSString)?.takeUnretainedValue()
    }

    return application.perform(selector)?.takeUnretainedValue()
  }

  if argument == nil {
    return application.value(forKeyPath: command)
  }

  sbSetError(errorOut, "application does not respond to selector \(selectorName)")
  return nil
}

@_cdecl("sb_application_create_with_bundle_identifier")
public func sb_application_create_with_bundle_identifier(
  _ bundleIdentifierPointer: UnsafePointer<CChar>?,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
  guard let bundleIdentifierPointer else {
    sbSetError(errorOut, "missing bundle identifier")
    return nil
  }

  let bundleIdentifier = String(cString: bundleIdentifierPointer)
  guard let application = SBApplication(bundleIdentifier: bundleIdentifier) else {
    sbSetError(errorOut, "could not create SBApplication for \(bundleIdentifier)")
    return nil
  }

  return sbRetainedApplicationHandle(application: application, bundleIdentifier: bundleIdentifier)
}

@_cdecl("sb_application_create_with_url")
public func sb_application_create_with_url(
  _ urlPointer: UnsafePointer<CChar>?,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
  guard let urlPointer else {
    sbSetError(errorOut, "missing application URL")
    return nil
  }

  let rawURL = String(cString: urlPointer)
  guard let url = sbURL(from: rawURL) else {
    sbSetError(errorOut, "could not parse application URL \(rawURL)")
    return nil
  }

  guard let application = SBApplication(url: url) else {
    sbSetError(errorOut, "could not create SBApplication for \(url.absoluteString)")
    return nil
  }

  return sbRetainedApplicationHandle(application: application, url: url)
}

@_cdecl("sb_application_create_with_process_identifier")
public func sb_application_create_with_process_identifier(
  _ processIdentifier: Int32,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
  guard let application = SBApplication(processIdentifier: processIdentifier) else {
    sbSetError(errorOut, "could not create SBApplication for process identifier \(processIdentifier)")
    return nil
  }

  return sbRetainedApplicationHandle(application: application, processIdentifier: processIdentifier)
}

@_cdecl("sb_application_shared_with_bundle_identifier")
public func sb_application_shared_with_bundle_identifier(
  _ bundleIdentifierPointer: UnsafePointer<CChar>?,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
  guard let bundleIdentifierPointer else {
    sbSetError(errorOut, "missing bundle identifier")
    return nil
  }

  let bundleIdentifier = String(cString: bundleIdentifierPointer)
  return sbSharedApplication(
    selectorName: "applicationWithBundleIdentifier:",
    argument: bundleIdentifier as NSString,
    bundleIdentifier: bundleIdentifier,
    errorOut: errorOut)
}

@_cdecl("sb_application_shared_with_url")
public func sb_application_shared_with_url(
  _ urlPointer: UnsafePointer<CChar>?,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
  guard let urlPointer else {
    sbSetError(errorOut, "missing application URL")
    return nil
  }

  let rawURL = String(cString: urlPointer)
  guard let url = sbURL(from: rawURL) else {
    sbSetError(errorOut, "could not parse application URL \(rawURL)")
    return nil
  }

  return sbSharedApplication(
    selectorName: "applicationWithURL:",
    argument: url as NSURL,
    url: url,
    errorOut: errorOut)
}

@_cdecl("sb_application_shared_with_process_identifier")
public func sb_application_shared_with_process_identifier(
  _ processIdentifier: Int32,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
  let applicationClass: AnyObject = SBApplication.self
  let selector = NSSelectorFromString("applicationWithProcessIdentifier:")
  guard applicationClass.responds(to: selector) else {
    sbSetError(errorOut, "SBApplication does not respond to applicationWithProcessIdentifier:")
    return nil
  }

  typealias SharedApplicationWithProcessIdentifier = @convention(c) (AnyObject, Selector, Int32) -> AnyObject?
  let implementation = applicationClass.method(for: selector)
  let function = unsafeBitCast(implementation, to: SharedApplicationWithProcessIdentifier.self)
  guard let application = function(applicationClass, selector, processIdentifier) as? SBApplication else {
    sbSetError(errorOut, "could not create shared SBApplication for process identifier \(processIdentifier)")
    return nil
  }

  return sbRetainedApplicationHandle(application: application, processIdentifier: processIdentifier)
}

@_cdecl("sb_application_copy_object")
public func sb_application_copy_object(_ rawHandle: UnsafeMutableRawPointer?) -> UnsafeMutableRawPointer? {
  guard let rawHandle else {
    return nil
  }

  let handle: SBRSApplicationHandle = sbBorrow(rawHandle)
  return sbRetain(SBRSObjectHandle(object: handle.application))
}

@_cdecl("sb_application_class_for_scripting_class")
public func sb_application_class_for_scripting_class(
  _ rawHandle: UnsafeMutableRawPointer?,
  _ classNamePointer: UnsafePointer<CChar>?,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
  guard let rawHandle else {
    sbSetError(errorOut, "missing SBApplication handle")
    return nil
  }
  guard let classNamePointer else {
    sbSetError(errorOut, "missing scripting class name")
    return nil
  }

  let handle: SBRSApplicationHandle = sbBorrow(rawHandle)
  let className = String(cString: classNamePointer)
  guard let scriptingClass = handle.application.class(forScriptingClass: className) else {
    return nil
  }

  return sbRetain(SBRSScriptingClassHandle(scriptingClass: scriptingClass))
}

@_cdecl("sb_scripting_class_name")
public func sb_scripting_class_name(_ rawHandle: UnsafeMutableRawPointer?) -> UnsafeMutablePointer<CChar>? {
  guard let rawHandle else {
    return nil
  }

  let handle: SBRSScriptingClassHandle = sbBorrow(rawHandle)
  return sbCString(NSStringFromClass(handle.scriptingClass))
}

@_cdecl("sb_scripting_class_release")
public func sb_scripting_class_release(_ rawHandle: UnsafeMutableRawPointer?) {
  guard let rawHandle else {
    return
  }

  sbRelease(rawHandle)
}

@_cdecl("sb_application_is_running")
public func sb_application_is_running(_ rawHandle: UnsafeMutableRawPointer?) -> Bool {
  guard let rawHandle else {
    return false
  }

  let handle: SBRSApplicationHandle = sbBorrow(rawHandle)
  return handle.application.isRunning
}

@_cdecl("sb_application_process_identifier")
public func sb_application_process_identifier(_ rawHandle: UnsafeMutableRawPointer?) -> Int32 {
  guard let rawHandle else {
    return -1
  }

  let handle: SBRSApplicationHandle = sbBorrow(rawHandle)
  return sbRunningApplication(for: handle)?.processIdentifier ?? -1
}

@_cdecl("sb_application_launch")
public func sb_application_launch(
  _ rawHandle: UnsafeMutableRawPointer?,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
  guard let rawHandle else {
    sbSetError(errorOut, "missing SBApplication handle")
    return false
  }

  let handle: SBRSApplicationHandle = sbBorrow(rawHandle)
  guard let url = sbApplicationURL(for: handle) else {
    sbSetError(errorOut, "could not resolve an application URL for launch")
    return false
  }

  let configuration = NSWorkspace.OpenConfiguration()
  let launchFlags = handle.application.launchFlags.rawValue
  configuration.activates = (launchFlags & sbLSLaunchDontSwitch) == 0
  configuration.createsNewApplicationInstance = (launchFlags & sbLSLaunchNewInstance) != 0

  let semaphore = DispatchSemaphore(value: 0)
  var didLaunch = false
  Task {
    do {
      _ = try await NSWorkspace.shared.openApplication(at: url, configuration: configuration)
      didLaunch = true
    } catch {
      sbSetError(errorOut, sbNSErrorMessage(error as NSError))
    }
    semaphore.signal()
  }

  if semaphore.wait(timeout: .now() + .seconds(30)) == .timedOut {
    sbSetError(errorOut, "timed out while launching \(url.absoluteString)")
    return false
  }

  return didLaunch
}

@_cdecl("sb_application_activate")
public func sb_application_activate(
  _ rawHandle: UnsafeMutableRawPointer?,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
  guard let rawHandle else {
    sbSetError(errorOut, "missing SBApplication handle")
    return false
  }

  let handle: SBRSApplicationHandle = sbBorrow(rawHandle)
  handle.application.activate()
  return true
}

@_cdecl("sb_application_quit")
public func sb_application_quit(
  _ rawHandle: UnsafeMutableRawPointer?,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
  guard let rawHandle else {
    sbSetError(errorOut, "missing SBApplication handle")
    return false
  }

  let handle: SBRSApplicationHandle = sbBorrow(rawHandle)
  _ = sbPerform(application: handle.application, command: "quit", argument: nil, errorOut: errorOut)
  return errorOut?.pointee == nil
}

@_cdecl("sb_application_terminate")
public func sb_application_terminate(
  _ rawHandle: UnsafeMutableRawPointer?,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
  guard let rawHandle else {
    sbSetError(errorOut, "missing SBApplication handle")
    return false
  }

  let handle: SBRSApplicationHandle = sbBorrow(rawHandle)
  guard let runningApplication = sbRunningApplication(for: handle) else {
    return true
  }

  if runningApplication.terminate() {
    return true
  }

  sbSetError(errorOut, "terminate() returned false")
  return false
}

@_cdecl("sb_application_get_launch_flags")
public func sb_application_get_launch_flags(_ rawHandle: UnsafeMutableRawPointer?) -> UInt32 {
  guard let rawHandle else {
    return 0
  }

  let handle: SBRSApplicationHandle = sbBorrow(rawHandle)
  return handle.application.launchFlags.rawValue
}

@_cdecl("sb_application_set_launch_flags")
public func sb_application_set_launch_flags(
  _ rawHandle: UnsafeMutableRawPointer?,
  _ launchFlags: UInt32,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
  guard let rawHandle else {
    sbSetError(errorOut, "missing SBApplication handle")
    return false
  }

  let handle: SBRSApplicationHandle = sbBorrow(rawHandle)
  handle.application.launchFlags = LSLaunchFlags(rawValue: launchFlags)
  return true
}

@_cdecl("sb_application_get_send_mode")
public func sb_application_get_send_mode(_ rawHandle: UnsafeMutableRawPointer?) -> Int32 {
  guard let rawHandle else {
    return 0
  }

  let handle: SBRSApplicationHandle = sbBorrow(rawHandle)
  return handle.application.sendMode
}

@_cdecl("sb_application_set_send_mode")
public func sb_application_set_send_mode(
  _ rawHandle: UnsafeMutableRawPointer?,
  _ sendMode: Int32,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
  guard let rawHandle else {
    sbSetError(errorOut, "missing SBApplication handle")
    return false
  }

  let handle: SBRSApplicationHandle = sbBorrow(rawHandle)
  handle.application.sendMode = sendMode
  return true
}

@_cdecl("sb_application_get_timeout")
public func sb_application_get_timeout(_ rawHandle: UnsafeMutableRawPointer?) -> Int64 {
  guard let rawHandle else {
    return 0
  }

  let handle: SBRSApplicationHandle = sbBorrow(rawHandle)
  return Int64(handle.application.timeout)
}

@_cdecl("sb_application_set_timeout")
public func sb_application_set_timeout(
  _ rawHandle: UnsafeMutableRawPointer?,
  _ timeout: Int64,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
  guard let rawHandle else {
    sbSetError(errorOut, "missing SBApplication handle")
    return false
  }

  let handle: SBRSApplicationHandle = sbBorrow(rawHandle)
  handle.application.timeout = Int(timeout)
  return true
}

@_cdecl("sb_application_set_delegate")
public func sb_application_set_delegate(
  _ rawHandle: UnsafeMutableRawPointer?,
  _ delegateHandle: UnsafeMutableRawPointer?,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Bool {
  guard let rawHandle else {
    sbSetError(errorOut, "missing SBApplication handle")
    return false
  }

  let handle: SBRSApplicationHandle = sbBorrow(rawHandle)
  if let delegateHandle {
    let delegate: SBRSApplicationDelegateHandle = sbBorrow(delegateHandle)
    handle.application.delegate = delegate
  } else {
    handle.application.delegate = nil
  }
  return true
}

@_cdecl("sb_application_has_delegate")
public func sb_application_has_delegate(_ rawHandle: UnsafeMutableRawPointer?) -> Bool {
  guard let rawHandle else {
    return false
  }

  let handle: SBRSApplicationHandle = sbBorrow(rawHandle)
  return handle.application.delegate != nil
}

@_cdecl("sb_application_tell")
public func sb_application_tell(
  _ rawHandle: UnsafeMutableRawPointer?,
  _ commandPointer: UnsafePointer<CChar>?,
  _ argumentPointer: UnsafePointer<CChar>?,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutablePointer<CChar>? {
  guard let rawHandle else {
    sbSetError(errorOut, "missing SBApplication handle")
    return nil
  }
  guard let commandPointer else {
    sbSetError(errorOut, "missing tell command")
    return nil
  }

  let handle: SBRSApplicationHandle = sbBorrow(rawHandle)
  let result = sbPerform(
    application: handle.application,
    command: String(cString: commandPointer),
    argument: argumentPointer.map { String(cString: $0) },
    errorOut: errorOut)

  guard let result else {
    return nil
  }

  return sbCString(String(describing: result))
}

@_cdecl("sb_application_send_event")
public func sb_application_send_event(
  _ rawHandle: UnsafeMutableRawPointer?,
  _ eventClass: UInt32,
  _ eventID: UInt32,
  _ parameterCodes: UnsafePointer<UInt32>?,
  _ parameterValues: UnsafePointer<UnsafeMutableRawPointer?>?,
  _ parameterCount: Int64,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
  guard let rawHandle else {
    sbSetError(errorOut, "missing SBApplication handle")
    return nil
  }

  let handle: SBRSApplicationHandle = sbBorrow(rawHandle)
  let result = sbInvokeSendEvent(
    on: handle.application,
    eventClass: eventClass,
    eventID: eventID,
    parameterCodes: parameterCodes,
    parameterValues: parameterValues,
    parameterCount: parameterCount,
    errorOut: errorOut)

  return sbDescriptorHandle(from: result)
}

@_cdecl("sb_application_object_for_key_path")
public func sb_application_object_for_key_path(
  _ rawHandle: UnsafeMutableRawPointer?,
  _ keyPathPointer: UnsafePointer<CChar>?,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
  guard let rawHandle else {
    sbSetError(errorOut, "missing SBApplication handle")
    return nil
  }
  guard let keyPathPointer else {
    sbSetError(errorOut, "missing key path")
    return nil
  }

  let handle: SBRSApplicationHandle = sbBorrow(rawHandle)
  guard let value = handle.application.value(forKeyPath: String(cString: keyPathPointer)) else {
    return nil
  }
  guard let object = value as? SBObject else {
    sbSetError(errorOut, "key path did not resolve to an SBObject")
    return nil
  }

  return sbRetain(SBRSObjectHandle(object: object))
}

@_cdecl("sb_application_element_array_for_key_path")
public func sb_application_element_array_for_key_path(
  _ rawHandle: UnsafeMutableRawPointer?,
  _ keyPathPointer: UnsafePointer<CChar>?,
  _ errorOut: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> UnsafeMutableRawPointer? {
  guard let rawHandle else {
    sbSetError(errorOut, "missing SBApplication handle")
    return nil
  }
  guard let keyPathPointer else {
    sbSetError(errorOut, "missing key path")
    return nil
  }

  let handle: SBRSApplicationHandle = sbBorrow(rawHandle)
  guard let value = handle.application.value(forKeyPath: String(cString: keyPathPointer)) else {
    return nil
  }
  guard let elementArray = value as? SBElementArray else {
    sbSetError(errorOut, "key path did not resolve to an SBElementArray")
    return nil
  }

  return sbRetain(SBRSElementArrayHandle(array: elementArray))
}

@_cdecl("sb_application_release")
public func sb_application_release(_ rawHandle: UnsafeMutableRawPointer?) {
  guard let rawHandle else {
    return
  }

  sbRelease(rawHandle)
}
